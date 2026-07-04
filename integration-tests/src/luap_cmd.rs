use std::path::Path;
use std::process::{Command, Output};

/// Build a `luap` subprocess with deterministic, colour-free output.
pub fn command(luap: &Path) -> Command {
    let mut command = Command::new(luap);
    command
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .env("CLICOLOR_FORCE", "0")
        .env("TERM", "dumb");
    command
}

pub fn run(luap: &Path, subcommand: &str, path: &Path) -> std::io::Result<Output> {
    command(luap).arg(subcommand).arg(path).output()
}
