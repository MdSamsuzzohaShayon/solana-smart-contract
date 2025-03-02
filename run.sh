#/bin/bash

solana --version


# Using Solana CLI - https://www.youtube.com/watch?v=HF-FuNtvZRE&list=PLndmg9UIKNoku3lbeSKyW-RqHt-HRclEu&index=4

# Install all dependencies 
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/solana-developers/solana-install/main/install.sh | bash


# Install only Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
PATH="/home/shayon/.local/share/solana/install/active_release/bin:$PATH"


# Set local development environments
# Docs - https://solana.com/docs/intro/installation#solana-config
solana --version
solana config get
# update the Solana CLI cluster
solana config set --url devnet

# The Solana test validator is a local emulator for the Solana blockchain, designed to provide developers with a private and controlled environment for building and testing Solana programs without the need to connect to a public testnet or mainnet.
# Docs - https://solana.com/developers/guides/getstarted/solana-test-validator
solana-test-validator
# Get that url from test net validator json rpc url
solana config set --url  http://127.0.0.1:8899


# Create Wallet
# Docs - https://solana.com/docs/intro/installation#create-wallet
solana-keygen new
cat /home/shayon/.config/solana/id.json
solana address
# Is solana address is not showing run command below
# solana address ~/.config/solana/id.json


# Check wallet balance
solana balance
# Address will be shown when we run solana address
solana balance BNaH5xsqUs3PRjLRc2FgKZCT5wCbpXF59Rr9cQGD7CXg

# Airdrop SOL
solana airdrop 2

# Account details
solana account BNaH5xsqUs3PRjLRc2FgKZCT5wCbpXF59Rr9cQGD7CXg

solana slot
solana block


# Reference - https://solana.com/docs/references/terminology


# Create solana project from scratch using rust and cargo
cargo new p-01-solana-first-program
# Install dependency
cargo add solana-program@2.1.14




