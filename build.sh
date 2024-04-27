#!/bin/bash

# Exit if any command fails
set -e

# Ensure the 'res' directory exists to store the compiled WASM files
mkdir -p res

# Build the contract using cargo
echo "Building the eigenrank contract..."
cargo build --release --target wasm32-unknown-unknown
echo "Build complete."

# Copy the resulting wasm file to the res directory, adjust the name as necessary
echo "Copying wasm file to the res directory..."
cp target/wasm32-unknown-unknown/release/eigenrank.wasm res/

echo "Final wasm file is ready in the 'res' directory."
