#![cfg_attr(feature = "strict", deny(warnings))]

use std::process::{Command, Stdio};

/// Determine if git exists as an executable.
pub fn git_exists() -> bool {
    Command::new("git")
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_ok()
}
