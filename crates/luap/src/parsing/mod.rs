mod syntax_error;
mod tree;

use salsa::Accumulator;
use type_sitter::Node;

pub use self::tree::Tree;
use crate::{
    diagnostics::{Diagnostic, DiagnosticKind},
    types::SourceFile,
};

/// Parse a source file into a concrete syntax tree.
#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source_file: SourceFile) -> Tree {
    let lang = tree_sitter::Language::from(tree_sitter_lua::LANGUAGE);
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&lang).unwrap();
    let source = source_file.contents(db);
    let tree = parser.parse(source, None).unwrap();

    collect_diagnostics(db, &lang, source_file, tree.root_node());

    tree.into()
}

fn collect_diagnostics(
    db: &dyn crate::Db,
    lang: &tree_sitter::Language,
    source_file: SourceFile,
    node: tree_sitter::Node<'_>,
) {
    if node.is_error() || node.is_missing() {
        let path = source_file.path(db);
        let source = source_file.contents(db);
        Diagnostic(DiagnosticKind::SyntaxError(syntax_error::describe(
            lang,
            path.as_str(),
            source,
            node,
        )))
        .accumulate(db);
        return;
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_diagnostics(db, lang, source_file, cursor.node());
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[salsa::tracked]
pub struct ParsedOutput<'db> {
    pub source_file: SourceFile,
    #[returns(ref)]
    pub tree: Tree,
}

#[salsa::tracked]
impl<'db> ParsedOutput<'db> {
    pub fn root(&self, db: &'db dyn crate::Db) -> crate::syntax::Chunk<'db> {
        let root = self.tree(db).root_node();
        crate::syntax::Chunk::try_from_raw(root).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use camino::Utf8Path;

    use super::*;
    use crate::{diagnostics::SyntaxError, Compiler, SourceFile};

    fn source_file(db: &Compiler, path: &str, source: &str) -> SourceFile {
        SourceFile::new(db, Arc::from(Utf8Path::new(path)), source.into())
    }

    fn parse_diagnostics(db: &Compiler, path: &str, source: &str) -> Vec<Diagnostic> {
        let source_file = source_file(db, path, source);
        let _tree = parse(db, source_file);
        parse::accumulated::<Diagnostic>(db, source_file)
            .into_iter()
            .cloned()
            .collect()
    }

    fn sole_syntax_error(db: &Compiler, path: &str, source: &str) -> SyntaxError {
        let diagnostics = parse_diagnostics(db, path, source);
        assert_eq!(diagnostics.len(), 1);
        match &diagnostics[0].0 {
            DiagnosticKind::SyntaxError(err) => err.clone(),
            DiagnosticKind::Unsupported(_) | DiagnosticKind::UnresolvedName(_) => {
                panic!("expected syntax error")
            }
        }
    }

    #[test]
    fn parse_emits_diagnostic_for_error_node() {
        let db = Compiler::default();
        let err = sole_syntax_error(&db, "test.lua", "@@@");

        assert_eq!(err.message, "unexpected '@@@', expected a statement");
        assert_eq!(err.span.offset(), 0);
        assert_eq!(err.span.len(), 3);
    }

    #[test]
    fn parse_emits_diagnostic_for_missing_node() {
        let db = Compiler::default();
        let err = sole_syntax_error(&db, "test.lua", "print(1");

        assert_eq!(err.message, "expected ')'");
        assert_eq!(err.span.offset(), 7);
        assert_eq!(err.span.len(), 0);
    }

    #[test]
    fn parse_emits_diagnostic_for_unclosed_call() {
        let db = Compiler::default();
        let err = sole_syntax_error(&db, "test.lua", "print(");

        assert_eq!(err.message, "expected expression or ')' before end of file");
        assert_eq!(err.span.offset(), 6);
        assert_eq!(err.span.len(), 0);
    }
}
