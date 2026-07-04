use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use luap::{vm::VirtualMachine, Compiler, SourceFile};

#[derive(Parser, Debug)]
pub struct RunCmd {
    #[arg(help = "The path to the Lua source file to run")]
    pub(crate) path: PathBuf,
}

impl RunCmd {
    pub fn run(self) -> miette::Result<ExitCode> {
        let db = Compiler::default();
        let source_file = SourceFile::from_path(&db, &self.path)
            .map_err(|err| miette::miette!("failed to read {}: {err}", self.path.display()))?;

        let bytecode = luap::compile::compile(&db, source_file);

        let diagnostics: Vec<_> =
            luap::compile::compile::accumulated::<luap::Diagnostic>(&db, source_file)
                .into_iter()
                .cloned()
                .collect();
        if !diagnostics.is_empty() {
            for diagnostic in &diagnostics {
                crate::diagnostics::print_report(&diagnostic.report());
            }
            return Ok(ExitCode::from(1));
        }

        let mut vm = VirtualMachine::default();
        match vm.run_program(&bytecode) {
            Ok(exit_code) => Ok(ExitCode::from(exit_code)),
            Err(err) => {
                eprintln!("runtime error: {err}");
                Ok(ExitCode::from(1))
            }
        }
    }
}
