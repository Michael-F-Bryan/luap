use std::fs;
use std::path::Path;

use libtest_mimic::Trial;

use crate::config::Config;
use crate::directives::{suite_from_path, Suite, TestCase};
use crate::runner::run_test;

pub fn discover(config: &Config) -> Vec<Trial> {
    let mut cases = Vec::new();
    collect_lua_files(&config.suites, &config.suites, &mut cases);

    cases
        .into_iter()
        .map(|case| {
            let ignored = case.directives.ignore;
            let name = case.name.clone();
            let config = config.clone();
            Trial::test(name, move || run_test(&config, &case)).with_ignored_flag(ignored)
        })
        .collect()
}

fn collect_lua_files(root: &Path, dir: &Path, cases: &mut Vec<TestCase>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|err| {
        panic!("failed to read {}: {err}", dir.display());
    });

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            collect_lua_files(root, &path, cases);
            continue;
        }

        if path.extension().is_none_or(|ext| ext != "lua") {
            continue;
        }

        let Some(suite) = suite_from_path(root, &path) else {
            continue;
        };

        if suite == Suite::Hir {
            continue;
        }

        let source = fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!("failed to read {}: {err}", path.display());
        });

        let directives = crate::directives::Directives::parse(&source, suite, &path)
            .unwrap_or_else(|err| panic!("{err}"));

        let relative = path
            .strip_prefix(root)
            .unwrap()
            .with_extension("")
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");

        cases.push(TestCase {
            name: relative,
            path,
            suite,
            directives,
        });
    }
}
