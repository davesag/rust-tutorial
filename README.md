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

### Things to note

### Strings

* A `String` is just a vector of 8-byte chars
* String constants can't be passed in to macros.

  so you can't do

      println!(SOME_STRING_CONSTANT)

  but you can do

      fn printer(msg: &str) { println!("{}", msg); }
      printer(&SOME_STRING_CONSTANT);

  but functions can't have default or optional params (that I can work out anyway)

  You _can't_ do something like

      fn printer(msgFmt: &str, val: u32) { println!(&msgFmt, val); }

### Integers

Length | Signed | Unsigned
-------|--------|---------
8-bit  | i8     | u8
16-bit | i16    | u16
32-bit | i32    | u32
64-bit | i64    | u64
arch   | isize  | usize

### Floats

`f32` and `f64`. `f64` is the default.

### Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // with destructuring
    let (x, y, z) = tup;
    // or via dot notation
    let n = tup.0; // gives 500

### Arrays

Arrays are fixed. If you need to add or remove items from a list use a `vector`.

    let a = [1, 2, 3, 4, 5];
    let one = a[0];

#### `let` vs `const` and the use of `mut`

Constants must have a type.

    const A_NUMBER: u32 = 153;
    const A_CONSTANT: &str = "This can't be changed or shadowed.";

Variables are immutable by default

    let my_string = "this can't be changed directly";

but can be shadowed

    let my_string = "but it can be shadowed via use of another let"
    let my_string = my_string.len() // including being shadowed by a new type.

Variables can use `mut` to declare them as mutable.

    let mut my change_me = "this can be changed"
    change_me = "but not to a new type"

### Expressions and statements

If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

So

    {
      let x = 3;
      x + 1
    }

is an expression that evaluates to `4`

### Return values from functions

    fn five() -> i32 {
        5
    }

    let x = five();

    fn add_one(n: i32) -> i32 {
        n + 1
    }

    let y = add_one(x);

### `if` .. `else` can be assigned

    fn some_test(is_it: bool) -> i32 {
      if is_it {
        1
      } else }
        0
      }
    }

    let x = is_it(true) // 1
    let y = is_it(false) // 0

### History and other info

* Rust was originally developed by [Graydon Hoare](https://twitter.com/graydon_pub) who now wroks at Apple on the Swift language.
* [Graydon is a biology nerd](https://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/).
* Rust is named after [a fungus](https://en.wikipedia.org/wiki/Rust_%28fungus%29) that is robust, distributed, and parallel.
* A Rust developer is known as a 'Rustacean' (rhymes with crustacean)
