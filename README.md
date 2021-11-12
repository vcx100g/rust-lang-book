# rust-lang-book

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

```bash
$ rustup update
```

```bash
$ rustup self uninstall
```

```bash
$ rustc --version
```

## Create hello world

create rush _main.rs_
```rust
fn main() {
    println!("Hello, world!");
}
```

compile and run main
```bash
$ rustc main.rs
$ ./main
Hello, world!
```

## Cargo

```bash
$ cargo --version
```

create new cargo
```bash
$ cargo new hello_cargo
```

filename: _Cargo.toml_
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2018"

[dependencies]
```

build cargo
```bash
$ cargo build
```

run cargo
```bash
$ cargo run
```

check cargo info
```bash
$ cargo check
```

build cargo for release
```bash
$ cargo build
```

## how to use dependencies
in cargo toml add
```toml
[dependencies]
rand = "0.8.4"
```

run cargo update to install dependencies
```bash
$ cargo update
```

then build and run 

install doc for dependencies
then open it on broswer
```bash
$ cargo doc --open
```

Add a test library
```bash
$ cargo new adder --lib
```

run 1 thread for test, will not use parallel
```bash
$ cargo test -- --test-threads=1
```

show test output
```bash
$ cargo test -- --show-output
```

run a single test function
```bash
$ cargo test tests::one_hundred
```

run multiple test with same name
```bash
$ cargo test add
```

run only ignored test
```bash
$ cargo test -- --ignored
```

test folder run test file
```bash
$ cargo test --test integration_test
```

run with environment variables with arguments
```bash
$ CASE_INSENSITIVE=1 cargo run to poem.txt
```

output to file
```bash
$ cargo run > output.txt
$ cargo run to poem.txt > output.txt
```

## Build cargo release

unoptimized with debug build
```bash
$ cargo build
```

optimized build for release
```bash
$ cargo build --release
```

set compile speed for build
0-3 higher is more optimized
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Publish to crate.io

check how to write doc in publish_crate
open doc for cargo
```bash
$ cargo doc --open
```

[https://crates.io/](https://crates.io/)
Create account and retrieve your API key login using terminal.
API token and store it locally in ~/.cargo/credentials
```bash
$ cargo loign abcdefghijklmnopqrstuvwxyz012345
```

set package name for your crate at _Cargo.toml_
```toml
name = "guessing_game"
version = "0.1.0"
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

publish or update new version using
```bash
$ cargo publish
```

Removing versions from crates.io

```bash
$ cargo yank --vers 1.0.1
```

By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again
```bash
$ cargo yank --vers 1.0.1 --undo
```