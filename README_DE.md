# Sonner Studio Language (SSL)

**Eine KI-native, universelle Programmiersprache der Zukunft**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/yourusername/ssl)
[![Lizenz](https://img.shields.io/badge/lizenz-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-bestanden-brightgreen.svg)](#tests)

---

## 🌍 Sprachen / Languages

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 📖 Schnellzugriff

**[📚 Vollständige Dokumentation](DOCUMENTATION_DE.md)** | **[💡 Beispiele](EXAMPLES.md)** | **[⚖️ Lizenz](LICENSE_DE.md)**

Verfügbar in: [EN](DOCUMENTATION.md) | [DE](DOCUMENTATION_DE.md) | [FR](DOCUMENTATION_FR.md) | [ES](DOCUMENTATION_ES.md) | [PT](DOCUMENTATION_PT.md) | [JA](DOCUMENTATION_JA.md)

---

## 🌟 Highlights

SSL ist eine **experimentelle Programmiersprache**, die moderne und futuristische Konzepte vereint:

- ⚡ **Parallel-by-Design**: Native Unterstützung für Threads und Message-Passing
- ⚛️ **Quantum Computing**: Integrierter Quantensimulator
- 🩹 **Self-Healing Code**: Automatische Fehlerbehandlung mit KI-Integration
- 🤖 **KI-Nativ**: Compiler mit KI-Optimierung und -Fehleranalyse
- 🔄 **Hybrides Typsystem**: Statisch + Dynamisch + Inferenz

---

## 🚀 Schnellstart

### Installation

```bash
git clone https://github.com/yourusername/ssl.git
cd ssl
cargo build --release
```

### Ihr erstes SSL-Programm

```ssl
fn main() {
    print("Hallo, SSL!")
}
```

Ausführen:
```bash
ssl run examples/hello.ssl
```

---

## 📖 Features

### 1. Parallelität ⚡

```ssl
let chan = channel()

spawn {
    send(chan[0], "Hallo vom Thread!")
}

print(recv(chan[1]))
```

### 2. Quantum Computing ⚛️

```ssl
let q = Qubit()
H(q)  // Hadamard: Superposition
let result = Measure(q)
print(result)  // 0 oder 1 (50/50)
```

### 3. Self-Healing Code 🩹

```ssl
try {
    let result = risikoreiche_operation()
} recover (err) {
    print("Fehler abgefangen:", err)
    // Automatische Wiederherstellung
}
```

### 4. Funktionen & Rekursion

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 { return n }
    return fib(n-1) + fib(n-2)
}

print(fib(10))  // 55
```

---

## 🛠️ Toolchain

### CLI-Befehle

```bash
ssl run <datei>     # Code ausführen
ssl build <pfad>    # Projekt kompilieren
ssl check <datei>   # Syntax prüfen
ssl doctor          # System-Check
ssl lsp             # Language Server starten
```

### KI-Daemon (ssld)

```bash
ssld  # Startet den KI-Daemon für Code-Analyse
```

### VS Code Extension

1. Öffne `editors/vscode/`
2. `npm install`
3. `npm run compile`
4. F5 zum Debuggen

---

## 🧪 Tests

```bash
cargo test
```

**Status**: Alle 9 Unit-Tests bestanden ✅

- ✅ Arithmetik & Variablen
- ✅ Funktionen & Rekursion
- ✅ Vergleichsoperatoren
- ✅ Threads (`spawn`)
- ✅ Channels (`channel`, `send`, `recv`)
- ✅ Quantum Gates (`Qubit`, `H`, `Measure`)
- ✅ Try-Recover (Self-Healing)

---

## 📚 Dokumentation

- [Vollständige Dokumentation](DOCUMENTATION_DE.md)
- [Beispiele](EXAMPLES.md)
- [Lizenz](LICENSE_DE.md)

---

## 🗺️ Roadmap

### Phase 0-5: ✅ Abgeschlossen
- [x] Vision & Philosophie
- [x] Kernarchitektur (EBNF, Typsystem)
- [x] Interpreter-Prototyp
- [x] KI-Integration (Ollama)
- [x] Toolchain (CLI, LSP, KI-Daemon)
- [x] Erweiterte Features (Parallel, Quantum, Self-Healing)

### Phase 6: 🚧 In Planung
- [ ] Community & Open Evolution
- [ ] Governance-Modell
- [ ] Dokumentation & Tutorials

### Phase 7: 🔮 Langzeitvision
- [ ] Evolutionäre Compiler-Konzepte
- [ ] Self-Modifying Code
- [ ] Distributed Computing

---

## 🤝 Beiträge

SSL ist ein experimentelles Projekt. Beiträge sind willkommen!

1. Fork das Projekt
2. Erstelle einen Feature-Branch (`git checkout -b feature/amazing`)
3. Commit deine Änderungen (`git commit -m 'Add amazing feature'`)
4. Push zum Branch (`git push origin feature/amazing`)
5. Öffne einen Pull Request

---

## 📄 Lizenz

MIT License - siehe [LICENSE](LICENSE) für Details.

---

## 🙏 Danksagungen

- **Rust Community** für die exzellente Tooling-Unterstützung
- **Ollama** für das KI-Framework
- **tower-lsp** für die LSP-Integration

---

## 📧 Kontakt

- GitHub: [@yourusername](https://github.com/yourusername)
- Email: your.email@example.com
- Discord: SSL Community Server (demnächst)

---

**Gebaut mit ❤️ und Rust** 🦀
