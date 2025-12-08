# SSL v7.0 Binary Release Guide

This document explains how to create a binary-only release of SSL v7.0 without exposing the compiler source code.

## Release Structure

```
SSL-v7.0-Release/
├── bin/                    # Compiled binaries
│   ├── ssl.exe            # Windows compiler
│   ├── ssl                # Linux/macOS compiler
│   └── ssl_runtime.lib    # Runtime library
├── docs/                  # Documentation (all .md files)
├── examples/              # Example programs
├── LICENSE               # Apache 2.0 license
├── README.md             # Main documentation
├── README_DE.md          # German documentation
└── CHANGELOG.md          # Version history
```

## What to Include

### ✅ Include (Public)

- **Binaries**: Pre-compiled `ssl.exe` / `ssl`
- **Runtime Library**: `ssl_runtime.lib`
- **Documentation**: All `.md` files from `docs/`
- **Examples**: All `.ssl` example programs
- **License**: LICENSE file
- **Logo**: SSL v7 logo assets

### ❌ Exclude (Private)

- **Source Code**: `ssl-v7/src/*.ssl` (compiler implementation)
- **Internal Tests**: Development test files
- **Build Scripts**: Internal build configurations
- **Development Tools**: Debug/profiling tools

## Creating the Release

### Step 1: Build Binaries

```bash
# Build release versions
cargo build --release

# The binary will be in target/release/ssl
```

### Step 2: Prepare Release Directory

```powershell
# Create release structure
mkdir SSL-v7.0-Release
mkdir SSL-v7.0-Release\bin
mkdir SSL-v7.0-Release\docs
mkdir SSL-v7.0-Release\examples

# Copy binaries
copy target\release\ssl.exe SSL-v7.0-Release\bin\
copy runtime\ssl_runtime.lib SSL-v7.0-Release\bin\

# Copy documentation
copy docs\*.md SSL-v7.0-Release\docs\
copy README.md SSL-v7.0-Release\
copy README_DE.md SSL-v7.0-Release\
copy CHANGELOG.md SSL-v7.0-Release\
copy LICENSE SSL-v7.0-Release\

# Copy examples
xcopy examples SSL-v7.0-Release\examples\ /E /I

# Copy logo
copy assets\ssl-v7-logo.svg SSL-v7.0-Release\
```

### Step 3: Create .gitignore for Source Protection

```gitignore
# SSL v7.0 - Protect Source Code

# Compiler source code (PRIVATE)
ssl-v7/src/
ssl-v7/tests/
ssl-v5/src/
ssl-v6/src/

# Development files
*.log
*.tmp
build/
target/
.vscode/
.idea/

# Keep only public files
!bin/
!docs/
!examples/
!README*.md
!LICENSE
!CHANGELOG.md
!assets/
```

### Step 4: Package Release

```bash
# Create ZIP archive
cd SSL-v7.0-Release
zip -r SSL-v7.0.0-windows-x64.zip .

# Or for cross-platform
tar -czf SSL-v7.0.0-linux-x64.tar.gz .
```

## GitHub Release Configuration

### .gitignore (Root)

```gitignore
# SSL Compiler Source Code - PROPRIETARY
/ssl-v7/src/
/ssl-v7/tests/
/ssl-v5/
/ssl-v6/
/src/lib.rs
/src/parser.rs
/src/lexer.rs
/src/compiler/

# Keep public interface only
!/src/main.rs
!/src/cli.rs

# Development artifacts
*.o
*.asm
*.log
build/
target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp

# Public files (explicitly allowed)
!README.md
!README_DE.md
!CHANGELOG.md
!LICENSE
!LICENSE-APACHE
!docs/
!examples/
!assets/
!bin/
```

### Repository Structure

```
SSL/ (GitHub Repository)
├── bin/
│   ├── ssl.exe              # ✅ Pre-compiled binary (public)
│   └── ssl_runtime.lib      # ✅ Runtime library (public)
├── docs/                    # ✅ All documentation (public)
├── examples/                # ✅ Example programs (public)
├── assets/                  # ✅ Logo and branding (public)
├── LICENSE                  # ✅ Apache 2.0 (public)
├── README.md               # ✅ Documentation (public)
├── README_DE.md            # ✅ German docs (public)
├── CHANGELOG.md            # ✅ Version history (public)
├── CONTRIBUTING.md         # ✅ Community guidelines (public)
├── CODE_OF_CONDUCT.md      # ✅ Code of conduct (public)
├── .gitignore              # ✅ Protects source code
│
├── ssl-v7/                 # ❌ NOT IN GIT (source code)
│   └── src/                # ❌ PRIVATE - compiler implementation
├── ssl-v5/                 # ❌ NOT IN GIT (old versions)
└── ssl-v6/                 # ❌ NOT IN GIT (old versions)
```

## Release Checklist

- [ ] Compile release binaries (`cargo build --release`)
- [ ] Test binaries on target platforms
- [ ] Copy binaries to `bin/`
- [ ] Verify all documentation is current
- [ ] Check `.gitignore` excludes source code
- [ ] Create release ZIP/tarball
- [ ] Test binary-only release (delete source, test compile)
- [ ] Tag release on GitHub (`v7.0.0`)
- [ ] Upload release binaries to GitHub Releases
- [ ] Update website/announcements

## Binary-Only Testing

Before publishing, verify the release works without source:

```bash
# Delete source code locally
rm -rf ssl-v7/src
rm -rf ssl-v5 ssl-v6

# Test that binaries still work
./bin/ssl --version
./bin/ssl compile examples/basics/hello.ssl
./hello
```

## Platform-Specific Binaries

### Windows (x64)

```
SSL-v7.0.0-windows-x64.zip
├── ssl.exe
└── ssl_runtime.lib
```

### Linux (x64)

```
SSL-v7.0.0-linux-x64.tar.gz
├── ssl
└── libssl_runtime.a
```

### macOS (Universal)

```
SSL-v7.0.0-macos-universal.tar.gz
├── ssl
└── libssl_runtime.a
```

### ARM64 (Linux)

```
SSL-v7.0.0-linux-arm64.tar.gz
├── ssl
└── libssl_runtime.a
```

## GitHub Release Page

### Title
```
SSL v7.0.0 - Native Compilation Edition
```

### Description
```markdown
## SSL v7.0.0 - Production Ready

Native x64 Compilation • Multi-Architecture • NLP Support

### Downloads

- **Windows (x64)**: SSL-v7.0.0-windows-x64.zip
- **Linux (x64)**: SSL-v7.0.0-linux-x64.tar.gz  
- **macOS (Universal)**: SSL-v7.0.0-macos-universal.tar.gz
- **Linux (ARM64)**: SSL-v7.0.0-linux-arm64.tar.gz

### What's New

- Native x64 assembly generation
- Multi-architecture support (x86_64, ARM64, Apple Silicon)
- Natural Language Programming in 9+ languages
- Production-grade compiler pipeline

See [CHANGELOG.md](CHANGELOG.md) for full details.

### Installation

1. Download the archive for your platform
2. Extract to desired location
3. Add `bin/` to your PATH
4. Run `ssl --version` to verify

### Documentation

- [Getting Started](docs/GETTING_STARTED.md)
- [Language Reference](docs/LANGUAGE_REFERENCE.md)
- [NLP Guide](docs/NLP_GUIDE.md)
- [Compiler Guide](docs/COMPILER_GUIDE.md)
```

---

**Questions?**
- Email: releases@sonnerstudio.com
- Discord: https://discord.gg/sonnerstudio
