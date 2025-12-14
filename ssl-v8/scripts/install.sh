#!/bin/bash
# SSL v8.0 Installer Script for Unix/Linux/macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.sh | sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SSL_VERSION="8.0.0"
GITHUB_REPO="SonnerStudio/SSL-v8"
INSTALL_DIR="/usr/local/bin"

echo -e "${GREEN}SSL v${SSL_VERSION} Installer${NC}"
echo "=========================="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    *)          echo -e "${RED}Unsupported OS: ${OS}${NC}"; exit 1;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64)     ARCH_NAME=x64;;
    arm64)      ARCH_NAME=arm64;;
    aarch64)    ARCH_NAME=arm64;;
    *)          echo -e "${RED}Unsupported architecture: ${ARCH}${NC}"; exit 1;;
esac

# Construct binary name
if [ "$PLATFORM" = "macos" ] && [ "$ARCH_NAME" = "arm64" ]; then
    BINARY_NAME="ssl-macos-arm64"
else
    BINARY_NAME="ssl-${PLATFORM}-${ARCH_NAME}"
fi

echo -e "Detected: ${GREEN}${PLATFORM}${NC} (${ARCH_NAME})"
echo ""

# Download URL
DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/download/v${SSL_VERSION}/${BINARY_NAME}"

echo "Downloading SSL v${SSL_VERSION}..."
echo "From: ${DOWNLOAD_URL}"
echo ""

# Check for dry-run mode
if [ "$1" = "--dry-run" ]; then
    echo -e "${YELLOW}DRY RUN MODE - No files will be downloaded or installed${NC}"
    echo "Would download from: ${DOWNLOAD_URL}"
    echo "Would install to: ${INSTALL_DIR}/ssl"
    exit 0
fi

# Download binary
TMP_DIR=$(mktemp -d)
TMP_FILE="${TMP_DIR}/${BINARY_NAME}"

if command -v curl >/dev/null 2>&1; then
    curl -fL "${DOWNLOAD_URL}" -o "${TMP_FILE}"
elif command -v wget >/dev/null 2>&1; then
    wget -q "${DOWNLOAD_URL}" -O "${TMP_FILE}"
else
    echo -e "${RED}Error: curl or wget required${NC}"
    exit 1
fi

# Make executable
chmod +x "${TMP_FILE}"

# Install
echo "Installing to ${INSTALL_DIR}..."

if [ -w "${INSTALL_DIR}" ]; then
    mv "${TMP_FILE}" "${INSTALL_DIR}/ssl"
else
    echo "Requesting sudo for installation..."
    sudo mv "${TMP_FILE}" "${INSTALL_DIR}/ssl"
fi

# Cleanup
rm -rf "${TMP_DIR}"

# Verify
echo ""
echo -e "${GREEN}✓ Installation complete!${NC}"
echo ""
echo "Verifying installation..."
if "${INSTALL_DIR}/ssl" --version >/dev/null 2>&1; then
    echo -e "${GREEN}✓ SSL is working correctly${NC}"
    "${INSTALL_DIR}/ssl" --version
else
    echo -e "${YELLOW}⚠ SSL installed but verification failed${NC}"
fi

echo ""
echo "Run 'ssl --help' to get started!"
