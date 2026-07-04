use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use luap::{Compiler, SourceFile};

#[derive(Parser, Debug)]
pub struct ParseCmd {
    #[arg(help = "The path to the Lua source file to parse")]
    pub(crate) path: PathBuf,
}

impl ParseCmd {
    pub fn run(self) -> miette::Result<ExitCode> {
        let db = Compiler::default();
        let source_file = SourceFile::from_path(&db, &self.path)
            .map_err(|err| miette::miette!("failed to read {}: {err}", self.path.display()))?;

        let tree = luap::parse(&db, source_file);
        println!("{tree}");

        let diagnostics: Vec<_> = luap::parse::accumulated::<luap::Diagnostic>(&db, source_file)
            .into_iter()
            .cloned()
            .collect();
        for diagnostic in &diagnostics {
            eprintln!("{:?}", diagnostic.report());
        }

        if diagnostics.is_empty() {
            Ok(ExitCode::SUCCESS)
        } else {
            Ok(ExitCode::from(1))
        }
    }
}
