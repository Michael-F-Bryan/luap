mod compiler;
mod compiling;
pub mod diagnostics;
mod lowering;
mod parsing;
pub mod pointer;
pub mod syntax;
mod types;
pub mod vm;

pub use crate::{
    compiler::Compiler,
    compiling::{bytecode, compile},
    diagnostics::{Diagnostic, DiagnosticKind},
    lowering::{hir, query::lower},
    parsing::{parse, ParsedOutput, Tree},
    types::SourceFile,
};

#[salsa::db]
pub trait Db: salsa::Database + 'static {}
