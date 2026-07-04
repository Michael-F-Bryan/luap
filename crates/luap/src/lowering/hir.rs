//! High-level intermediate representation.

use crate::pointer::{ChunkPtr, IdentifierPtr, StatementPtr, StringPtr};

#[salsa::tracked]
#[derive(Debug)]
pub struct File<'db> {
    #[returns(ref)]
    pub statements: Vec<Stmt>,
    pub ptr: ChunkPtr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Call(CallStmt),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallStmt {
    pub callee: Expr,
    pub args: Vec<Expr>,
    pub ptr: StatementPtr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    StringLiteral { value: String, ptr: StringPtr },
    Name { name: String, ptr: IdentifierPtr },
}
