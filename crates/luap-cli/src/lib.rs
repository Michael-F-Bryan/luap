mod build;
mod check;
mod diagnostics;
mod lsp;
mod parse;
mod run;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub enum Cmd {
    /// Compile a Lua source file into a bytecode chunk.
    Build(build::BuildCmd),
    /// Check a Lua source file for syntactic and semantic errors.
    Check(check::CheckCmd),
    /// Parse a Lua source file into a syntax tree.
    Parse(parse::ParseCmd),
    /// Run a Lua bytecode chunk or script.
    Run(run::RunCmd),
    /// Start the Language Server Protocol server.
    Lsp(lsp::LspCmd),
}
