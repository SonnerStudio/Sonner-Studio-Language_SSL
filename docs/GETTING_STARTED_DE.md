# Erste Schritte mit SSL v7.0

Willkommen bei Sonner Studio Language v7.0! Dieser Leitfaden hilft Ihnen, mit den nativen Kompilierungs-Features von SSL zu starten.

## Inhaltsverzeichnis

1. [Installation](#installation)
2. [Ihr erstes Programm](#ihr-erstes-programm)
3 [Kompilieren und Ausführen](#kompilieren-und-ausführen)
4. [Sprach-Grundlagen](#sprach-grundlagen)
5. [Nächste Schritte](#nächste-schritte)

## Installation

### Voraussetzungen

**Windows:**
```powershell
# Rust installieren (für Bootstrapping)
winget install Rustlang.Rustup

# NASM installieren
winget install NASM.NASM

# Visual Studio Build Tools installieren (für Linker)
# Von Microsoft-Website herunterladen
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install build-essential nasm

# Für ARM64 Cross-Compilation
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

**macOS:**
```bash
# Xcode Command Line Tools installieren
xcode-select --install

# NASM via Homebrew installieren
brew install nasm
```

### SSL v7.0 installieren

```bash
# Repository klonen
git clone https://github.com/SonnerStudio/SSL.git
cd SSL

# Build und Installation
cargo install --path .

# Installation verifizieren
ssl --version
# Ausgabe: SSL v7.0.0
```

## Ihr erstes Programm

Erstellen Sie eine Datei namens `hallo.ssl`:

```ssl
fn main() -> Int {
    print("Hallo, SSL v7.0!")
    return 0
}
```

**Erklärung:**
- `fn main() -> Int` - Jedes SSL-Programm startet mit einer `main`-Funktion, die ein Int zurückgibt
- `print(...)` - Eingebaute Funktion für Ausgabe
- `return 0` - Exit-Code (0 = Erfolg)

## Kompilieren und Ausführen

### Schnell Ausführen (Interpretieren)

```bash
ssl run hallo.ssl
```

Ausgabe:
```
Hallo, SSL v7.0!
```

### Zu Nativ kompilieren

```bash
# Zu Assembly kompilieren
ssl compile hallo.ssl

# Dies erzeugt:
# - hallo.asm (NASM Assembly)
# - hallo.o (Object-Datei)
# - hallo.exe/hallo (Ausführbare Datei)

# Kompiliertes Programm ausführen
./hallo
```

### Multi-Architektur-Kompilierung

```bash
# Für ARM64 kompilieren
ssl compile --arch arm64 hallo.ssl

# Für Apple Silicon kompilieren
ssl compile --arch apple_m hallo.ssl

# Für spezifische Plattformen kompilieren
ssl compile --target x86_64-linux hallo.ssl
```

## Sprach-Grundlagen

### Variablen

```ssl
fn main() -> Int {
    let x = 42              // Unveränderlich
    let mut y = 10          // Veränderlich
    
    y = y + 5
    print(int_to_string(y)) // Ausgabe: 15
    
    return 0
}
```

### Funktionen

```ssl
fn addiere(a: Int, b: Int) -> Int {
    return a + b
}

fn main() -> Int {
    let ergebnis = addiere(5, 3)
    print(int_to_string(ergebnis)) // Ausgabe: 8
    return 0
}
```

### Kontrollfluss

```ssl
fn main() -> Int {
    let x = 10
    
    // If-else
    if x > 5 {
        print("Größer als 5")
    } else {
        print("Kleiner oder gleich 5")
    }
    
    // While-Schleife
    let mut i = 0
    while i < 5 {
        print(int_to_string(i))
        i = i + 1
    }
    
    return 0
}
```

### Typen

SSL v7.0 unterstützt folgende Basis-Typen:

- `Int` - 64-Bit signed Integer
- `Float` - 64-Bit Floating Point
- `String` - UTF-8 String
- `Bool` - Boolean (true/false)

## Interaktive REPL

SSL v7.0 bietet eine interaktive Read-Eval-Print-Schleife:

```bash
ssl repl
```

```
SSL v7.0 REPL
Type 'exit' to quit

> let x = 42
> x + 10
52
> fn verdoppeln(n: Int) -> Int { return n * 2 }
> verdoppeln(21)
42
```

## Projekt-Struktur

Für größere Projekte organisieren Sie Ihren Code:

```
mein_projekt/
├── src/
│   ├── main.ssl          # Einstiegspunkt
│   ├── utils.ssl         # Hilfsfunktionen
│   └── types.ssl         # Typ-Definitionen
├── tests/
│   └── test_main.ssl     # Tests
└── ssl.toml              # Projekt-Konfiguration
```

## Debugging

```bash
# Mit Debug-Symbolen kompilieren
ssl compile --debug hallo.ssl

# Generiertes Assembly ansehen
ssl compile --emit asm hallo.ssl
cat hallo.asm

# Syntax prüfen ohne Kompilierung
ssl check hallo.ssl
```

## Häufige Fehler

### 1. Typ-Inkompatibilität
```ssl
fn main() -> Int {
    return "string"  // FEHLER: Int erwartet, String erhalten
}
```

**Lösung**: Sicherstellen, dass Rückgabetyp übereinstimmt.

### 2. Undefinierte Funktion
```ssl
fn main() -> Int {
    foo()  // FEHLER: Funktion 'foo' nicht gefunden
    return 0
}
```

**Lösung**: Funktion definieren oder Schreibweise prüfen.

## Nächste Schritte

- **Sprachreferenz**: Erweiterte Syntax in [SPRACHREFERENZ_DE.md](SPRACHREFERENZ_DE.md)
- **Compiler-Handbuch**: Kompilierung verstehen in [COMPILER_HANDBUCH_DE.md](COMPILER_HANDBUCH_DE.md)
- **Beispiele**: Mehr Beispiele in [examples/](../examples/)
- **Betriebssystem-Entwicklung**: OS-Kernel bauen in [OS_DEV_DE.md](OS_DEV_DE.md)

## Hilfe erhalten

- **Dokumentation**: [docs/](../docs/)
- **Discord**: [Community beitreten](https://discord.gg/sonnerstudio)
- **GitHub Issues**: [Bugs melden](https://github.com/SonnerStudio/SSL/issues)

---

**Willkommen in der SSL-Community!** 🚀

Weiter: [Sprachreferenz →](SPRACHREFERENZ_DE.md)
