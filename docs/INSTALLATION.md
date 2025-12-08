# SSL v7.0 Installation Guide

## Installation Methods

### Method 1: Cargo Install (Recommended) ⭐

The easiest and fastest way to install SSL v7.0:

```bash
# Install directly from GitHub
cargo install --git https://github.com/SonnerStudio/SSL

# Verify installation
ssl --version
# Output: SSL v7.0.0 - Native Compilation Edition
```

**Advantages:**
- ✅ Automatic dependency management
- ✅ Easy updates (`cargo install --git ... --force`)
- ✅ Cross-platform support
- ✅ No manual PATH configuration needed

### Method 2: Install from Source

For development or customization:

```bash
# Clone repository
git clone https://github.com/SonnerStudio/SSL.git
cd SSL

# Build and install
cargo install --path .

# Or build without installing
cargo build --release
# Binary will be in target/release/ssl
```

### Method 3: Pre-compiled Binaries

Download pre-built binaries from [GitHub Releases](https://github.com/SonnerStudio/SSL/releases):

**Windows (x64):**
```powershell
# Download SSL-v7.0.0-windows-x64.zip
# Extract to desired location
# Add bin/ to PATH
```

**Linux (x64):**
```bash
# Download SSL-v7.0.0-linux-x64.tar.gz
tar -xzf SSL-v7.0.0-linux-x64.tar.gz
sudo mv ssl /usr/local/bin/
```

**macOS (Universal):**
```bash
# Download SSL-v7.0.0-macos-universal.tar.gz
tar -xzf SSL-v7.0.0-macos-universal.tar.gz
sudo mv ssl /usr/local/bin/
```

---

## Prerequisites

### All Platforms

- **Rust**: 1.70 or later (for cargo install)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Windows

- **NASM**: Assembler
  ```powershell
  winget install NASM.NASM
  ```

- **Visual Studio Build Tools**: C++ compiler and linker
  - Download from [Microsoft](https://visualstudio.microsoft.com/downloads/)
  - Install "Desktop development with C++"

### Linux

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential nasm

# Fedora/RHEL
sudo dnf install gcc nasm

# Arch
sudo pacman -S base-devel nasm
```

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install NASM via Homebrew
brew install nasm
```

---

## Verification

After installation, verify SSL is working:

```bash
# Check version
ssl --version

# Try compiling a simple program
echo 'fn main() -> Int { return 0 }' > test.ssl
ssl compile test.ssl
./test

# Run interactive REPL
ssl repl
```

---

## Updating

### Cargo Install

```bash
cargo install --git https://github.com/SonnerStudio/SSL --force
```

### From Source

```bash
cd SSL
git pull
cargo install --path . --force
```

---

## Uninstalling

### Cargo Install

```bash
cargo uninstall ssl
```

### Manual

```bash
# Linux/macOS
sudo rm /usr/local/bin/ssl

# Windows
# Remove from installation directory
# Remove from PATH
```

---

## Troubleshooting

### "cargo: command not found"

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "NASM not found"

Install NASM for your platform (see Prerequisites above).

### "Linker error"

**Windows**: Install Visual Studio Build Tools  
**Linux**: `sudo apt install build-essential`  
**macOS**: `xcode-select --install`

### Permission denied

**Linux/macOS**:
```bash
chmod +x /path/to/ssl
```

---

## Next Steps

- [Getting Started Guide](docs/GETTING_STARTED.md)
- [Language Reference](docs/LANGUAGE_REFERENCE.md)
- [Examples](examples/)

---

**Questions?**
- Discord: https://discord.gg/sonnerstudio
- GitHub Issues: https://github.com/SonnerStudio/SSL/issues
