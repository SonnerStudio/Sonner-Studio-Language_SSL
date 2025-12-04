# Sonner Studio Language (SSL) v3.1 - The Official Manual

## Table of Contents
1.  [Introduction](#1-introduction)
2.  [Installation & Setup](#2-installation--setup)
3.  [Language Basics](#3-language-basics)
4.  [Functional Programming (v3.0)](#4-functional-programming)
5.  [The Aurora JIT Compiler (v3.1)](#5-the-aurora-jit-compiler)
6.  [Standard Library](#6-standard-library)
7.  [Tooling & Ecosystem](#7-tooling--ecosystem)

---

## 1. Introduction

Welcome to **SSL v3.1**, the programming language designed for the future of software development. SSL combines the safety and expressiveness of functional programming with the raw performance of a native JIT compiler, all wrapped in a developer-friendly ecosystem with AI tools and time-travel debugging.

### Key Philosophy
-   **Hybrid Paradigm:** Seamlessly mix imperative and functional code.
-   **Performance First:** Native machine code generation via LLVM.
-   **Developer Experience:** Tools that understand your code (AI, Hot Reload).

---

## 2. Installation & Setup

### Prerequisites
-   Rust Toolchain (latest stable)
-   LLVM 17.0 (for JIT support)

### Building from Source
```bash
git clone https://github.com/SonnerStudio/Sonner-Studio-Language_SSL
cd Sonner-Studio-Language_SSL
cargo build --release --features llvm
```

### Running Your First Program
Create a file named `hello.ssl`:
```ssl
print("Hello, World!")
```

Run it:
```bash
ssl run hello.ssl
```

---

## 3. Language Basics

### Variables
By default, variables are **immutable** in SSL v3.0+.
```ssl
let x = 10      // Immutable
// x = 20       // Error!

let mut y = 10  // Mutable
y = 20          // OK
```

### Data Types
-   `Int`: 64-bit signed integer
-   `Float`: 64-bit floating point
-   `String`: UTF-8 string
-   `Bool`: `true` or `false`
-   `List<T>`: Dynamic array
-   `Map<K, V>`: Key-value store

### Control Flow
```ssl
if x > 10 {
    print("Big")
} else {
    print("Small")
}

// Loops
for i in 0..10 {
    print(i)
}
```

### Functions
```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

---

## 4. Functional Programming

SSL v3.0 introduces powerful functional features.

### The Pipe Operator (`|>`)
Chain function calls to create readable data pipelines.
```ssl
// Old way
let res = square(double(5))

// SSL way
let res = 5 |> double |> square
```

### Auto-Currying
Functions can be partially applied.
```ssl
fn multiply(a: Int, b: Int) -> Int { return a * b }

let double = multiply(2)  // Returns a new function
print(double(10))         // Prints 20
```

### Immutability
Data structures are immutable by default. Modifications return *new* copies.
```ssl
let list = [1, 2, 3]
let new_list = list.push(4)
// list is still [1, 2, 3]
// new_list is [1, 2, 3, 4]
```

---

## 5. The Aurora JIT Compiler

SSL v3.1 features a state-of-the-art LLVM-based JIT compiler.

### How it Works
1.  **Interpretation:** Code starts running immediately in the interpreter.
2.  **Hot-Path Detection:** The runtime identifies frequently executed functions.
3.  **Compilation:** "Hot" functions are compiled to native machine code in the background.
4.  **Optimization:** LLVM applies aggressive optimizations (inlining, constant folding).
5.  **Native Execution:** Subsequent calls use the ultra-fast native code.

### Performance
Benchmarks show a **5-10x speedup** for compute-heavy tasks compared to the interpreter.

---

## 6. Standard Library

SSL comes with a "Batteries Included" standard library.

-   `std::io` - File and console I/O
-   `std::net` - HTTP client/server
-   `std::json` - JSON parsing/serialization
-   `std::math` - Advanced math functions
-   `std::async` - Channels and futures

---

## 7. Tooling & Ecosystem

### Time-Travel Debugger
Step *backwards* in time to find bugs.
```bash
ssl run app.ssl --debug
# Use @back to step back
```

### AI Code Review
Get instant feedback on your code.
```bash
ssl run app.ssl --ai-review
```

### Hot Reload
Change code while the app is running.
```bash
ssl run app.ssl --watch
```

---

*© 2024 Sonner Studio. Licensed under MIT/Apache 2.0.*
