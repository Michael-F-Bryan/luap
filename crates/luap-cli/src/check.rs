use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use luap::{Compiler, SourceFile};

#[derive(Parser, Debug)]
pub struct CheckCmd {
    #[arg(help = "The path to the Lua source file to check")]
    pub(crate) path: PathBuf,
}

impl CheckCmd {
    pub fn run(self) -> miette::Result<ExitCode> {
        let db = Compiler::default();
        let source_file = SourceFile::from_path(&db, &self.path)
            .map_err(|err| miette::miette!("failed to read {}: {err}", self.path.display()))?;

        let _tree = luap::lower(&db, source_file);

        let diagnostics: Vec<_> = luap::lower::accumulated::<luap::Diagnostic>(&db, source_file)
            .into_iter()
            .cloned()
            .collect();
        for diagnostic in &diagnostics {
            crate::diagnostics::print_report(&diagnostic.report());
        }

        if diagnostics.is_empty() {
            Ok(ExitCode::SUCCESS)
        } else {
            Ok(ExitCode::from(1))
        }
    }
}
