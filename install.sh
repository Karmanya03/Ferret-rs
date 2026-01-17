#!/bin/bash

# Installation script for Ferret (fr)
# For Linux/Unix systems (Ubuntu, Arch, Kali, Debian, Fedora, etc.)

set -e

echo "=================================="
echo "Ferret 🦡 - Installer"
echo "=================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "After installation, run: source ~/.cargo/env"
    exit 1
fi

echo "✓ Rust found: $(rustc --version)"
echo ""

# Build the project
echo "🔨 Building Ferret..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo ""
else
    echo "❌ Build failed!"
    exit 1
fi

# Install to /usr/local/bin
echo "📦 Installing to /usr/local/bin..."

if [ -w /usr/local/bin ]; then
    cp target/release/fr /usr/local/bin/
    echo "✓ Installed to /usr/local/bin/fr"
else
    echo "Need sudo privileges to install to /usr/local/bin"
    sudo cp target/release/fr /usr/local/bin/
    echo "✓ Installed to /usr/local/bin/fr"
fi

# Verify installation
if command -v fr &> /dev/null; then
    echo ""
    echo "=================================="
    echo "✅ Installation successful!"
    echo "=================================="
    echo ""
    echo "You can now use 'fr' command:"
    echo "  fr find '*.txt'         - Find files"
    echo "  fr organize ~/Downloads - Organize files"
    echo "  fr stats                - Show directory stats"
    echo ""
    echo "Quick examples:"
    echo "  fr find '*config*' -i   - Find config files"
    echo "  fr find '*.log' --min-size 10M  - Find large logs"
    echo "  fr find '*.tmp' -x 'rm {} 2>/dev/null'  - Delete temps"
    echo ""
    echo "For more help: fr --help"
    echo ""
else
    echo ""
    echo "⚠️  Installation completed but 'fr' not found in PATH"
    echo "You may need to add /usr/local/bin to your PATH"
    echo ""
fi
