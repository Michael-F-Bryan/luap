use crate::SourceFile;

use super::hir;

/// Lower from a concrete syntax tree to a high-level intermediate
/// representation.
#[salsa::tracked]
pub fn lower(db: &dyn crate::Db, source_file: SourceFile) -> hir::File<'_> {
    let _tree = crate::parse(db, source_file);
    todo!();
}
