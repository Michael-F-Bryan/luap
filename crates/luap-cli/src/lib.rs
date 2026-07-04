mod build;
mod lsp;
mod parse;
mod run;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub enum Cmd {
    Build(build::BuildCmd),
    Parse(parse::ParseCmd),
    Run(run::RunCmd),
    Lsp(lsp::LspCmd),
}
