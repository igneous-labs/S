//! Known workspace paths resolution

use std::path::{Path, PathBuf};

/// Copied from https://stackoverflow.com/a/74942075/5057425
pub fn workspace_root_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn test_fixtures_dir() -> PathBuf {
    workspace_root_dir().join("test-fixtures")
}
