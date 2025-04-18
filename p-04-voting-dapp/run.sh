#!/bin/bash

# Create a new Solana dapp
npx create-solana-dapp

# Install dependencies
cd <project-name>
npm install

# Run the Solana test validator for local development  in another terminal
solana-test-validator

# Run the development server
npm run dev


# Using solana explorer to view the local development environment 
# Go to https://explorer.solana.com/ then set custom RPC to http://localhost:8899 (got from solana-test-validator) and You should see the local development environment in the explorer

# Install "vscode-icons" extension to get icons in vscode, type ctrl+p and paste this command `ext install vscode-icons-team.vscode-icons`


# Build the anchor program to get the program id and deploy the program to the local development environment
cd anchor
anchor build

# anchor-bankrun is a small but powerful extension to solana-bankrun that enables using both Anchor and Bankrun with only a one-line code change. 
npm install anchor-bankrun

# Create directory and copy binary file
mkdir test/fixtures
cp -r target/deploy/voting.so tests/fixtures/voting.so

# Run the tests
anchor test --skip-local-validator --skip-deploy

# Bankrun is depricated, use liteSVM instead - https://github.com/LiteSVM/litesvm/tree/master/crates/node-litesvm


# Get the current solana config
solana config get

# Run the test validator
solana-test-validator --reset

# Set the current solana config to the local development environment
solana config set -ul

# Generate a new keypair
solana-keygen new -o /home/shayon/.config/solana/id.json

# Get the address of the new keypair
solana address

# Airdrop 5 SOL to the new keypair
solana airdrop 5

cd anchor
# Deploy the program
solana program deploy target/deploy/voting.so


