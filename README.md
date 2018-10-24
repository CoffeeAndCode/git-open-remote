# git-remote-open

[![Build Status](https://travis-ci.org/CoffeeAndCode/git-remote-open.svg?branch=master)](https://travis-ci.org/CoffeeAndCode/git-remote-open)

CLI helper to quickly open git remote origin url in a browser. Run
`git-remote-open` from a `git` directory to open the `origin` remote's url in
your default browser.

You can also pass in the directory you'd like to open
as the first argument like `git-remote-open ../some/other/folder`

If you'd like, you can alias the command in your `~/.gitconfig` file which
would allow you to call `git open` from a project repo.

```ini
[alias]
    open = "git-remote-open"
```

## Installation

This package is not pushed to crates.io, but you can install it directly from
source by running:

```shell
cargo install --force git-remote-open --git https://github.com/CoffeeAndCode/git-remote-open
```

## Development

The project uses [clippy](https://github.com/rust-lang-nursery/rust-clippy) for
linting. It will also use [rustfmt](https://github.com/rust-lang-nursery/rustfmt)
to format all Rust files.

You can run `bin/lint` with a recent version of Rust to run the same checks the
CI server will run.

Run `cargo test` to run the project's unit and integration tests.
