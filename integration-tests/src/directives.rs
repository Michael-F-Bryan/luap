use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suite {
    Ui,
    Run,
    Hir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    CheckPass,
    CheckFail,
    Run,
}

#[derive(Debug, Clone)]
pub struct Directives {
    pub ignore: bool,
    pub expectation: Expectation,
    pub normalizations: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DirectiveError {
    pub message: String,
}

impl fmt::Display for DirectiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl Directives {
    pub fn parse(source: &str, suite: Suite, test_path: &Path) -> Result<Self, DirectiveError> {
        let mut ignore = false;
        let mut expectation: Option<Expectation> = None;
        let mut normalizations = Vec::new();

        for (line_no, line) in source.lines().enumerate() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("--@") else {
                continue;
            };
            let rest = rest.trim();
            if rest.is_empty() {
                continue;
            }

            if let Some((key, value)) = rest.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "normalize-stderr" => {
                        let Some((from, to)) = value.split_once("->") else {
                            return Err(DirectiveError {
                                message: format!(
                                    "{}:{}: normalize-stderr requires `from -> to` syntax",
                                    test_path.display(),
                                    line_no + 1
                                ),
                            });
                        };
                        normalizations.push((
                            from.trim().trim_matches('"').to_string(),
                            to.trim().trim_matches('"').to_string(),
                        ));
                    }
                    other => {
                        return Err(DirectiveError {
                            message: format!(
                                "{}:{}: unknown directive `{other}`",
                                test_path.display(),
                                line_no + 1
                            ),
                        });
                    }
                }
                continue;
            }

            match rest {
                "ignore" => ignore = true,
                "check-pass" => expectation = Some(Expectation::CheckPass),
                "check-fail" => expectation = Some(Expectation::CheckFail),
                "run" => expectation = Some(Expectation::Run),
                other => {
                    return Err(DirectiveError {
                        message: format!(
                            "{}:{}: unknown directive `{other}`",
                            test_path.display(),
                            line_no + 1
                        ),
                    });
                }
            }
        }

        let expectation = expectation.unwrap_or_else(|| default_expectation(suite));
        validate_expectation(suite, expectation, test_path)?;

        Ok(Self {
            ignore,
            expectation,
            normalizations,
        })
    }
}

fn default_expectation(suite: Suite) -> Expectation {
    match suite {
        Suite::Ui => Expectation::CheckFail,
        Suite::Run => Expectation::Run,
        Suite::Hir => Expectation::CheckFail,
    }
}

fn validate_expectation(
    suite: Suite,
    expectation: Expectation,
    test_path: &Path,
) -> Result<(), DirectiveError> {
    let valid = match suite {
        Suite::Ui => matches!(expectation, Expectation::CheckPass | Expectation::CheckFail),
        Suite::Run => expectation == Expectation::Run,
        Suite::Hir => matches!(expectation, Expectation::CheckPass | Expectation::CheckFail),
    };

    if valid {
        return Ok(());
    }

    Err(DirectiveError {
        message: format!(
            "{}: {:?} suite does not support {:?} expectation",
            test_path.display(),
            suite,
            expectation
        ),
    })
}

pub fn suite_from_path(suites_root: &Path, path: &Path) -> Option<Suite> {
    let relative = path.strip_prefix(suites_root).ok()?;
    let component = relative.components().next()?;
    match component.as_os_str().to_str()? {
        "ui" => Some(Suite::Ui),
        "run" => Some(Suite::Run),
        "hir" => Some(Suite::Hir),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub path: PathBuf,
    pub suite: Suite,
    pub directives: Directives,
}

impl TestCase {
    pub fn sidecar_path(&self, extension: &str) -> PathBuf {
        self.path.with_extension(extension)
    }
}
