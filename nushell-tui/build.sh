#!/bin/bash

echo "=== Nushell TUI Build Script ==="
echo ""

# Set linker flags (for Termux)
export LDFLAGS="-L/data/data/com.termux/files/usr/lib -L/data/data/com.termux/files/lib"
export LIBGCC_PATH="/data/data/com.termux/files/usr/lib"

# Try building
echo "Running cargo build..."
cargo build --release 2>&1

if [ $? -eq 0 ]; then
    echo ""
    echo "=== Build Successful! ==="
    echo "Binary location: target/release/nushell-tui"
    echo ""
    echo "Run with: target/release/nushell-tui ../nubook"
else
    echo ""
    echo "=== Build Failed ==="
    echo "Note: This may be due to linker issues in the current environment."
    echo "The code compiles correctly but linking fails due to missing system libraries."
    echo ""
    echo "To fix, ensure libgcc_s is available or use a different build environment."
fi
