use type_sitter::Node as _;

use crate::syntax::{self, anon_unions};

pub(crate) fn decode(string: &syntax::String<'_>, source: &str) -> String {
    let raw = string
        .raw()
        .utf8_text(source.as_bytes())
        .expect("valid utf8");

    match string.start().expect("string start delimiter") {
        anon_unions::DoubleQuote_Quote_LBracketLBracket::DoubleQuote(_)
        | anon_unions::DoubleQuote_Quote_LBracketLBracket::Quote(_) => {
            decode_quoted(&raw[1..raw.len() - 1])
        }
        anon_unions::DoubleQuote_Quote_LBracketLBracket::LBracketLBracket(_) => raw
            .strip_prefix("[[")
            .and_then(|inner| inner.strip_suffix("]]"))
            .expect("long string delimiters")
            .to_string(),
    }
}

fn decode_quoted(inner: &str) -> String {
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let Some(escaped) = chars.next() else {
            out.push('\\');
            break;
        };
        match escaped {
            'a' => out.push('\x07'),
            'b' => out.push('\x08'),
            'f' => out.push('\x0c'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'v' => out.push('\x0b'),
            '\\' | '"' | '\'' => out.push(escaped),
            'z' => {
                while matches!(chars.clone().next(), Some(' ' | '\t')) {
                    chars.next();
                }
            }
            'x' => {
                let hi = chars.next().and_then(|c| c.to_digit(16));
                let lo = chars.next().and_then(|c| c.to_digit(16));
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push(char::from_u32(hi * 16 + lo).unwrap_or('\u{FFFD}'));
                }
            }
            '0'..='9' => {
                let mut value = escaped.to_digit(10).unwrap();
                for _ in 0..2 {
                    match chars.clone().next().and_then(|c| c.to_digit(10)) {
                        Some(digit) if value < 256 => {
                            value = value * 10 + digit;
                            chars.next();
                        }
                        _ => break,
                    }
                }
                out.push(char::from_u32(value).unwrap_or('\u{FFFD}'));
            }
            '\r' => {
                if matches!(chars.clone().next(), Some('\n')) {
                    chars.next();
                }
            }
            '\n' => {}
            other => {
                out.push('\\');
                out.push(other);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use type_sitter::Node;

    use super::*;
    use crate::syntax::String;

    fn with_string_literal(source: &str, f: impl FnOnce(String<'_>)) {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter::Language::from(tree_sitter_lua::LANGUAGE))
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut cursor = tree.root_node().walk();
        loop {
            let node = cursor.node();
            if node.kind() == "string" {
                f(String::try_from_raw(node).unwrap());
                return;
            }
            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    panic!("no string literal in source");
                }
            }
        }
    }

    #[test]
    fn decode_plain_double_quoted_string() {
        let source = r#""Hello, world!""#;
        with_string_literal(source, |string| {
            assert_eq!(decode(&string, source), "Hello, world!");
        });
    }

    #[test]
    fn decode_escape_sequences() {
        let source = r#""a\nb\tc\\d""#;
        with_string_literal(source, |string| {
            assert_eq!(decode(&string, source), "a\nb\tc\\d");
        });
    }
}
