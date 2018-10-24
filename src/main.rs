extern crate git_remote_open;
extern crate open;

use std::env;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

fn open_file(file_path: &str) -> Result<ExitStatus, std::io::Error> {
    open::that(file_path)
}

fn main() {
    if !git_remote_open::git_exists() {
        eprintln!("Unable to find git on your system.");
        exit(1);
    }

    let directory: String = env::args().skip(1).take(1).collect();
    let root = Path::new(&directory);
    env::set_current_dir(&root).ok();

    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .expect("unable to retrieve git remote references");

    let output = String::from_utf8_lossy(&output.stdout);
    if output.is_empty() {
        println!("No git remote named \"origin\" found.");
    } else if let Err(_err) = open_file(output.to_string().trim()) {
        println!("Unable to open {}", output.to_string().trim());
        exit(3);
    }
}
