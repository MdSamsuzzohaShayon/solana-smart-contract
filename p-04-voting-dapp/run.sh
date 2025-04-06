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

