use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct RunCmd {
    #[arg(help = "The path to the Lua source file to run")]
    pub(crate) path: PathBuf,
}

impl RunCmd {
    pub fn run(self) {
        todo!("Run the Lua source file")
    }
}
