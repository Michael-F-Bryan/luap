//! Compiletest-style integration test runner for luap.

mod config;
mod directives;
mod discover;
mod harness;
mod luap_cmd;
mod normalize;
mod runner;

use std::process::ExitCode;

pub use config::Config;
pub use discover::discover;
use libtest_mimic::Arguments;
pub use runner::run_test;

/// Entry point for the `luap-test` binary and `suites` integration test target.
pub fn run() -> ExitCode {
    let (config, libtest_args) = harness::parse_args();
    let args = Arguments::from_iter(libtest_args);
    let tests = discover(&config);
    libtest_mimic::run(&args, tests).exit_code()
}
