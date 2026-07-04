//! High-level intermediate representation.

use crate::pointer::ChunkPtr;

#[salsa::tracked]
#[derive(Debug)]
pub struct File<'db> {
    statements: Vec<Statement<'db>>,
    ptr: ChunkPtr,
}

#[salsa::tracked]
#[derive(Debug)]
pub struct Statement<'db> {}
