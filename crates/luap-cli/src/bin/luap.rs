use clap::Parser;
use luap_cli::Cmd;

fn main() {
    let cmd = Cmd::parse();
    match cmd {
        Cmd::Build(build_cmd) => build_cmd.run(),
        Cmd::Run(run_cmd) => run_cmd.run(),
        Cmd::Lsp(lsp_cmd) => lsp_cmd.run(),
    }
}
