# Sonner Studio Language (SSL)

**An AI-native, universal programming language of the future**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](#tests)

---

## 🌍 Languages / Sprachen

**[English](README.md)** | **[Deutsch](README_DE.md)** | **[Français](README_FR.md)** | **[Español](README_ES.md)** | **[Português](README_PT.md)** | **[日本語](README_JA.md)**

---

## 📖 Quick Links

**[📚 Full Documentation](DOCUMENTATION.md)** | **[💡 Examples](EXAMPLES.md)** | **[⚖️ License](LICENSE)**

Available in: [EN](DOCUMENTATION.md) | [DE](DOCUMENTATION_DE.md) | [FR](DOCUMENTATION_FR.md) | [ES](DOCUMENTATION_ES.md) | [PT](DOCUMENTATION_PT.md) | [JA](DOCUMENTATION_JA.md)

---

## 🌟 Highlights

SSL is an **experimental programming language** that combines modern and futuristic concepts:

- ⚡ **Parallel-by-Design**: Native support for threads and message-passing
- ⚛️ **Quantum Computing**: Integrated quantum simulator
- 🩹 **Self-Healing Code**: Automatic error handling with AI integration
- 🤖 **AI-Native**: Compiler with AI optimization and error analysis
- 🔄 **Hybrid Type System**: Static + Dynamic + Inference

---

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
    print("Hello, SSL!")
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
    send(chan[0], "Hello from thread!")
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
    let result = risky_operation()
} recover (err) {
    print("Error caught:", err)
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
ssl run <file>      # Code ausführen
ssl build <path>    # Projekt kompilieren
ssl check <file>    # Syntax prüfen
ssl doctor          # System-Check
ssl lsp             # Language Server starten
```

### AI-Daemon (ssld)

```bash
ssld  # Startet den AI-Daemon für Code-Analyse
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

## 📚 Documentation

- [Full Documentation](DOCUMENTATION.md)
- [Examples](EXAMPLES.md)
- [License](LICENSE)

---

## 🗺️ Roadmap

### Phase 0-5: ✅ Abgeschlossen
- [x] Vision & Philosophie
- [x] Kernarchitektur (EBNF, Typsystem)
- [x] Interpreter-Prototyp
- [x] KI-Integration (Ollama)
- [x] Toolchain (CLI, LSP, AI-Daemon)
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

- GitHub: [@SonnerStudio](https://github.com/yourusername)
- Email: hbcomputer@freenet.de
- Discord: [Join our community](https://discord.gg/J8eXPzpt)

---

**Gebaut mit ❤️ und Rust** 🦀
