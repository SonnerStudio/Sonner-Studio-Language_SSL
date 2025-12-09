# SSL v7.0 Installation Guide

## 📥 Binary Download (Recommended)

SSL v7.0 is distributed as **pre-compiled binaries** for maximum security and performance.

### Windows Installation

**Step 1: Download**
```powershell
# Download latest release
Invoke-WebRequest -Uri "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-windows-x64.zip" -OutFile "ssl.zip"

# Extract
Expand-Archive ssl.zip -DestinationPath "$env:LOCALAPPDATA\SSL"

# Add to PATH
$env:PATH += ";$env:LOCALAPPDATA\SSL"
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, "User")
```

**Step 2: Verify**
```powershell
ssl --version
# Output: SSL v7.0.0 - Native Compilation Edition
```

### Linux/macOS Installation

**Step 1: Download**
```bash
# Linux x64
curl -L "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-linux-x64.tar.gz" -o ssl.tar.gz

# macOS (Intel)
curl -L "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-macos-intel.tar.gz" -o ssl.tar.gz

# macOS (Apple Silicon)
curl -L "https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-macos-arm64.tar.gz" -o ssl.tar.gz
```

**Step 2: Install**
```bash
# Extract
tar -xzf ssl.tar.gz

# Move to /usr/local/bin
sudo mv ssl /usr/local/bin/
sudo chmod +x /usr/local/bin/ssl
```

**Step 3: Verify**
```bash
ssl --version
```

## 🚀 Quick Start

### Your First Program

Create `hello.ssl`:
```ssl
fn main() -> Int {
    print("Hello, SSL v7.0!")
    return 0
}
```

### Compile and Run

```bash
# Compile
ssl compile hello.ssl

# Output: hello.exe (Windows) or hello (Linux/macOS)

# Run
./hello
# Output: Hello, SSL v7.0!
```

## 📦 Available Downloads

### Current Release: v7.0.0

| Platform | Architecture | Download Size | Link |
|----------|--------------|---------------|------|
| Windows | x64 | ~15 MB | [ssl-windows-x64.zip](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-windows-x64.zip) |
| Linux | x64 | ~12 MB | [ssl-linux-x64.tar.gz](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-linux-x64.tar.gz) |
| Linux | ARM64 | ~12 MB | [ssl-linux-arm64.tar.gz](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-linux-arm64.tar.gz) |
| macOS | Intel | ~13 MB | [ssl-macos-intel.tar.gz](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-macos-intel.tar.gz) |
| macOS | Apple Silicon | ~13 MB | [ssl-macos-arm64.tar.gz](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases/download/v7.0.0/ssl-macos-arm64.tar.gz) |

## 🔧 System Requirements

**Minimum:**
- Windows 10/11, Linux (kernel 4.0+), macOS 10.15+
- 4 GB RAM
- 100 MB disk space

**Recommended:**
- 8 GB RAM
- SSD storage
- Multi-core CPU for parallel compilation

## 🤖 Ollama Integration (Optional)

For AI-assisted development:

```bash
# Install Ollama
winget install Ollama.Ollama  # Windows
# or
brew install ollama           # macOS

# Download AI models
ollama pull codellama
ollama pull mistral

# Use with SSL
ssl compile --ai-assist mycode.ssl
```

## 📖 Documentation

After installation, access docs at:
- Local: `ssl docs`
- Online: [https://docs.ssl-lang.dev](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/tree/main/docs)

## 🆘 Troubleshooting

### "ssl: command not found"

**Windows:**
```powershell
# Check PATH
echo $env:PATH

# Re-add SSL directory
$env:PATH += ";$env:LOCALAPPDATA\SSL"
```

**Linux/macOS:**
```bash
# Check PATH
echo $PATH

# Re-add if needed
export PATH="/usr/local/bin:$PATH"
```

### Permission Denied (Linux/macOS)

```bash
sudo chmod +x /usr/local/bin/ssl
```

### Antivirus Blocking (Windows)

Add SSL directory to Windows Defender exclusions:
```powershell
Add-MpPreference -ExclusionPath "$env:LOCALAPPDATA\SSL"
```

## 🔄 Updating

```bash
# Check current version
ssl --version

# Download latest release and repeat installation steps
```

## 🗑️ Uninstallation

**Windows:**
```powershell
Remove-Item -Recurse "$env:LOCALAPPDATA\SSL"
# Remove from PATH manually via System Settings
```

**Linux/macOS:**
```bash
sudo rm /usr/local/bin/ssl
```

## 📜 License

SSL v7.0 is released under the Apache 2.0 License.
Binaries are provided as-is for evaluation and production use.

---

**Need Help?** 
- 📖 [Documentation](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/tree/main/docs)
- 💬 [Discussions](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/discussions)
- 🐛 [Issues](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/issues)

**Welcome to SSL v7.0! 🚀**
