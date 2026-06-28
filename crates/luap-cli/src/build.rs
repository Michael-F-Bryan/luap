use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct BuildCmd {
    #[arg(help = "The path to the Lua source file to build")]
    pub(crate) path: PathBuf,
    #[arg(help = "Where to save the compiled executable")]
    pub(crate) output: Option<PathBuf>,
}

impl BuildCmd {
    pub fn run(self) {
        todo!("Compile to a binary")
    }
}
