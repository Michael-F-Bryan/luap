use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use libtest_mimic::{Failed, Trial};
use luap::{diagnostics::Diagnostic, Compiler, SourceFile};

static TEST_CASES: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("test_cases"));

pub fn discover() -> Vec<Trial> {
    let mut test_cases = Vec::new();

    for entry in TEST_CASES.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some("lua".as_ref()) {
            test_cases.push(TestCase::from_path(path));
        }
    }

    test_cases.into_iter().map(TestCase::into_trial).collect()
}

#[derive(Debug, Clone)]
struct TestCase {
    path: PathBuf,
    name: String,
    ignored: bool,
}

impl TestCase {
    fn from_path(path: PathBuf) -> Self {
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap();
        let (name, ignored) = match name.strip_prefix("_") {
            Some(name) => (name, true),
            None => (name, false),
        };

        TestCase {
            name: name.to_string(),
            path,
            ignored,
        }
    }

    fn into_trial(self) -> Trial {
        Trial::test(self.name, move || {
            let compiler = Compiler::default();
            let source_file = SourceFile::from_path(&compiler, &self.path)?;

            let diagnostics: Vec<&Diagnostic> = luap::lower::accumulated(&compiler, source_file);

            if !diagnostics.is_empty() {
                return Err(Failed::from("diagnostics"));
            }

            let mut settings = insta::Settings::clone_current();

            settings.set_input_file(&self.path);
            settings.set_omit_expression(true);
            let snapshot_path = self.path.with_extension(".snap");
            settings.set_snapshot_path(snapshot_path);

            settings.bind(|| {
                let lowered = luap::lower(&compiler, source_file);
                insta::assert_debug_snapshot!(&lowered);
            });

            Ok(())
        })
        .with_ignored_flag(self.ignored)
    }
}
