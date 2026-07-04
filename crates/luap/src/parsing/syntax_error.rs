use std::ops::Range;

use crate::diagnostics::SyntaxError;
use miette::{NamedSource, SourceSpan};

pub(crate) fn describe(
    lang: &tree_sitter::Language,
    path: &str,
    source: &str,
    node: tree_sitter::Node<'_>,
) -> SyntaxError {
    let (message, label, span) = if node.is_missing() {
        missing(lang, node)
    } else {
        error(lang, source, node)
    };

    SyntaxError {
        message,
        label,
        src: NamedSource::new(path, std::sync::Arc::from(source)),
        span: SourceSpan::from(span),
    }
}

fn missing(
    lang: &tree_sitter::Language,
    node: tree_sitter::Node<'_>,
) -> (String, String, Range<usize>) {
    let expected = format_expected_symbol(lang, node.kind());
    let message = format!("expected {expected}");
    let label = format!("expected {expected} here");
    (message, label, node.byte_range())
}

fn error(
    lang: &tree_sitter::Language,
    source: &str,
    node: tree_sitter::Node<'_>,
) -> (String, String, Range<usize>) {
    let Some(last) = last_child(node) else {
        return (
            "invalid syntax".into(),
            "invalid syntax here".into(),
            node.byte_range(),
        );
    };

    let at_eof = at_logical_eof(source, last);
    let token_text = &source[last.byte_range()];
    let expected = expected_tokens(lang, last);

    if at_eof {
        if last.kind() == "(" {
            return (
                "expected expression or ')' before end of file".into(),
                "unexpected end of file".into(),
                last.end_byte()..last.end_byte(),
            );
        }

        if node.is_error() && at_logical_eof(source, last) && !last.byte_range().is_empty() {
            let message = format!(
                "unexpected {}, expected {}",
                format_found_token(token_text),
                summarize_expected(&expected, last.kind()),
            );
            return (message, "unexpected token".into(), last.byte_range());
        }

        let message = format!(
            "expected {} before end of file",
            summarize_expected(&expected, last.kind())
        );
        return (
            message,
            "unexpected end of file".into(),
            last.end_byte()..last.end_byte(),
        );
    }

    let message = format!(
        "unexpected {}, expected {}",
        format_found_token(token_text),
        summarize_expected(&expected, last.kind()),
    );
    (message, "unexpected token".into(), last.byte_range())
}

fn expected_tokens(lang: &tree_sitter::Language, node: tree_sitter::Node<'_>) -> Vec<&'static str> {
    let state = lang.next_state(node.parse_state(), node.grammar_id());
    useful_lookahead(lang, state)
}

fn useful_lookahead(lang: &tree_sitter::Language, state: u16) -> Vec<&'static str> {
    lang.lookahead_iterator(state)
        .map(|it| {
            it.filter_map(|symbol| {
                let name = lang.node_kind_for_id(symbol)?;
                if lang.node_kind_is_supertype(symbol) {
                    return None;
                }
                if name.contains('_')
                    && lang.id_for_node_kind(name, true) != 0
                    && lang.node_kind_is_named(symbol)
                {
                    return None;
                }
                if matches!(
                    name,
                    "chunk"
                        | "comment"
                        | "comment_content"
                        | "string_content"
                        | "escape_sequence"
                        | "hash_bang_line"
                ) {
                    return None;
                }
                Some(name)
            })
            .collect()
        })
        .unwrap_or_default()
}

fn summarize_expected(tokens: &[&str], last_kind: &str) -> String {
    if tokens.is_empty() {
        return "something else".into();
    }
    if tokens.len() <= 6 {
        return format_expected_list(tokens);
    }
    if last_kind == "(" {
        return "expression or ')'".into();
    }
    "a statement".into()
}

fn format_expected_list(tokens: &[&str]) -> String {
    match tokens.len() {
        0 => "something else".into(),
        1 => format_expected_symbol_name(tokens[0]),
        2 => format!(
            "{} or {}",
            format_expected_symbol_name(tokens[0]),
            format_expected_symbol_name(tokens[1]),
        ),
        _ => {
            let head = tokens[..tokens.len() - 1]
                .iter()
                .map(|token| format_expected_symbol_name(token))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{head}, or {}",
                format_expected_symbol_name(tokens[tokens.len() - 1])
            )
        }
    }
}

fn format_expected_symbol(_lang: &tree_sitter::Language, kind: &str) -> String {
    format_expected_symbol_name(kind)
}

fn format_expected_symbol_name(kind: &str) -> String {
    if kind.chars().all(|ch| !ch.is_alphanumeric() && ch != '_') {
        format!("'{kind}'")
    } else {
        format!("`{kind}`")
    }
}

fn format_found_token(token: &str) -> String {
    if token.chars().all(|ch| ch.is_whitespace()) {
        "whitespace".into()
    } else {
        format!("'{token}'")
    }
}

fn last_child(node: tree_sitter::Node<'_>) -> Option<tree_sitter::Node<'_>> {
    let mut cursor = node.walk();
    if !cursor.goto_first_child() {
        return None;
    }

    let mut last = None;
    loop {
        let child = cursor.node();
        if !child.is_extra() {
            last = Some(child);
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }

    last
}

fn at_logical_eof(source: &str, last: tree_sitter::Node<'_>) -> bool {
    source[last.end_byte()..]
        .chars()
        .all(|ch| ch.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lang() -> tree_sitter::Language {
        tree_sitter::Language::from(tree_sitter_lua::LANGUAGE)
    }

    fn parse_tree(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        let lang = lang();
        parser.set_language(&lang).unwrap();
        parser.parse(source, None).unwrap()
    }

    fn find_error_or_missing<'tree>(
        node: tree_sitter::Node<'tree>,
    ) -> Option<tree_sitter::Node<'tree>> {
        if node.is_error() || node.is_missing() {
            return Some(node);
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                if let Some(node) = find_error_or_missing(cursor.node()) {
                    return Some(node);
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        None
    }

    #[test]
    fn missing_paren_is_described_at_insertion_point() {
        let source = "print(1";
        let tree = parse_tree(source);
        let node = find_error_or_missing(tree.root_node()).unwrap();
        let err = describe(&lang(), "test.lua", source, node);

        assert_eq!(err.message, "expected ')'");
        assert_eq!(err.label, "expected ')' here");
        assert_eq!(err.span.offset(), 7);
        assert_eq!(err.span.len(), 0);
    }

    #[test]
    fn unclosed_call_gets_eof_message() {
        let source = "print(";
        let tree = parse_tree(source);
        let node = find_error_or_missing(tree.root_node()).unwrap();
        let err = describe(&lang(), "test.lua", source, node);

        assert_eq!(err.message, "expected expression or ')' before end of file");
        assert_eq!(err.label, "unexpected end of file");
        assert_eq!(err.span.offset(), 6);
        assert_eq!(err.span.len(), 0);
    }

    #[test]
    fn garbage_token_is_called_out() {
        let source = "@@@";
        let tree = parse_tree(source);
        let node = find_error_or_missing(tree.root_node()).unwrap();
        let err = describe(&lang(), "test.lua", source, node);

        assert_eq!(err.message, "unexpected '@@@', expected a statement");
        assert_eq!(err.label, "unexpected token");
        assert_eq!(err.span.offset(), 0);
        assert_eq!(err.span.len(), 3);
    }
}
