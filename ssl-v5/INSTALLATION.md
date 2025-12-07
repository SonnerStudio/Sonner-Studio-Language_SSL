# SSL v5.0 Installation

## Systemvoraussetzungen

- Betriebssystem: Windows 10/11, macOS 12+, Linux (Ubuntu 20.04+)
- Arbeitsspeicher: Mindestens 4 GB RAM
- Festplattenspeicher: 500 MB für vollständige Installation
- Optional: GPU für beschleunigte Berechnungen

## Installation

### Windows

1. Laden Sie die neueste Version von der [Releases-Seite](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/releases) herunter
2. Führen Sie den Installer `ssl-v5.0.0-windows-x64.msi` aus
3. Folgen Sie den Anweisungen des Installationsassistenten
4. Fügen Sie SSL zum PATH hinzu (wird automatisch angeboten)

### macOS

```bash
# Mit Homebrew (empfohlen)
brew tap sonnerstudio/ssl
brew install ssl

# Oder manuell
curl -fsSL https://ssl-lang.dev/install.sh | sh
```

### Linux

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install ssl

# Fedora
sudo dnf install ssl

# Oder mit Snap
sudo snap install ssl

# Oder manuell
curl -fsSL https://ssl-lang.dev/install.sh | sh
```

## Überprüfung der Installation

Nach der Installation können Sie die Funktionalität testen:

```bash
ssl --version
# Ausgabe: SSL v5.0.0 - Self-Hosting Edition

ssl doctor
# Führt Systemdiagnose durch
```

## Erste Schritte

### Hello World

Erstellen Sie eine Datei `hello.ssl`:

```ssl
fn main() {
    println("Hello, SSL v5.0!")
}
```

Ausführen:

```bash
ssl run hello.ssl
```

### Interaktive REPL

```bash
ssl repl
```

## Editor-Integration

### Visual Studio Code

1. Öffnen Sie VS Code
2. Gehen Sie zu Extensions (Ctrl+Shift+X)
3. Suchen Sie nach "SSL Language"
4. Installieren Sie die Erweiterung

### Vim/Neovim

```vim
" In .vimrc oder init.vim
Plug 'sonnerstudio/ssl.vim'
```

### Emacs

```elisp
(use-package ssl-mode
  :ensure t)
```

## Konfiguration

Die Projektkonfiguration erfolgt über `ssl.toml`:

```toml
[package]
name = "mein-projekt"
version = "1.0.0"
edition = "2024"

[dependencies]
# Abhängigkeiten hier

[build]
target = "bytecode"
optimization = "release"
```

## Deinstallation

### Windows
- Systemsteuerung → Programme → SSL deinstallieren

### macOS
```bash
brew uninstall ssl
```

### Linux
```bash
sudo apt-get remove ssl
# oder
sudo snap remove ssl
```

## Hilfe und Support

- Dokumentation: https://ssl-lang.dev/docs
- GitHub Issues: https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/issues
- E-Mail: support@sonnerstudio.de

---

© 2024 SonnerStudio GmbH
