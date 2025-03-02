#/bin/bash

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compiling and running
# Ref - https://doc.rust-lang.org/cargo/reference/unstable.html?highlight=rustc#rustc---print
rustc 01_main.rs
./01_main

# Ref - https://doc.rust-lang.org/cargo/
# Create a project with Cargo
cargo new cargo_project
cargo build
ls ./target/debug
./target/debug/cargo_project
cargo run
cargo check

# Formatting
rustfmt src/main.rs