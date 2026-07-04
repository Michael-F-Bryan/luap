use std::fs;
use std::path::Path;

use libtest_mimic::Failed;

use crate::config::Config;
use crate::directives::{Expectation, Suite, TestCase};
use crate::luap_cmd;
use crate::normalize::normalize_output;

pub fn run_test(config: &Config, case: &TestCase) -> Result<(), Failed> {
    match case.suite {
        Suite::Ui => run_ui(config, case),
        Suite::Run => run_run(config, case),
        Suite::Hir => Err(Failed::from("HIR suite is not implemented yet")),
    }
}

fn run_ui(config: &Config, case: &TestCase) -> Result<(), Failed> {
    let output = luap_cmd::run(&config.luap, "check", case.path.as_path()).map_err(|err| {
        Failed::from(format!(
            "failed to spawn {}: {err}",
            config.luap.display()
        ))
    })?;

    let stderr = normalize_output(
        &decode(&output.stderr),
        &case.path,
        &case.directives.normalizations,
    );

    match case.directives.expectation {
        Expectation::CheckPass => {
            if output.status.code() != Some(0) {
                return Err(Failed::from(format!(
                    "expected success, got exit {:?}\nstderr:\n{stderr}",
                    output.status.code()
                )));
            }
            if !stderr.is_empty() {
                return Err(Failed::from(format!(
                    "expected empty stderr, got:\n{stderr}"
                )));
            }
            Ok(())
        }
        Expectation::CheckFail => {
            compare_sidecar(config, case, "stderr", &stderr)?;

            if output.status.code() == Some(0) {
                return Err(Failed::from("expected failure, but luap check succeeded"));
            }
            Ok(())
        }
        Expectation::Run => unreachable!("validated at discovery"),
    }
}

fn run_run(config: &Config, case: &TestCase) -> Result<(), Failed> {
    let output = luap_cmd::run(&config.luap, "run", case.path.as_path()).map_err(|err| {
        Failed::from(format!(
            "failed to spawn {}: {err}",
            config.luap.display()
        ))
    })?;

    let stdout = normalize_output(
        &decode(&output.stdout),
        &case.path,
        &case.directives.normalizations,
    );
    let stderr = normalize_output(
        &decode(&output.stderr),
        &case.path,
        &case.directives.normalizations,
    );

    if output.status.code() != Some(0) {
        return Err(Failed::from(format!(
            "expected success, got exit {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
            output.status.code()
        )));
    }

    compare_sidecar(config, case, "run.stdout", &stdout)?;
    compare_sidecar(config, case, "run.stderr", &stderr)?;

    Ok(())
}

fn compare_sidecar(
    config: &Config,
    case: &TestCase,
    extension: &str,
    actual: &str,
) -> Result<(), Failed> {
    let path = case.sidecar_path(extension);

    if config.bless {
        if actual.is_empty() && !path.exists() {
            return Ok(());
        }
        write_expected(&path, actual)?;
        return Ok(());
    }

    if !path.exists() {
        if actual.is_empty() {
            return Ok(());
        }
        return Err(Failed::from(format!(
            "missing expectation file {} (actual output was non-empty)",
            path.display()
        )));
    }

    let expected = read_expected(&path)?;
    if actual != expected {
        return Err(Failed::from(format!(
            "{extension} mismatch for {}\n--- expected ({}) ---\n{expected}\n--- actual ---\n{actual}",
            case.name,
            path.display()
        )));
    }

    Ok(())
}

fn decode(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn read_expected(path: &Path) -> Result<String, Failed> {
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(path).map_err(|err| {
        Failed::from(format!("failed to read {}: {err}", path.display()))
    })
}

fn write_expected(path: &Path, contents: &str) -> Result<(), Failed> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            Failed::from(format!("failed to create {}: {err}", parent.display()))
        })?;
    }
    fs::write(path, contents).map_err(|err| {
        Failed::from(format!("failed to write {}: {err}", path.display()))
    })
}
