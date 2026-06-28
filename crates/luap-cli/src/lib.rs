mod build;
mod lsp;
mod run;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub enum Cmd {
    Build(build::BuildCmd),
    Run(run::RunCmd),
    Lsp(lsp::LspCmd),
}
