use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub bless: bool,
    pub luap: PathBuf,
    pub suites: PathBuf,
}

impl Config {
    pub fn luap_binary() -> PathBuf {
        if let Some(path) = std::env::var_os("CARGO_BIN_EXE_luap") {
            return PathBuf::from(path);
        }
        if let Some(path) = std::env::var_os("LUAP_BIN") {
            return PathBuf::from(path);
        }

        let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".into());
        let workspace_luap = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(profile)
            .join("luap");
        if workspace_luap.exists() {
            return workspace_luap;
        }

        PathBuf::from("luap")
    }
}
