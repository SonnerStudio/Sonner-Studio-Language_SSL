#!/bin/bash

# SSL v9 macOS Installer (Universal)

set -e

echo "Searching for SSL v9 binaries..."

# Determine Architecture
ARCH=$(uname -m)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="ssl-v9"

if [ "$ARCH" == "x86_64" ]; then
    echo "Detected Architecture: Intel (x86_64)"
    RELATIVE_PATH="binaries/macos/intel/ssl-v9"
elif [ "$ARCH" == "arm64" ]; then
    echo "Detected Architecture: Apple Silicon (arm64)"
    RELATIVE_PATH="binaries/macos/apple_silicon/ssl-v9"
else
    echo "Error: Unsupported architecture: $ARCH"
    exit 1
fi

LOCAL_PATH="$SCRIPT_DIR/$RELATIVE_PATH"
REMOTE_URL="https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/raw/main/$RELATIVE_PATH"

if [ -f "$LOCAL_PATH" ]; then
    SOURCE_PATH="$LOCAL_PATH"
else
    echo "Binary not found locally. Downloading from GitHub..."
    SOURCE_PATH="/tmp/ssl-v9-install"
    curl -L -o "$SOURCE_PATH" "$REMOTE_URL"
fi

echo "Found binary at: $SOURCE_PATH"

# Install
echo "Installing to $INSTALL_DIR/$BINARY_NAME..."
echo "This requires sudo privileges."

sudo cp "$SOURCE_PATH" "$INSTALL_DIR/$BINARY_NAME"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "-----------------------------------"
echo "Installation Successful!"
echo "-----------------------------------"
echo "Verifying installation:"
$BINARY_NAME --version || echo "Warning: Could not run $BINARY_NAME. You may need to restart your terminal."
