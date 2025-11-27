## 🌍 Sprachen / Languages

**[English](DOCUMENTATION.md)** | **[Deutsch](DOCUMENTATION_DE.md)** | **[Français](DOCUMENTATION_FR.md)** | **[Español](DOCUMENTATION_ES.md)** | **[Português](DOCUMENTATION_PT.md)** | **[日本語](DOCUMENTATION_JA.md)**

---

# SSL Dokumentation

## Inhaltsverzeichnis

1. [Einführung](#einführung)
2. [Sprachfeatures](#sprachfeatures)
3. [Installation](#installation)
4. [Erste Schritte](#erste-schritte)
5. [Sprachsyntax](#sprachsyntax)
6. [Eingebaute Funktionen](#eingebaute-funktionen)
7. [Beispiele](#beispiele)
8. [FAQ](#faq)

---

## Einführung

Sonner Studio Language (SSL) ist eine experimentelle, KI-native Programmiersprache für die Zukunft der Datenverarbeitung. Sie kombiniert:

- **Parallel-by-Design**: Native Nebenläufigkeit mit Threads und Channels
- **Quantum Computing**: Integrierter Quantensimulator für Quantenalgorithmen
- **Self-Healing Code**: Automatische Fehlerwiederherstellung mit KI-Unterstützung
- **Modernes Typsystem**: Hybrides statisches/dynamisches Typing mit Inferenz

SSL wurde entwickelt, um fortgeschrittene Programmierkonzepte zugänglich und intuitiv zu machen.

---

## Sprachfeatures

### 1. Variablen und Typen

```ssl
// Unveränderliche Variable
let x = 10

// Veränderliche Variable
mut count = 0
count = count + 1

// Typannotationen (optional)
let name: String = "SSL"
```

### 2. Funktionen

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn greet(name: String) {
    print("Hallo, ", name)
}
```

### 3. Kontrollfluss

```ssl
// If-Else
if x > 10 {
    print("Größer")
} else {
    print("Kleiner")
}

// For-Schleife
for i in 0..10 {
    print(i)
}

// While-Schleife
mut i = 0
while i < 10 {
    print(i)
    i = i + 1
}
```

### 4. Parallele Programmierung

```ssl
// Channel erstellen
let chan = channel()
let tx = chan[0]  // Sender
let rx = chan[1]  // Empfänger

// Thread starten
spawn {
    send(tx, 42)
}

// Daten empfangen
let result = recv(rx)
print(result)  // 42
```

### 5. Quantum Computing

```ssl
// Qubit erstellen
let q = Qubit()

// Hadamard-Gate anwenden (Superposition)
H(q)

// Qubit messen
let result = Measure(q)
print(result)  // 0 oder 1 (50/50 Wahrscheinlichkeit)
```

### 6. Fehlerbehandlung

```ssl
try {
    let result = riskante_operation()
    print(result)
} recover (err) {
    print("Fehler aufgetreten:", err)
    // Fehler behandeln oder Fallback verwenden
}
```

---

## Installation

### Voraussetzungen

- Rust-Toolchain (1.70+)
- Git

### Schritte

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

Die `ssl`-Binary befindet sich in `target/release/`.

---

## Erste Schritte

### Ihr erstes Programm

Erstellen Sie eine Datei `hello.ssl`:

```ssl
fn main() {
    print("Hallo, Welt!")
}
```

Ausführen:

```bash
ssl run hello.ssl
```

### Fibonacci-Beispiel

```ssl
fn fib(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    return fib(n-1) + fib(n-2)
}

fn main() {
    print("Fibonacci(10) =", fib(10))
}
```

---

## Sprachsyntax

### Kommentare

```ssl
// Einzeiliger Kommentar
```

### Datentypen

- `Int`: Ganzzahlen (64-bit)
- `Float`: Gleitkommazahlen
- `String`: Textstrings
- `Bool`: Boolean (true/false)
- `List`: Dynamische Arrays
- `Qubit`: Quantenbits

### Operatoren

**Arithmetik**: `+`, `-`, `*`, `/`
**Vergleich**: `==`, `!=`, `<`, `<=`, `>`, `>=`
**Bereich**: `..` (z.B. `0..10`)

---

## Eingebaute Funktionen

### I/O-Funktionen

- `print(...args)`: Werte in Konsole ausgeben

### Parallel-Funktionen

- `channel()`: Kommunikationskanal erstellen
- `send(sender, value)`: Wert über Kanal senden
- `recv(receiver)`: Wert vom Kanal empfangen
- `spawn { ... }`: Neuen Thread starten

### Quantum-Funktionen

- `Qubit()`: Quantenbit erstellen
- `H(qubit)`: Hadamard-Gate anwenden
- `X(qubit)`: Pauli-X-Gate anwenden
- `Measure(qubit)`: Qubit-Zustand messen

---

## Beispiele

### Producer-Consumer-Pattern

```ssl
fn main() {
    let chan = channel()
    
    // Producer
    spawn {
        for i in 0..10 {
            send(chan[0], i)
        }
    }
    
    // Consumer
    for i in 0..10 {
        let value = recv(chan[1])
        print("Empfangen:", value)
    }
}
```

### Quanten-Superposition

```ssl
fn main() {
    let q = Qubit()
    H(q)  // Superposition erzeugen
    
    // Mehrfach messen
    for i in 0..10 {
        let q = Qubit()
        H(q)
        print(Measure(q))
    }
}
```

---

## FAQ

### Ist SSL produktionsreif?

Nein, SSL ist eine experimentelle Sprache für Forschung und Bildung.

### Welche Plattformen werden unterstützt?

SSL läuft auf allen von Rust unterstützten Plattformen (Windows, macOS, Linux).

### Wie schnell ist SSL?

SSL verwendet einen Tree-Walking-Interpreter, der für Klarheit über Geschwindigkeit optimiert ist.

---

**Gebaut mit ❤️ und Rust** 🦀
