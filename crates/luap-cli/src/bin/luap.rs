use std::process::ExitCode;

use clap::Parser;
use luap_cli::Cmd;

fn main() -> miette::Result<ExitCode> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().build())
    }))?;

    Ok(match Cmd::parse() {
        Cmd::Build(build_cmd) => {
            build_cmd.run();
            ExitCode::SUCCESS
        }
        Cmd::Check(check_cmd) => check_cmd.run()?,
        Cmd::Parse(parse_cmd) => parse_cmd.run()?,
        Cmd::Run(run_cmd) => run_cmd.run()?,
        Cmd::Lsp(lsp_cmd) => {
            lsp_cmd.run();
            ExitCode::SUCCESS
        }
    })
}
