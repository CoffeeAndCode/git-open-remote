use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn it_will_output_to_stderr_if_git_not_found() {
    Command::main_binary()
        .unwrap()
        .env("PATH", "")
        .assert()
        .stderr("Unable to find git on your system.\n")
        .code(1);
}
