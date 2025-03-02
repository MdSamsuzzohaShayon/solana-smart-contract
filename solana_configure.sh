#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

# Step 1: Completely remove Solana
echo "Removing old Solana installation..."
rm -rf ~/.local/share/solana ~/.config/solana ~/.cache/solana
rm -f /usr/local/bin/solana*

echo "Solana removed successfully."

# Step 2: Completely remove Rust
echo "Removing Rust installation..."
rustup self uninstall -y
rm -rf ~/.rustup ~/.cargo

echo "Rust removed successfully."

# Step 3: Clean up any remaining files
echo "Cleaning up system..."
sudo rm -rf /usr/local/lib/rust* /usr/local/cargo*
sudo rm -rf /usr/bin/rustc /usr/bin/cargo

echo "Cleanup completed."

echo "All Rust and Solana-related files have been removed successfully."

# Step 2: Reinstall Solana CLI
echo "Installing Solana CLI..."
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Step 3: Reload environment variables
echo "Reloading environment variables..."
source ~/.profile

# Verify Solana installation
echo "Checking Solana installation..."
solana --version || { echo "Solana installation failed!"; exit 1; }

echo "Solana installed successfully."

# Step 4: Install Solana's SBF Rust Toolchain manually
echo "Installing Solana SBF Rust toolchain..."
solana config set -u devnet
solana-install update || { echo "Solana SBF tools installation failed! Trying an alternative method..."; }

# Step 5: Verify installation
echo "Verifying Solana Rust toolchain..."
if ls -lah ~/.local/share/solana/install/releases/stable-*/solana-release/bin/sdk/sbf/dependencies/platform-tools/rust/bin/ 2>/dev/null; then
    echo "Solana Rust toolchain installed successfully."
else
    echo "Solana Rust toolchain installation failed!"
    exit 1
fi

# Step 6: Build the Solana Program
echo "Building Solana program..."
cargo clean
cargo build-sbf || { echo "Solana build failed!"; exit 1; }

echo "Solana program built successfully."
