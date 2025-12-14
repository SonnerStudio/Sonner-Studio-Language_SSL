# SSL v8.0 - Installation Guide

**🌐 Diese Anleitung auf Deutsch:** [INSTALLATION_DE.md](INSTALLATION_DE.md)

---

## Quick Install (Recommended)

### Unix/Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.sh | sh
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.ps1 | iex
```

---

## Package Managers

### Scoop (Windows)

1. **Add the SSL bucket:**
   ```powershell
   scoop bucket add sonnerstudio https://github.com/SonnerStudio/scoop-ssl
   ```

2. **Install SSL:**
   ```powershell
   scoop install ssl
   ```

3. **Verify installation:**
   ```powershell
   ssl --version
   ```

### Homebrew (macOS/Linux)

1. **Tap the SSL repository:**
   ```bash
   brew tap sonnerstudio/ssl
   ```

2. **Install SSL:**
   ```bash
   brew install ssl
   ```

3. **Verify installation:**
   ```bash
   ssl --version
   ```

---

## Manual Installation

### 1. Download Binary

Download the appropriate binary for your platform from [GitHub Releases](https://github.com/SonnerStudio/SSL-v8/releases/latest):

|  Platform | Binary | Architecture |
|---------|--------|--------------|
| **Windows** | `ssl-windows-x64.exe` | x86_64 |
| **Linux** | `ssl-linux-x64` | x86_64 |
| **macOS (Intel)** | `ssl-macos-x64` | x86_64 |
| **macOS (Apple Silicon)** | `ssl-macos-arm64` | ARM64 |

### 2. Make Executable (Unix/Linux/macOS only)

```bash
chmod +x ssl-*
```

### 3. Add to PATH

**Linux/macOS:**
```bash
sudo mv ssl-* /usr/local/bin/ssl
```

**Windows:**
1. Move `ssl-windows-x64.exe` to `C:\Program Files\SSL\ssl.exe`
2. Add `C:\Program Files\SSL` to your PATH environment variable

### 4. Verify Installation

```bash
ssl --version
# Expected output: SSL v8.0.0 - The Ultimate Programming Language
```

---

## System Requirements

### Minimum Requirements
- **OS**: Windows 10+, Linux (Kernel 3.10+), macOS 10.13+
- **RAM**: 512 MB
- **Disk**: 50 MB

### Recommended
- **OS**: Windows 11, Ubuntu 22.04+, macOS 13+
- **RAM**: 2 GB
- **Disk**: 200 MB

---

## Usage

### Compile a Program
```bash
ssl compile hello.ssl
```

### Run Directly
```bash
ssl run hello.ssl
```

### Interactive REPL
```bash
ssl repl
```

### View Help
```bash
ssl --help
```

---

## Troubleshooting

### "ssl: command not found"

**Solution:** Ensure SSL is in your PATH.

**Linux/macOS:**
```bash
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows:**
Add the SSL installation directory to your PATH via System Properties → Environment Variables.

### Permission Denied (Unix/Linux/macOS)

**Solution:** Make the binary executable:
```bash
chmod +x /usr/local/bin/ssl
```

### Windows SmartScreen Warning

**Solution:** Click "More info" → "Run anyway". SSL binaries are not currently code-signed.

---

## Updating

### Scoop
```powershell
scoop update ssl
```

### Homebrew
```bash
brew upgrade ssl
```

### Manual
Download the latest release and replace the existing binary.

---

## Uninstalling

### Scoop
```powershell
scoop uninstall ssl
```

### Homebrew
```bash
brew uninstall ssl
```

### Manual

**Linux/macOS:**
```bash
sudo rm /usr/local/bin/ssl
```

**Windows:**
Delete `C:\Program Files\SSL\` and remove from PATH.

---

## Next Steps

- 📚 [Complete Feature List](FEATURES.md)
- 📖 [Main README](README.md)
- 📝 [Change Log](CHANGELOG.md)

---

**Questions or issues?** [Open an issue on GitHub](https://github.com/SonnerStudio/SSL-v8/issues)
