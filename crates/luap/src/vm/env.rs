use std::io::Write;

pub struct Environment {
    pub stdout: Box<dyn Write>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            stdout: Box::new(std::io::stdout()),
        }
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Environment { stdout: _ } = self;
        f.debug_struct("Environment").finish_non_exhaustive()
    }
}
