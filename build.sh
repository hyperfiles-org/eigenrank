#!/bin/bash
# Run the following before executing this build script:
# chmod +x build.sh

# Stop executing after any error
set -e

echo "Cleaning old builds..."
# Clean existing build artifacts to ensure a fresh start
cargo clean

echo "Building contract..."
# Build the Rust project and generate Wasm file specifically for the WASM target
cargo build --target wasm32-unknown-unknown --release

echo "Post-processing..."
# Navigate to the target directory where the wasm file is located
cd target/wasm32-unknown-unknown/release

# Using wasm-opt to optimize the wasm file, if wasm-opt is installed
# This step is optional but recommended to reduce the wasm size and optimize execution
# Ensure wasm-opt is installed and accessible in your PATH for this step to work
if command -v wasm-opt &> /dev/null
then
    echo "Optimizing WASM using wasm-opt..."
    wasm-opt -Oz -o eigentrust.wasm eigentrust_near.wasm
else
    echo "wasm-opt not found, skipping optimization."
    mv eigentrust_near.wasm eigentrust.wasm
fi

echo "Build complete. Wasm file located at $(pwd)/eigentrust.wasm"
