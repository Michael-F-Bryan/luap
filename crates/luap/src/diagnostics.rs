use std::sync::Arc;

use miette::{NamedSource, SourceSpan};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::accumulator]
pub struct Diagnostic(DiagnosticKind);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    SyntaxError(SyntaxError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, miette::Diagnostic, thiserror::Error)]
#[error("syntax error")]
pub struct SyntaxError {
    #[source_code]
    src: NamedSource<Arc<str>>,
    #[label("This bit here")]
    bad_bit: SourceSpan,
}
