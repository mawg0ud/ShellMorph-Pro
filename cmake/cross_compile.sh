#!/bin/bash

set -e
echo "Starting cross-compilation for ShellMorph Pro..."

# Define target platforms
PLATFORMS=("x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu")

# Install Rust target platforms if not already installed
for PLATFORM in "${PLATFORMS[@]}"; do
    if ! rustup target list | grep -q "${PLATFORM} (installed)"; then
        rustup target add $PLATFORM
    fi
done

# Compile for each platform
for PLATFORM in "${PLATFORMS[@]}"; do
    echo "Building for $PLATFORM..."
    cargo build --release --target $PLATFORM
done

echo "Cross-compilation completed successfully!"
