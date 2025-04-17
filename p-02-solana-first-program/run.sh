#!/bin/bash

# This script sets up a new Solana program project
cargo new p-02-solana-first-program

# Create a command to install solana-program 2.1.20 version
cargo add solana-program@2.1.20

# Install nightly rust for solana program build
rustup install nightly

# Build the solana program
cargo +nightly build-sbf



