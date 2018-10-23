use std::process::{Command, Stdio};

pub fn git_exists() -> bool {
    Command::new("git")
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_ok()
}
