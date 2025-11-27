## 🌍 Languages / Sprachen

**[English](DOCUMENTATION.md)** | **[Deutsch](DOCUMENTATION_DE.md)** | **[Français](DOCUMENTATION_FR.md)** | **[Español](DOCUMENTATION_ES.md)** | **[Português](DOCUMENTATION_PT.md)** | **[日本語](DOCUMENTATION_JA.md)**

---

# SSL Documentation

## Table of Contents

1. [Introduction](#introduction)
2. [Language Features](#language-features)
3. [Installation](#installation)
4. [Getting Started](#getting-started)
5. [Language Syntax](#language-syntax)
6. [Built-in Functions](#built-in-functions)
7. [Examples](#examples)
8. [FAQ](#faq)

---

## Introduction

Sonner Studio Language (SSL) is an experimental, AI-native programming language designed for the future of computing. It combines:

- **Parallel-by-Design**: Native concurrency with threads and channels
- **Quantum Computing**: Built-in quantum simulator for quantum algorithms
- **Self-Healing Code**: Automatic error recovery with AI assistance
- **Modern Type System**: Hybrid static/dynamic typing with inference

SSL is designed to make advanced programming concepts accessible and intuitive.

---

## Language Features

### 1. Variables and Types

```ssl
// Immutable variable
let x = 10

// Mutable variable
mut count = 0
count = count + 1

// Type annotations (optional)
let name: String = "SSL"
```

### 2. Functions

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn greet(name: String) {
    print("Hello, ", name)
}
```

### 3. Control Flow

```ssl
// If-Else
if x > 10 {
    print("Greater")
} else {
    print("Smaller")
}

// For Loop
for i in 0..10 {
    print(i)
}

// While Loop
mut i = 0
while i < 10 {
    print(i)
    i = i + 1
}
```

### 4. Parallel Programming

```ssl
// Create a channel
let chan = channel()
let tx = chan[0]  // Sender
let rx = chan[1]  // Receiver

// Spawn a thread
spawn {
    send(tx, 42)
}

// Receive data
let result = recv(rx)
print(result)  // 42
```

### 5. Quantum Computing

```ssl
// Create a qubit
let q = Qubit()

// Apply Hadamard gate (superposition)
H(q)

// Measure the qubit
let result = Measure(q)
print(result)  // 0 or 1 (50/50 probability)
```

### 6. Error Handling

```ssl
try {
    let result = risky_operation()
    print(result)
} recover (err) {
    print("Error occurred:", err)
    // Handle error or use fallback
}
```

---

## Installation

### Prerequisites

- Rust toolchain (1.70+)
- Git

### Steps

```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL.git
cd Sonner-Studio-Language_SSL
cargo build --release
```

The `ssl` binary will be in `target/release/`.

---

## Getting Started

### Your First Program

Create a file `hello.ssl`:

```ssl
fn main() {
    print("Hello, World!")
}
```

Run it:

```bash
ssl run hello.ssl
```

### Fibonacci Example

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

## Language Syntax

### Comments

```ssl
// Single-line comment
```

### Data Types

- `Int`: Integer numbers (64-bit)
- `Float`: Floating-point numbers
- `String`: Text strings
- `Bool`: Boolean (true/false)
- `List`: Dynamic arrays
- `Qubit`: Quantum bits

### Operators

**Arithmetic**: `+`, `-`, `*`, `/`
**Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
**Range**: `..` (e.g., `0..10`)

---

## Built-in Functions

### I/O Functions

- `print(...args)`: Print values to console

### Parallel Functions

- `channel()`: Create a communication channel
- `send(sender, value)`: Send value through channel
- `recv(receiver)`: Receive value from channel
- `spawn { ... }`: Start new thread

### Quantum Functions

- `Qubit()`: Create a quantum bit
- `H(qubit)`: Apply Hadamard gate
- `X(qubit)`: Apply Pauli-X gate
- `Measure(qubit)`: Measure qubit state

---

## Examples

### Producer-Consumer Pattern

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
        print("Received:", value)
    }
}
```

### Quantum Superposition

```ssl
fn main() {
    let q = Qubit()
    H(q)  // Create superposition
    
    // Measure multiple times
    for i in 0..10 {
        let q = Qubit()
        H(q)
        print(Measure(q))
    }
}
```

### Error Recovery

```ssl
fn divide(a: Int, b: Int) -> Int {
    try {
        return a / b
    } recover (err) {
        print("Cannot divide by zero!")
        return 0
    }
}
```

---

## FAQ

### Is SSL production-ready?

No, SSL is an experimental language for research and education.

### What platforms are supported?

SSL runs on any platform supported by Rust (Windows, macOS, Linux).

### How fast is SSL?

SSL uses a tree-walking interpreter, optimized for clarity over speed.

### Can I contribute?

Yes! See our [GitHub repository](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL) for contribution guidelines.

### Where can I report bugs?

Please open an issue on our [GitHub Issues page](https://github.com/SonnerStudio/Sonner-Studio-Language_SSL/issues).

---

**Built with ❤️ and Rust** 🦀
