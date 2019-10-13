use std::process::Command;
use std::str;

#[test]
fn it_will_output_to_stderr_if_git_not_found() {
    let binary_name = env!("CARGO_PKG_NAME");
    let output = Command::new(format!("./target/debug/{}", binary_name))
        .env("PATH", "")
        .output()
        .expect("failed to execute process");

    assert_eq!(1, output.status.code().unwrap());
    assert_eq!(
        "Unable to find git on your system.\n",
        str::from_utf8(&output.stderr).unwrap()
    );
}
