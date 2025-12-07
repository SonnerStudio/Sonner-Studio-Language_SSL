# SSL v5.0 Installation

## Systemvoraussetzungen

- Betriebssystem: Windows 10/11, macOS 12+, Linux (Ubuntu 20.04+)
- Rust Toolchain (für Kompilierung): https://rustup.rs/
- Arbeitsspeicher: Mindestens 4 GB RAM
- Festplattenspeicher: 500 MB für vollständige Installation

## Installation mit Cargo (Empfohlen)

SSL wird mit Cargo, dem Rust-Paketmanager, installiert:

### 1. Repository klonen

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
```

### 2. SSL kompilieren und installieren

```bash
# Release-Build erstellen
cargo build --release

# SSL global installieren
cargo install --path .
```

### 3. Installation überprüfen

```bash
ssl --version
# Ausgabe: SSL v5.0.0 - Self-Hosting Edition

ssl doctor
# Führt Systemdiagnose durch
```

## Alternative: Direkt mit Cargo Run

Sie können SSL auch direkt ohne Installation ausführen:

```bash
# SSL-Programm ausführen
cargo run -- run mein_programm.ssl

# SSL REPL starten
cargo run -- repl

# SSL-Datei kompilieren
cargo run -- build mein_programm.ssl
```

## Plattform-spezifische Hinweise

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
