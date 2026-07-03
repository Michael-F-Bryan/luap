mod compiler;
pub mod diagnostics;
mod lowering;
mod parsing;
mod types;

pub use crate::{
    compiler::Compiler,
    lowering::{hir, query::lower},
    parsing::{parse, ParsedOutput, Tree},
    types::SourceFile,
};

#[salsa::db]
pub trait Db: salsa::Database + 'static {}
