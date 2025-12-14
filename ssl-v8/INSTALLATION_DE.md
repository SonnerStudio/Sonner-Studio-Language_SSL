# SSL v8.0 - Installations-Anleitung

**🌐 This guide in English:** [INSTALLATION.md](INSTALLATION.md)

---

## Schnellinstallation (Empfohlen)

### Unix/Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.sh | sh
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/SonnerStudio/SSL-v8/main/scripts/install.ps1 | iex
```

---

## Package Manager

### Scoop (Windows)

1. **SSL-Bucket hinzufügen:**
   ```powershell
   scoop bucket add sonnerstudio https://github.com/SonnerStudio/scoop-ssl
   ```

2. **SSL installieren:**
   ```powershell
   scoop install ssl
   ```

3. **Installation überprüfen:**
   ```powershell
   ssl --version
   ```

### Homebrew (macOS/Linux)

1. **SSL-Repository hinzufügen:**
   ```bash
   brew tap sonnerstudio/ssl
   ```

2. **SSL installieren:**
   ```bash
   brew install ssl
   ```

3. **Installation überprüfen:**
   ```bash
   ssl --version
   ```

---

## Manuelle Installation

### 1. Binary herunterladen

Laden Sie die entsprechende Binary für Ihre Plattform von [GitHub Releases](https://github.com/SonnerStudio/SSL-v8/releases/latest) herunter:

| Plattform | Binary | Architektur |
|-----------|--------|-------------|
| **Windows** | `ssl-windows-x64.exe` | x86_64 |
| **Linux** | `ssl-linux-x64` | x86_64 |
| **macOS (Intel)** | `ssl-macos-x64` | x86_64 |
| **macOS (Apple Silicon)** | `ssl-macos-arm64` | ARM64 |

### 2. Ausführbar machen (nur Unix/Linux/macOS)

```bash
chmod +x ssl-*
```

### 3. Zum PATH hinzufügen

**Linux/macOS:**
```bash
sudo mv ssl-* /usr/local/bin/ssl
```

**Windows:**
1. Verschieben Sie `ssl-windows-x64.exe` nach `C:\Programme\SSL\ssl.exe`
2. Fügen Sie `C:\Programme\SSL` zu Ihrer PATH-Umgebungsvariable hinzu

### 4. Installation überprüfen

```bash
ssl --version
# Erwartete Ausgabe: SSL v8.0.0 - The Ultimate Programming Language
```

---

## Systemanforderungen

### Mindestanforderungen
- **OS**: Windows 10+, Linux (Kernel 3.10+), macOS 10.13+
- **RAM**: 512 MB
- **Festplatte**: 50 MB

### Empfohlen
- **OS**: Windows 11, Ubuntu 22.04+, macOS 13+
- **RAM**: 2 GB
- **Festplatte**: 200 MB

---

## Verwendung

### Programm kompilieren
```bash
ssl compile hello.ssl
```

### Direkt ausführen
```bash
ssl run hello.ssl
```

### Interaktive REPL
```bash
ssl repl
```

### Hilfe anzeigen
```bash
ssl --help
```

---

## Problemlösung

### "ssl: Befehl nicht gefunden"

**Lösung:** Stellen Sie sicher, dass SSL in Ihrem PATH ist.

**Linux/macOS:**
```bash
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows:**
Fügen Sie das SSL-Installationsverzeichnis über Systemeigenschaften → Umgebungsvariablen zum PATH hinzu.

### Zugriff verweigert (Unix/Linux/macOS)

**Lösung:** Binary ausführbar machen:
```bash
chmod +x /usr/local/bin/ssl
```

### Windows SmartScreen Warnung

**Lösung:** Klicken Sie auf "Weitere Informationen" → "Trotzdem ausführen". SSL-Binaries sind derzeit nicht code-signiert.

---

## Aktualisierung

### Scoop
```powershell
scoop update ssl
```

### Homebrew
```bash
brew upgrade ssl
```

### Manuell
Laden Sie die neueste Version herunter und ersetzen Sie die vorhandene Binary.

---

## Deinstallation

### Scoop
```powershell
scoop uninstall ssl
```

### Homebrew
```bash
brew uninstall ssl
```

### Manuell

**Linux/macOS:**
```bash
sudo rm /usr/local/bin/ssl
```

**Windows:**
Löschen Sie `C:\Programme\SSL\` und entfernen Sie es aus dem PATH.

---

## Nächste Schritte

- 📚 [Vollständige Feature-Liste](FEATURES_DE.md)
- 📖 [Haupt-README](README_DE.md)
- 📝 [Änderungsprotokoll](CHANGELOG.md)

---

**Fragen oder Probleme?** [Öffnen Sie ein Issue auf GitHub](https://github.com/SonnerStudio/SSL-v8/issues)
