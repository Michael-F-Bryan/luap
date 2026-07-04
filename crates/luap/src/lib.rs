mod compiler;
pub mod diagnostics;
mod lowering;
mod parsing;
pub mod pointer;
pub mod syntax;
mod types;

pub use crate::{
    compiler::Compiler,
    diagnostics::{Diagnostic, DiagnosticKind},
    lowering::{hir, query::lower},
    parsing::{parse, ParsedOutput, Tree},
    types::SourceFile,
};

#[salsa::db]
pub trait Db: salsa::Database + 'static {}
