# SSL v5.0 Installation

## Systemvoraussetzungen

- Betriebssystem: Windows 10/11 (x64)
- Arbeitsspeicher: Mindestens 4 GB RAM
- Festplattenspeicher: 50 MB für Installation

## Installation (Windows)

### 1. Binary herunterladen

Laden Sie die aktuelle Version aus dem `release/` Verzeichnis herunter:
- `ssl.exe` - SSL Compiler und Interpreter

### 2. In Pfad kopieren

Kopieren Sie `ssl.exe` in ein Verzeichnis Ihrer Wahl, z.B.:
```
C:\Program Files\SSL\ssl.exe
```

### 3. Umgebungsvariable setzen

Fügen Sie das Verzeichnis zur PATH-Umgebungsvariable hinzu:
1. Windows-Suche: "Umgebungsvariablen"
2. "Path" bearbeiten
3. Neuen Eintrag hinzufügen: `C:\Program Files\SSL`

### 4. Installation überprüfen

```bash
ssl --version
# Ausgabe: ssl 5.0.0

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

## Verfügbare Befehle

| Befehl | Beschreibung |
|--------|--------------|
| `ssl run <datei>` | Programm ausführen |
| `ssl repl` | Interaktive Shell |
| `ssl check <datei>` | Syntax prüfen |
| `ssl help` | Hilfe anzeigen |
| `ssl version` | Version anzeigen |
| `ssl doctor` | Systemdiagnose |

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
