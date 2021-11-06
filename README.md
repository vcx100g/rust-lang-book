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
