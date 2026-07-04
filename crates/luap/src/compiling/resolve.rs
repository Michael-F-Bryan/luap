use salsa::Accumulator;

use super::builtins::BuiltinId;
use crate::{
    diagnostics::{Diagnostic, DiagnosticKind, UnresolvedName},
    pointer::IdentifierPtr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolvedName {
    Builtin(BuiltinId),
}

/// Resolve a name reference when compilation or analysis needs a concrete target.
#[salsa::tracked]
pub fn resolve_name(
    db: &dyn crate::Db,
    ptr: IdentifierPtr,
    name: String,
) -> Option<ResolvedName> {
    let Some(id) = BuiltinId::from_name(&name) else {
        let pointer = ptr.untyped();
        let source_file = pointer.source_file;
        let path = source_file.path(db);
        let source = source_file.contents(db);
        Diagnostic(DiagnosticKind::UnresolvedName(UnresolvedName::at(
            path.as_str(),
            source,
            &name,
            pointer.span,
        )))
        .accumulate(db);
        return None;
    };

    Some(ResolvedName::Builtin(id))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use camino::Utf8Path;

    use super::*;
    use crate::{lowering::query::lower, Compiler, SourceFile};

    fn source_file(db: &Compiler, path: &str, source: &str) -> SourceFile {
        SourceFile::new(db, Arc::from(Utf8Path::new(path)), source.into())
    }

    fn name_in_hello_world(db: &Compiler) -> (IdentifierPtr, String) {
        let source_file = source_file(db, "hello_world.lua", r#"print("Hello, world!")"#);
        let file = lower(db, source_file);
        let stmt = &file.statements(db)[0];
        let crate::lowering::hir::Stmt::Call(call) = stmt;
        let crate::lowering::hir::Expr::Name { name, ptr } = &call.callee else {
            panic!("expected name callee");
        };
        (*ptr, name.clone())
    }

    #[test]
    fn resolve_print_to_builtin() {
        let db = Compiler::default();
        let (ptr, name) = name_in_hello_world(&db);
        assert_eq!(
            resolve_name(&db, ptr, name),
            Some(ResolvedName::Builtin(BuiltinId::Print))
        );
        assert!(resolve_name::accumulated::<crate::Diagnostic>(&db, ptr, "print".into()).is_empty());
    }

    #[test]
    fn resolve_unknown_name_emits_diagnostic() {
        let db = Compiler::default();
        let (ptr, _) = name_in_hello_world(&db);
        assert_eq!(resolve_name(&db, ptr, "unknown".into()), None);
        let diagnostics = resolve_name::accumulated::<crate::Diagnostic>(&db, ptr, "unknown".into());
        assert_eq!(diagnostics.len(), 1);
        assert!(matches!(
            &diagnostics[0].0,
            DiagnosticKind::UnresolvedName(err) if err.name == "unknown"
        ));
    }
}
