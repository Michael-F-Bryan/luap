use std::sync::Arc;

use miette::{NamedSource, SourceSpan};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::accumulator]
pub struct Diagnostic(pub DiagnosticKind);

impl Diagnostic {
    pub fn report(&self) -> miette::Report {
        match &self.0 {
            DiagnosticKind::SyntaxError(err) => miette::Report::new(err.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    SyntaxError(SyntaxError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, miette::Diagnostic, thiserror::Error)]
#[error("{message}")]
pub struct SyntaxError {
    pub message: String,
    pub label: String,
    #[source_code]
    pub src: NamedSource<Arc<str>>,
    #[label("{label}")]
    pub span: SourceSpan,
}
