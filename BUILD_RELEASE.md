# SSL v7.0 Binary Release Builder

## Automated Build Script

This script creates release packages for all platforms.

### Prerequisites

```bash
# Install PyInstaller
pip install pyinstaller

# For cross-platform builds (optional)
pip install pyinstaller-hooks-contrib
```

### Build Commands

**Windows x64:**
```powershell
# Build executable
python -m PyInstaller --onefile --name ssl ssl-v7\ssl.py

# Create release package
New-Item -ItemType Directory -Force release\windows-x64
Copy-Item dist\ssl.exe release\windows-x64\
Copy-Item ssl-v7\README.txt release\windows-x64\
Copy-Item LICENSE release\windows-x64\
Compress-Archive -Path release\windows-x64\* -DestinationPath release\ssl-windows-x64.zip

# Calculate checksum
CertUtil -hashfile release\ssl-windows-x64.zip SHA256
```

**Linux x64 (via WSL or Linux):**
```bash
# Build
pyinstaller --onefile --name ssl ssl-v7/ssl.py

# Package
mkdir -p release/linux-x64
cp dist/ssl release/linux-x64/
cp ssl-v7/README.txt release/linux-x64/
cp LICENSE release/linux-x64/
tar -czf release/ssl-linux-x64.tar.gz -C release/linux-x64 .

# Checksum
shasum -a 256 release/ssl-linux-x64.tar.gz
```

**macOS (on macOS):**
```bash
# Intel
pyinstaller --onefile --name ssl --target-arch x86_64 ssl-v7/ssl.py
mkdir -p release/macos-intel
cp dist/ssl release/macos-intel/
cp ssl-v7/README.txt release/macos-intel/
cp LICENSE release/macos-intel/
tar -czf release/ssl-macos-intel.tar.gz -C release/macos-intel .

# Apple Silicon
pyinstaller --onefile --name ssl --target-arch arm64 ssl-v7/ssl.py
mkdir -p release/macos-arm64
cp dist/ssl release/macos-arm64/
cp ssl-v7/README.txt release/macos-arm64/
cp LICENSE release/macos-arm64/
tar -czf release/ssl-macos-arm64.tar.gz -C release/macos-arm64 .
```

### Automated Build (All Platforms)

**build-release.ps1** (Windows):
```powershell
# Clean previous builds
Remove-Item -Recurse -Force dist, build, release -ErrorAction SilentlyContinue

# Build Windows binary
python -m PyInstaller --onefile --name ssl ssl-v7\ssl.py

# Create release directory
New-Item -ItemType Directory -Force release\windows-x64

# Copy files
Copy-Item dist\ssl.exe release\windows-x64\
Copy-Item ssl-v7\README.txt release\windows-x64\
Copy-Item LICENSE release\windows-x64\

# Create ZIP
Compress-Archive -Path release\windows-x64\* -DestinationPath release\ssl-windows-x64.zip -Force

# Calculate checksum
$hash = (CertUtil -hashfile release\ssl-windows-x64.zip SHA256 | Select-String -Pattern "^[0-9a-fA-F]{64}$").Matches.Value
Write-Host "SHA256: $hash"
$hash | Out-File release\ssl-windows-x64.zip.sha256

Write-Host "`nWindows x64 package created: release\ssl-windows-x64.zip"
Write-Host "Size: $((Get-Item release\ssl-windows-x64.zip).Length / 1MB) MB"
```

**build-release.sh** (Linux/macOS):
```bash
#!/bin/bash
set -e

# Clean
rm -rf dist build release

# Build
pyinstaller --onefile --name ssl ssl-v7/ssl.py

# Detect platform
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux-x64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ $(uname -m) == "arm64" ]]; then
        PLATFORM="macos-arm64"
    else
        PLATFORM="macos-intel"
    fi
fi

# Package
mkdir -p release/$PLATFORM
cp dist/ssl release/$PLATFORM/
cp ssl-v7/README.txt release/$PLATFORM/
cp LICENSE release/$PLATFORM/

# Create tarball
cd release
tar -czf ssl-$PLATFORM.tar.gz $PLATFORM
cd ..

# Checksum
shasum -a 256 release/ssl-$PLATFORM.tar.gz > release/ssl-$PLATFORM.tar.gz.sha256

echo ""
echo "$PLATFORM package created: release/ssl-$PLATFORM.tar.gz"
echo "Size: $(du -h release/ssl-$PLATFORM.tar.gz | cut -f1)"
echo "SHA256: $(cat release/ssl-$PLATFORM.tar.gz.sha256)"
```

### Build All (CI/CD)

**GitHub Actions** (.github/workflows/release.yml):
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - run: pip install pyinstaller
      - run: .\build-release.ps1
      - uses: actions/upload-artifact@v3
        with:
          name: windows-x64
          path: release/ssl-windows-x64.zip

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - run: pip install pyinstaller
      - run: chmod +x build-release.sh && ./build-release.sh
      - uses: actions/upload-artifact@v3
        with:
          name: linux-x64
          path: release/ssl-linux-x64.tar.gz

  build-macos:
    runs-on: macos-latest
    strategy:
      matrix:
        arch: [intel, arm64]
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - run: pip install pyinstaller
      - run: chmod +x build-release.sh && ./build-release.sh
      - uses: actions/upload-artifact@v3
        with:
          name: macos-${{ matrix.arch }}
          path: release/ssl-macos-*.tar.gz

  create-release:
    needs: [build-windows, build-linux, build-macos]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
      - uses: softprops/action-gh-release@v1
        with:
          files: |
            windows-x64/ssl-windows-x64.zip
            linux-x64/ssl-linux-x64.tar.gz
            macos-intel/ssl-macos-intel.tar.gz
            macos-arm64/ssl-macos-arm64.tar.gz
```

### Manual Release Steps

1. **Build** for your platform
2. **Test** the executable
3. **Upload** to GitHub Releases
4. **Update** INSTALL.md with correct checksums
5. **Announce** on social media

---

**Ready to build? Run the appropriate script for your platform! 🚀**
