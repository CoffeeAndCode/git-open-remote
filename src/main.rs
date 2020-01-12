//! git-remote-open's a small cli app to quickly open your git remote url.
//!
//! If called without arguments it will use the current directory when looking
//! for the git remote to use. If `origin` is not found, an error is output.
//!
//! If interested, you can add an `alias` entry to your global .gitconfig file
//! similar to:
//!
//! ```
//! [alias]
//! open = "!git-remote-open"
//! ```
#![cfg_attr(feature = "strict", deny(warnings))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use git_remote_open;
use open;
use std::env;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

fn git_url_to_https(url: &str) -> String {
    url.replace("git@bitbucket.org:", "https://bitbucket.org/")
        .replace("git@github.com:", "https://github.com/")
        .replace("git@gitlab.com:", "https://gitlab.com/")
}

fn open_file(file_path: &str) -> Result<ExitStatus, std::io::Error> {
    open::that(git_url_to_https(file_path))
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

#[test]
fn test_git_url_for_bitbucket() {
    assert_eq!(
        "https://bitbucket.org/CoffeeAndCode/git-remote-open.git",
        git_url_to_https("git@bitbucket.org:CoffeeAndCode/git-remote-open.git")
    )
}

#[test]
fn test_git_url_for_github() {
    assert_eq!(
        "https://github.com/CoffeeAndCode/git-remote-open.git",
        git_url_to_https("git@github.com:CoffeeAndCode/git-remote-open.git")
    )
}

#[test]
fn test_git_url_for_gitlab() {
    assert_eq!(
        "https://gitlab.com/CoffeeAndCode/git-remote-open.git",
        git_url_to_https("git@gitlab.com:CoffeeAndCode/git-remote-open.git")
    )
}
