use std::path::Path;

use regex::Regex;

pub fn normalize_output(output: &str, test_path: &Path, replacements: &[(String, String)]) -> String {
    let mut normalized = output.replace("\r\n", "\n");

    if let Ok(canon) = test_path.canonicalize() {
        let canon = canon.to_string_lossy();
        normalized = normalized.replace(canon.as_ref(), "$FILE");
    }
    let path = test_path.to_string_lossy();
    normalized = normalized.replace(path.as_ref(), "$FILE");

    if let Some(parent) = test_path.parent() {
        if let Ok(canon) = parent.canonicalize() {
            let canon = canon.to_string_lossy();
            normalized = normalized.replace(canon.as_ref(), "$DIR");
        }
        let parent = parent.to_string_lossy();
        normalized = normalized.replace(parent.as_ref(), "$DIR");
    }

    normalized = normalized.replace('\\', "/");

    for (from, to) in replacements {
        let pattern = Regex::new(from).unwrap_or_else(|err| {
            panic!("invalid normalize-stderr pattern `{from}`: {err}");
        });
        normalized = pattern.replace_all(&normalized, to.as_str()).into_owned();
    }

    normalized
}
