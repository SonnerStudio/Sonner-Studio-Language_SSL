# ⚡ SSL v5.0 Manual

> **Complete Feature Reference & API Documentation**

---

### Choose Language / Sprache auswählen

| 🇬🇧 🇺🇸 <br> **ENGLISH** | 🇩🇪 🇦🇹 🇨🇭 <br> [**DEUTSCH**](MANUAL_DE.md) |
| :---: | :---: |
| *(Selected / Ausgewählt)* | *(Switch / Wechseln)* |

---

## Table of Contents

1. [Core Language](#1-core-language)
2. [Standard Library](#2-standard-library-stdlib)
3. [Advanced Features](#3-advanced-features)
4. [GUI Framework](#4-gui-framework)
5. [Development Tools](#5-development-tools)
6. [Quantum Computing](#6-quantum-computing)
7. [Distributed Computing](#7-distributed-computing)
8. [AI/ML](#8-aiml)

---

## About this Manual

This manual is the official reference for the **Sonner Studio Language v5.0 (Self-Hosting Edition)**. It covers all language features, the standard library, and specialized modules (Quantum, AI, etc.).

All code examples shown are valid SSL v5.0 code.

---

## 1. Core Language

### Variables

```ssl
let immutable = 42       // Immutable
var mutable = 0          // Mutable
const PI = 3.14159       // Constant
```

### Data Types

| Type | Example | Description |
|-----|----------|--------------|
| `Int` | `42` | 64-bit Integer |
| `Float` | `3.14` | 64-bit Floating Point |
| `Bool` | `true` | Boolean |
| `String` | `"Hello"` | UTF-8 String |
| `Char` | `'a'` | Unicode Character |
| `List<T>` | `[1, 2, 3]` | Dynamic Array |
| `Map<K, V>` | `{"a": 1}` | Hash Map |
| `Option<T>` | `Some(42)` | Optional Value |
| `Result<T, E>` | `Ok(value)` | Error Handling |

### Functions

```ssl
// Simple Function
fn add(a: Int, b: Int) -> Int {
    a + b
}

// Generic Function
fn first<T>(list: List<T>) -> Option<T> {
    if list.is_empty() { None } else { Some(list[0]) }
}

// Lambda / Closure
let double = |x| x * 2
let result = [1, 2, 3].map(|x| x * x)
```

### Control Flow

```ssl
// If-Expression
let max = if a > b { a } else { b }

// Match (Pattern Matching)
match value {
    0 => "zero"
    1..=9 => "single digit"
    n if n < 0 => "negative"
    _ => "large"
}
```

### Structs and Enums

```ssl
// Struct
struct Point {
    x: Float
    y: Float
}

impl Point {
    fn new(x: Float, y: Float) -> Point {
        Point { x, y }
    }
}

// Enum
type Color =
    | Red
    | Green
    | Blue
    | RGB(Int, Int, Int)
```

---

## 2. Standard Library (stdlib)

### io

```ssl
import stdlib.io

io::println("Hello!")
let line = io::input()
```

### string

```ssl
import stdlib.string

let s = "Hello, World!"
s.length()              // 13
s.to_upper()            // "HELLO, WORLD!"
s.contains("World")     // true
```

### list

```ssl
import stdlib.list

let list = [1, 2, 3]
list.push(4)
list.map(|x| x * 2)     // [2, 4, 6]
list.filter(|x| x > 2)  // [3, 4]
list.reduce(0, |a, b| a + b)
```

(See `MANUAL_DE.md` for full API details as methods map 1:1)

---

## 3. Advanced Features

### Algebraic Effects

```ssl
import features.effects

effect Logger {
    fn log(msg: String) -> Unit
}

handle {
    log("Hello")
} with Logger {
    log(msg) => { println(msg); resume(()) }
}
```

### Linear Types

```ssl
import features.linear

linear struct Connection { ... }

// Compiler enforces resource cleanup
let conn = Connection.open()
conn.send("data")
conn.close()  // Mandatory!
```

---

## 4. GUI Framework

### Window Shapes

SSL supports non-rectangular windows natively:

```ssl
import gui

let circle = Window.circle("Circle App", 200.0)
let star = star_window("Star App", 100.0, 50.0, 5)
```

### Widgets

```ssl
let button = Button.new("Click Me")
    .on_click(|| println("Clicked!"))
    .style(Primary)
```

---

## 5. Development Tools

- **CLI**: `ssl run`, `ssl build`, `ssl test`
- **LSP**: Full IDE support for VS Code
- **Hot Reload**: `HotReloadEngine` for live updates
- **Debugger**: Time-Travel debugging (`step_back()`, `replay()`)

---

## 6. Quantum Computing

```ssl
import quantum

let q = Qubit.zero()
let superposition = hadamard().apply(q)
let result = q.measure()
```

---

## 7. Distributed Computing

```ssl
import distributed

var system = ActorSystem.create("cluster")
let actor = system.spawn("worker", WorkerActor {})
```

---

## 8. AI/ML

```ssl
import ai

let t = Tensor.randn([3, 3])
let model = Sequential.new()
model.add(Linear.new(784, 10))
```

---

**SSL v5.0 - Self-Hosting Edition**
© 2024 SonnerStudio GmbH
