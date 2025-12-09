#!/bin/bash
# SSL v7.0 One-Line Installer for Linux/macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/SonnerStudio/Sonner-Studio-Language_SSL/main/install.sh | bash

set -e

echo "Installing SSL v7.0..."

VERSION="v7.0.0"
INSTALL_DIR="$HOME/.local/bin"

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [[ $(uname -m) == "x86_64" ]]; then
        URL="https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/$VERSION/ssl-linux-x64.tar.gz"
    elif [[ $(uname -m) == "aarch64" ]]; then
        URL="https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/$VERSION/ssl-linux-arm64.tar.gz"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == "arm64" ]]; then
        URL="https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/$VERSION/ssl-macos-arm64.tar.gz"
    else
        URL="https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/$VERSION/ssl-macos-intel.tar.gz"
    fi
else
    echo "❌ Unsupported platform: $OSTYPE"
    exit 1
fi

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and install
TEMP_FILE=$(mktemp)
echo "Downloading from $URL..."
curl -fsSL "$URL" -o "$TEMP_FILE"

echo "Installing to $INSTALL_DIR..."
tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/ssl"

# Cleanup
rm "$TEMP_FILE"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Add to your shell profile (~/.bashrc or ~/.zshrc):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "✅ SSL v7.0 installed successfully!"
echo ""
echo "Test it:"
echo "  ssl --version"
