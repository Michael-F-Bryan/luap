use type_sitter::Node as _;

use super::{hir, lower::LowerCtx};
use crate::SourceFile;

/// Lower from a concrete syntax tree to a high-level intermediate
/// representation.
#[salsa::tracked]
pub fn lower(db: &dyn crate::Db, source_file: SourceFile) -> hir::File<'_> {
    let tree = crate::parse(db, source_file);
    let chunk = syntax::Chunk::try_from_raw(tree.root_node()).expect("parse root is chunk");

    let ctx = LowerCtx { db, source_file };
    ctx.lower_chunk(chunk)
}

use crate::syntax;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use camino::Utf8Path;
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::{diagnostics::DiagnosticKind, Compiler, SourceFile};

    fn source_file(db: &Compiler, path: &str, source: &str) -> SourceFile {
        SourceFile::new(db, Arc::from(Utf8Path::new(path)), source.into())
    }

    fn lower_file<'a>(db: &'a Compiler, path: &str, source: &str) -> hir::File<'a> {
        let source_file = source_file(db, path, source);
        lower(db, source_file)
    }

    fn lower_diagnostics(db: &Compiler, path: &str, source: &str) -> Vec<String> {
        let source_file = source_file(db, path, source);
        let _ = lower(db, source_file);
        lower::accumulated::<crate::Diagnostic>(db, source_file)
            .into_iter()
            .map(|diagnostic| match &diagnostic.0 {
                DiagnosticKind::SyntaxError(err) => err.message.clone(),
                DiagnosticKind::Unsupported(err) => err.message.clone(),
                DiagnosticKind::UnresolvedName(err) => err.name.clone(),
            })
            .collect()
    }

    #[test]
    fn lower_hello_world() {
        let db = Compiler::default();
        let file = lower_file(&db, "hello_world.lua", r#"print("Hello, world!")"#);
        assert_debug_snapshot!(file.statements(&db));
        assert!(lower_diagnostics(&db, "hello_world.lua", r#"print("Hello, world!")"#).is_empty());
    }

    #[test]
    fn lower_emits_unsupported_for_unknown_statement() {
        let db = Compiler::default();
        let diagnostics = lower_diagnostics(&db, "local.lua", "local x = 1");
        assert_debug_snapshot!(diagnostics);
    }
}
