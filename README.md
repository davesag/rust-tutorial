# rust-tutorial

Just me [learning Rust](https://doc.rust-lang.org) by doing the tutorial exercises.

* `develop` [![CircleCI](https://circleci.com/gh/davesag/rust-tutorial/tree/develop.svg?style=svg)](https://circleci.com/gh/davesag/rust-tutorial/tree/develop)
* `master` [![CircleCI](https://circleci.com/gh/davesag/rust-tutorial/tree/master.svg?style=svg)](https://circleci.com/gh/davesag/rust-tutorial/tree/master)

## Getting started with Rust on macOS (with Homebrew)

### Install stuff

    brew install rustup
    rustup-init

Use the defaults then, once that's done also do

    rustup completions bash > $(brew --prefix)/etc/bash_completion.d/rustup.bash-completion
    source $HOME/.cargo/env

check it all worked with

    rustc --version

You should get something like

    ~$ rustc 1.25.0 (84203cac6 2018-03-25)

Now install `cargo-edit`

    cargo install cargo-edit

Let that complete.

Now you can add dependencies to your project using

    cargo add <some-library>

instead of having to look up the speccific library version and manually edit the `Cargo.toml` file.

Then add `rustfmt-preview` to your toolchain.

    rustup component add rustfmt-preview

Now you can run the rust code formatter (aka linting)

### Keep it up to date

    rustup update

### Handy `cargo` commands

Create a new application project

    cargo new <project_name>

Create a new library project

    cargo new <project_name> --lib

Generate project documentation (including for dependencies) and open the docs in your browser

    cargo doc --open

Build a project

    cargo build

Run the debug version of the built project

    cargo run

Build and run the release version of the project

    cargo build --release
    cargo run --release

Run tests (when there are tests to run)

    cargo test

Linting

    cargo fmt -- --write-mode=diff
