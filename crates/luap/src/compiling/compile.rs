use crate::{compiling::bytecode::Bytecode, lowering::query::lower, SourceFile};

use super::builder::CodeBuilder;

/// Compile a Lua source file to bytecode.
#[salsa::tracked]
pub fn compile(db: &dyn crate::Db, source_file: SourceFile) -> Bytecode {
    let file = lower(db, source_file);
    let mut builder = CodeBuilder::new();
    file.compile(db, &mut builder);
    builder.finish()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use camino::Utf8Path;
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::{diagnostics::DiagnosticKind, Compiler};

    fn source_file(db: &Compiler, path: &str, source: &str) -> SourceFile {
        SourceFile::new(db, Arc::from(Utf8Path::new(path)), source.into())
    }

    fn compile_file<'a>(db: &'a Compiler, path: &str, source: &str) -> Bytecode {
        let source_file = source_file(db, path, source);
        compile(db, source_file)
    }

    fn compile_diagnostics(db: &Compiler, path: &str, source: &str) -> Vec<String> {
        let source_file = source_file(db, path, source);
        let _ = compile(db, source_file);
        compile::accumulated::<crate::Diagnostic>(db, source_file)
            .into_iter()
            .map(|diagnostic| match &diagnostic.0 {
                DiagnosticKind::SyntaxError(err) => err.message.clone(),
                DiagnosticKind::Unsupported(err) => err.message.clone(),
                DiagnosticKind::UnresolvedName(err) => err.name.clone(),
            })
            .collect()
    }

    #[test]
    fn compile_hello_world() {
        let db = Compiler::default();
        let bytecode = compile_file(&db, "hello_world.lua", r#"print("Hello, world!")"#);

        assert_debug_snapshot!((
            &bytecode.instructions,
            &bytecode.constants,
            bytecode.num_regs,
        ));
        assert!(
            compile_diagnostics(&db, "hello_world.lua", r#"print("Hello, world!")"#).is_empty()
        );
    }

    #[test]
    fn compile_emits_unresolved_name_for_unknown_callee() {
        let db = Compiler::default();
        let diagnostics = compile_diagnostics(&db, "unknown.lua", r#"unknown("Hello, world!")"#);
        assert_debug_snapshot!(diagnostics);
    }
}
