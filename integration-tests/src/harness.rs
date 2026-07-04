use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;

/// Options for the luap integration test harness.
///
/// Arguments after the harness flags are forwarded to libtest-mimic (filters,
/// `--list`, `--exact`, etc.). Put harness flags before libtest flags:
/// `luap-test --bless ui/parse`, not `luap-test ui/parse --bless`.
#[derive(Parser, Debug)]
#[command(
    name = "luap-test",
    about = "Run luap compiletest-style integration tests",
    after_help = "EXAMPLES:
    cargo test -p integration-tests --test suites -- --bless
    cargo nextest run -p integration-tests
    cargo run -p integration-tests --bin luap-test -- --bless ui/parse"
)]
pub struct Cli {
    /// Rewrite expectation files (.stderr, .run.stdout, …) with actual output.
    #[arg(long)]
    pub bless: bool,

    /// Path to the `luap` binary.
    #[arg(long, env = "LUAP_BIN")]
    pub luap: Option<PathBuf>,

    /// Root directory containing `ui/`, `run/`, and `hir/` suites.
    #[arg(long, default_value = "suites")]
    pub suites: PathBuf,

    /// Arguments forwarded to libtest-mimic.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub libtest_args: Vec<String>,
}

pub fn parse_args() -> (Config, Vec<String>) {
    let cli = Cli::parse();
    let config = Config {
        bless: cli.bless,
        luap: cli.luap.unwrap_or_else(Config::luap_binary),
        suites: resolve_suites_dir(cli.suites),
    };
    let libtest_args = std::iter::once("luap-test".to_string())
        .chain(cli.libtest_args)
        .collect();
    (config, libtest_args)
}

fn resolve_suites_dir(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
    }
}
