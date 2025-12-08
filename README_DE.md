<div align="center">

![SSL v7 Logo](assets/ssl-v7-logo.svg)

# ⚡ Sonner Studio Language (SSL) v7.0

### *Native Compilation Edition - Produktionsreif*

[![Release](https://img.shields.io/badge/release-v7.0.0-blue.svg)](https://github.com/SonnerStudio/SSL/releases)
[![Lizenz](https://img.shields.io/badge/lizenz-Apache%202.0-green.svg)](LICENSE)
[![Plattform](https://img.shields.io/badge/plattform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()

---

### 🌍 Sprache wählen / Select Language

| 🇩🇪 🇦🇹 🇨🇭 **DEUTSCH** | 🇬🇧 🇺🇸 [**ENGLISH**](README.md) |
| :---: | :---: |
| 📖 [**Handbuch**](docs/MANUAL_DE.md) | 📖 [**Manual**](docs/MANUAL_EN.md) |
| *(Ausgewählt)* | *(Wechseln / Switch)* |

---

> **"Eine revolutionäre Programmiersprache mit nativer x64-Kompilierung, Multi-Architektur-Unterstützung und selbst-hostendem Compiler."**

`Produktionsreif` • `x64 Nativ` • `Multi-Architektur` • `Self-Hosted` • `Typ-Sicher`

</div>

---

## 🚀 Schnellstart

### Installation via Cargo (Empfohlen)

Der einfachste Weg zur Installation von SSL v7.0:

```bash
# Von GitHub installieren
cargo install --git https://github.com/SonnerStudio/SSL ssl

# Oder aus dem Quellcode installieren
git clone https://github.com/SonnerStudio/SSL.git
cd SSL
cargo install --path .

# Installation verifizieren
ssl --version
```

### Schnell-Beispiel

```bash
# SSL v7.0 installieren
git clone https://github.com/SonnerStudio/SSL.git
cd SSL

# Programm kompilieren
ssl compile hello.ssl

# Direkt ausführen
ssl run hello.ssl

# Interaktive REPL
ssl repl
```

## 📋 Inhaltsverzeichnis

- [Features](#-features)
- [Architektur-Unterstützung](#-multi-architektur-unterstützung)
- [Installation](#-installation)
- [Dokumentation](#-dokumentation)
- [Beispiele](#-beispiele)
- [Benchmarks](#-benchmarks)
- [Versionshistorie](#-versionshistorie)
- [Mitwirken](#-mitwirken)
- [Lizenz](#-lizenz)

## ✨ Features

### 🎯 Kernsprache (v1.0 - v7.0)

- **Statische Typisierung** - Hindley-Milner Typ-Inferenz
- **Pattern Matching** - Erschöpfende Match-Ausdrücke
- **First-Class Funktionen** - Closures, Higher-Order Functions
- **Generics & Traits** - Parametrische Polymorphie
- **Speichersicherheit** - Linear Types, Ownership-System
- **Nebenläufigkeit** - Async/await, Actors, Channels

### 🔥 SSL v7.0 Highlights

#### Native Kompilierung
- **Direkte x64 Assembly** - Kein VM, kein Interpreter
- **NASM/MASM Kompatibel** - Standard Assembly-Ausgabe
- **Multi-Architektur** - x86_64, ARM64, Apple Silicon
- **ELF64/Mach-O Support** - Linux, macOS, Windows

#### Produktions-Compiler-Pipeline
```
Quellcode → Lexer → Parser → IR Generator → Optimizer → Code Generator → Assembly
```

- **Lexer**: Vollständige Token-Unterstützung, Literale (Hex/Float), String-Escapes
- **Parser**: Recursive Descent + Pratt Expression Parsing
- **IR**: SSA-Form, Basic Blocks, Control Flow Graphs
- **Optimizer**: Konstantenfaltung, Eliminierung toter Code
- **Codegen**: x64 ABI, Stack Frames, Register-Allokation

#### Betriebssystem-Integration
- **ZetaTron-OS**: Native Kernel-Module in SSL
- **Bare-Metal Support**: Freestanding Umgebungen
- **HAL-Abstraktion**: Hardware Abstraction Layer
- **Cross-Compilation**: Build für mehrere Targets

### 🌟 Unique Features (Alle Versionen)

| Feature | Seit | Beschreibung |
|---------|------|--------------|
| **Self-Hosting** | v5.0 | Compiler in SSL geschrieben |
| **Time-Travel Debugging** | v5.0 | Omnisziente Fehlersuche |
| **Quantum Computing** | v5.0 | IBM Quantum, IonQ Integration |
| **Non-Rectangular GUI** | v5.0 | Beliebige Fensterformen |
| **Native Compilation** | v7.0 | Direkte x64 Assembly-Ausgabe |
| **Multi-Architektur** | v7.0 | x86_64, ARM64, Apple Silicon |
| **Natural Language Programming** | v7.0 | Code in 24+ Sprachen |

### 🗣️ Natural Language Programming (NLP)

**Revolutionäres Feature:** Programmieren Sie in Ihrer Muttersprache! SSL v7.0 unterstützt Natural Language Programming mit vollständiger Schlüsselwort-Übersetzung und Bezeichner-Support in 24+ Sprachvarianten.

#### Unterstützte Sprachen (9 Hauptsprachen + 15 Dialekte)

| Sprache | Code | Region | Schlüsselwörter | Beispiel |
|---------|------|--------|-----------------|----------|
| **Englisch** | `en` | 🇬🇧 🇺🇸 | ✅ Vollständig | `function`, `if`, `while` |
| **Deutsch** | `de` | 🇩🇪 🇦🇹 🇨🇭 | ✅ Vollständig | `funktion`, `wenn`, `solange` |
| **Französisch** | `fr` | 🇫🇷 | ✅ Vollständig | `fonction`, `si`, `pendant` |
| **Spanisch** | `es` | 🇪🇸 | ✅ Vollständig | `función`, `si`, `mientras` |
| **Portugiesisch** | `pt` | 🇵🇹 🇧🇷 | ✅ Vollständig | `função`, `se`, `enquanto` |
| **Hebräisch** | `he` | 🇮🇱 | ✅ Vollständig | `פונקציה`, `אם`, `כל עוד` |
| **Japanisch** | `ja` | 🇯🇵 | ✅ Vollständig | `関数`, `もし`, `間` |
| **Hindi** | `hi` | 🇮🇳 | ✅ Vollständig | `फ़ंक्शन`, `यदि`, `जबकि` |
| **Bengali** | `bn` | 🇧🇩 | ✅ Vollständig | `ফাংশন`, `যদি`, `যখন` |

#### Regionale Dialekte

**Deutsche Varianten:**
- 🇩🇪 Standarddeutsch (Hochdeutsch)
- 🇦🇹 Österreichisches Deutsch
- 🇨🇭 Schweizerdeutsch
- Norddeutsch, Mitteldeutsch, Ostdeutsch, Westdeutsch, Süddeutsch

**Englische Varianten:**
- 🇺🇸 Amerikanisches Englisch
- 🇬🇧 Britisches Englisch
- 🇦🇺 Australisches Englisch
- 🇨🇦 Kanadisches Englisch

**Spanische Varianten:**
- 🇪🇸 Kastilisches Spanisch
- 🇲🇽 Mexikanisches Spanisch
- 🇦🇷 Argentinisches Spanisch

**Portugiesische Varianten:**
- 🇵🇹 Europäisches Portugiesisch
- 🇧🇷 Brasilianisches Portugiesisch

#### Code-Beispiele in verschiedenen Sprachen

**English:**
```ssl
fn fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    return fibonacci(n-1) + fibonacci(n-2)
}
```

**Deutsch:**
```ssl
#!lang de
funktion fibonacci(n: Ganzzahl) -> Ganzzahl {
    wenn n <= 1 {
        rückgabe n
    }
    rückgabe fibonacci(n-1) + fibonacci(n-2)
}
```

**Français:**
```ssl
#!lang fr
fonction fibonacci(n: Entier) -> Entier {
    si n <= 1 {
        retour n
    }
    retour fibonacci(n-1) + fibonacci(n-2)
}
```

**Español:**
```ssl
#!lang es
función fibonacci(n: Entero) -> Entero {
    si n <= 1 {
        retorno n
    }
    retorno fibonacci(n-1) + fibonacci(n-2)
}
```

#### NLP-Features

- **Schlüsselwort-Übersetzung**: Alle Sprach-Schlüsselwörter (`fn`, `if`, `while`, usw.) in der Muttersprache verfügbar
- **Typ-Namen**: Basis-Typen übersetzt (`Int` → `Ganzzahl`, `String` → `Zeichenkette`)
- **Eingebaute Funktionen**: Standardbibliothek in jeder Sprache (`print` → `ausgabe`, `println` → `出力`)
- **Mehrsprachige Projekte**: Module in verschiedenen Sprachen kombinieren
- **IDE-Unterstützung**: Autovervollständigung und Syntax-Highlighting für alle Sprachen
- **Dokumentation**: Vollständige Anleitungen in jeder Sprache verfügbar

**Mehr erfahren**: [NLP Guide](docs/NLP_GUIDE.md) | [NLP Leitfaden (DE)](docs/NLP_LEITFADEN_DE.md)

## 🏗️ Multi-Architektur-Unterstützung

SSL v7.0 kompiliert zu nativer Assembly für mehrere Architekturen:

| Architektur | Status | Output-Format | Bootfähig |
|-------------|--------|---------------|-----------|
| **x86_64** | ✅ Produktion | ELF64, ISO | Ja |
| **ARM64** | ✅ Produktion | ELF64, IMG | Ja |
| **Apple Intel** | ✅ Produktion | Mach-O | Ja |
| **Apple Silicon (M1-M5)** | ✅ Produktion | Mach-O | Ja |
| **Steam Deck** | ✅ Produktion | ELF64, ISO | Ja |

**Build-Befehle:**
```bash
# x86_64 (Intel/AMD)
ssl compile --arch x86_64 programm.ssl

# ARM64 (Generic)
ssl compile --arch arm64 programm.ssl

# Apple Silicon (M1/M2/M3/M4/M5)
ssl compile --arch apple_m programm.ssl
```

## 📦 Installation

### Voraussetzungen

**Windows:**
- NASM oder MASM (`ml64.exe`)
- Microsoft Linker (`link.exe`)
- Python 3.8+ (für Build-Tools)

**Linux/macOS:**
- NASM
- GNU Linker (`ld`)
- GCC/Clang (für Runtime)

### Installations-Schritte

```bash
# Repository klonen
git clone https://github.com/SonnerStudio/SSL.git
cd SSL

# Installieren (nutzt Rust für Bootstrapping)
cargo install --path .

# Installation verifizieren
ssl --version
```

### Cross-Compilation Setup

Für ARM64/Apple Silicon Builds:

```bash
# ARM64 Toolchain installieren (Linux/WSL)
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu

# QEMU für Testing installieren (optional)
sudo apt install qemu-system-x86 qemu-system-arm
```

## 📚 Dokumentation

### Erste Schritte
- [Schnellstart-Anleitung](docs/ERSTE_SCHRITTE_DE.md)
- [Sprach-Tutorial](docs/TUTORIAL_DE.md)
- [Standardbibliothek-Referenz](docs/STDLIB_DE.md)

### Sprachreferenz
- [Syntax-Referenz](docs/SYNTAX_DE.md)
- [Typ-System](docs/TYPEN_DE.md)
- [Pattern Matching](docs/PATTERNS_DE.md)
- [Speicherverwaltung](docs/SPEICHER_DE.md)

### Compiler-Anleitungen
- [Compiler-Architektur](docs/COMPILER_DE.md)
- [Cross-Compilation](docs/CROSS_COMPILE_DE.md)
- [Optimierungs-Level](docs/OPTIMIERUNG_DE.md)
- [Debugging](docs/DEBUGGING_DE.md)

### Fortgeschrittene Themen
- [Betriebssystem-Entwicklung](docs/OS_DEV_DE.md)
- [Bare-Metal-Programmierung](docs/BARE_METAL_DE.md)
- [FFI (Foreign Function Interface)](docs/FFI_DE.md)

## 💡 Beispiele

### Hello World

```ssl
fn main() -> Int {
    print("Hallo, Welt!")
    return 0
}
```

### Fibonacci (Rekursiv)

```ssl
fn fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() -> Int {
    let ergebnis = fibonacci(10)
    print(int_to_string(ergebnis))
    return 0
}
```

### Betriebssystem Kernel-Modul

```ssl
fn kernel_main() -> Int {
    vga_print("ZetaTron-OS gestartet!")
    
    init_memory()
    init_scheduler()
    init_security()
    
    while 1 > 0 {
        // Idle-Schleife
        0
    }
    
    return 0
}
```

**Weitere Beispiele**: [examples/](examples/)

## ⚡ Benchmarks

SSL v7.0 liefert außergewöhnliche Performance mit nativer Kompilierung:

| Benchmark | SSL v7.0 | SSL v5.0 | Python | C |
|-----------|----------|----------|--------|---|
| Fibonacci(30) | **15ms** | 42ms | 380ms | 12ms |
| Primzahl-Sieb(100k) | **3.2ms** | 8.3ms | 45ms | 2.8ms |
| Matrix-Mult(100x100) | **45ms** | 125ms | 850ms | 38ms |

**Vorteile der nativen Kompilierung:**
- 2,8x schneller als SSL v5.0 (Interpreter)
- 25x schneller als Python
- Innerhalb von 95% der C-Performance

## 📖 Versionshistorie

Die Entwicklung von SSL begann im **November 2025** mit einem rapiden Entwicklungszyklus, der im Release von v7.0 am **09. Dezember 2025** gipfelte.

### SSL v7.0 (09. Dezember 2025) - **AKTUELLES RELEASE**
- ✅ Native x64-Kompilierung (NASM/MASM)
- ✅ Multi-Architektur-Support (x86_64, ARM64, Apple Silicon)
- ✅ Natural Language Programming (NLP)
- ✅ Produktions-Compiler-Pipeline
- ✅ Erstes öffentliches GitHub-Release

### Rapide Entwicklungsphase (Nov - Dez 2025)

**v6.0 (Anfang Dez 2025)**
- Erweitertes Typ-System & Fehlermeldungen

**v5.0 (Ende Nov 2025)**
- Self-Hosting Compiler
- Time-Travel Debugging & Hot Reload

**v4.0 (Ende Nov 2025)**
- Algebraische Effekte & Lineare Typen

**v1.0 - v3.0 (November 2025)**
- Initiales Design & Prototyping
- Basis-Syntax & VM-Implementierung

**Vollständiges Changelog**: [CHANGELOG.md](CHANGELOG.md)

## 🤝 Mitwirken

Wir freuen uns über Beiträge! Siehe [CONTRIBUTING.md](CONTRIBUTING.md) für Richtlinien.

**Bereiche, die Hilfe benötigen:**
- ARM64 HAL-Implementierungen
- Standardbibliothek-Erweiterung
- Dokumentations-Verbesserungen
- Test-Abdeckung

## 🌐 Community

- **Discord**: [Unserem Server beitreten](https://discord.gg/sonnerstudio)
- **Forum**: [discuss.sonnerstudio.com](https://discuss.sonnerstudio.com)
- **Twitter**: [@SonnerStudio](https://twitter.com/SonnerStudio)

## 📄 Lizenz

Copyright © 2024-2025 SonnerStudio GmbH

Lizenziert unter der Apache License, Version 2.0 (die "Lizenz");
Sie dürfen diese Datei nur in Übereinstimmung mit der Lizenz verwenden.
Eine Kopie der Lizenz erhalten Sie unter

    http://www.apache.org/licenses/LICENSE-2.0

Sofern nicht durch geltendes Recht vorgeschrieben oder schriftlich vereinbart,
wird die unter der Lizenz vertriebene Software "WIE BESEHEN" VERTRIEBEN,
OHNE GARANTIEN ODER BEDINGUNGEN IRGENDEINER ART, weder ausdrücklich noch stillschweigend.
Siehe die Lizenz für die spezifische Sprache, die Berechtigungen und
Einschränkungen unter der Lizenz regelt.

---

<div align="center">

**Mit ❤️ von SonnerStudio**

[![GitHub Stars](https://img.shields.io/github/stars/SonnerStudio/SSL?style=social)](https://github.com/SonnerStudio/SSL)
[![Twitter Follow](https://img.shields.io/twitter/follow/SonnerStudio?style=social)](https://twitter.com/SonnerStudio)

</div>
