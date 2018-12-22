//! Helper methods for git-remote-open's cli command.
#![cfg_attr(feature = "strict", deny(warnings))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

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
