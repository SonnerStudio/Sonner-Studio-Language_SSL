# SSL Manual

## Introduction

Sonner Studio Language (SSL) is a modern programming language designed for AI-era development. This manual covers the core language features and syntax.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Variables](#variables)
3. [Types](#types)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Pattern Matching](#pattern-matching)
7. [Data Structures](#data-structures)
8. [Modules](#modules)
9. [Error Handling](#error-handling)
10. [Advanced Features](#advanced-features)

---

## Getting Started

### Hello World

```ssl
fn main() {
    print("Hello, SSL!")
}
```

### Running SSL Code

```bash
# Run a file
ssl run main.ssl

# Interactive REPL
ssl run

# Check syntax
ssl check main.ssl
```

---

## Variables

### Immutable (default)

```ssl
let name = "SSL"
let age = 1
let pi = 3.14159
```

### Mutable

```ssl
var counter = 0
counter = counter + 1
```

### Type Annotations

```ssl
let name: String = "SSL"
let numbers: List<Int> = [1, 2, 3]
```

---

## Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| Int | 64-bit integer | `42` |
| Float | 64-bit float | `3.14` |
| Bool | Boolean | `true`, `false` |
| String | UTF-8 string | `"hello"` |
| Char | Unicode character | `'a'` |

### Compound Types

```ssl
// Lists
let numbers = [1, 2, 3, 4, 5]

// Maps
let person = {
    name: "Alice",
    age: 30
}

// Tuples
let point = (10, 20)
```

### Custom Types

```ssl
// Struct
struct User {
    name: String
    email: String
    age: Int
}

// Enum (Sum Type)
type Result<T, E> = Ok(T) | Err(E)
type Option<T> = Some(T) | None
```

---

## Functions

### Basic Functions

```ssl
fn add(a: Int, b: Int) -> Int {
    a + b
}

fn greet(name: String) {
    print("Hello, ${name}!")
}
```

### Lambda Expressions

```ssl
let double = |x| x * 2
let add = |a, b| a + b

let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(|x| x * 2)
```

### Higher-Order Functions

```ssl
fn apply_twice<T>(f: (T) -> T, x: T) -> T {
    f(f(x))
}

let result = apply_twice(|x| x * 2, 5)  // 20
```

---

## Control Flow

### If/Else

```ssl
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}

// If as expression
let max = if a > b { a } else { b }
```

### Loops

```ssl
// For loop
for item in list {
    print(item)
}

// For with range
for i in 0..10 {
    print(i)
}

// While loop
while condition {
    // ...
}
```

---

## Pattern Matching

```ssl
match value {
    0 => print("zero")
    1 | 2 => print("one or two")
    n if n < 10 => print("small: ${n}")
    _ => print("other")
}

// Destructuring
match point {
    (0, 0) => print("origin")
    (x, 0) => print("on x-axis at ${x}")
    (0, y) => print("on y-axis at ${y}")
    (x, y) => print("at (${x}, ${y})")
}

// Option matching
match maybe_value {
    Some(x) => print("Got: ${x}")
    None => print("Nothing")
}
```

---

## Data Structures

### Structs

```ssl
struct Point {
    x: Float
    y: Float
}

impl Point {
    fn new(x: Float, y: Float) -> Point {
        Point { x, y }
    }
    
    fn distance(self, other: Point) -> Float {
        sqrt((self.x - other.x)^2 + (self.y - other.y)^2)
    }
}

let p = Point.new(3.0, 4.0)
print(p.distance(Point.new(0.0, 0.0)))  // 5.0
```

### Enums

```ssl
type Color = Red | Green | Blue | Rgb(Int, Int, Int)

fn color_name(c: Color) -> String {
    match c {
        Red => "red"
        Green => "green"
        Blue => "blue"
        Rgb(r, g, b) => "rgb(${r}, ${g}, ${b})"
    }
}
```

---

## Modules

### Importing

```ssl
import math { sqrt, sin, cos }
import http { Request, Response }
import json

// Aliased import
import long_module_name as short
```

### Defining Modules

```ssl
// In math.ssl
pub fn add(a: Int, b: Int) -> Int {
    a + b
}

fn private_helper() {
    // Not exported
}
```

---

## Error Handling

### Result Type

```ssl
fn divide(a: Float, b: Float) -> Result<Float, String> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

// Using Result
match divide(10.0, 2.0) {
    Ok(result) => print("Result: ${result}")
    Err(msg) => print("Error: ${msg}")
}

// With ? operator
fn calculate() -> Result<Float, String> {
    let x = divide(10.0, 2.0)?
    let y = divide(x, 3.0)?
    Ok(y)
}
```

### Try/Catch

```ssl
try {
    risky_operation()
} catch e {
    print("Error: ${e}")
} finally {
    cleanup()
}
```

---

## Advanced Features

### Generics

```ssl
fn identity<T>(x: T) -> T {
    x
}

struct Stack<T> {
    items: List<T>
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item)
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

### Traits

```ssl
trait Display {
    fn display(self) -> String
}

impl Display for Point {
    fn display(self) -> String {
        "(${self.x}, ${self.y})"
    }
}
```

### Async/Await

```ssl
async fn fetch_data(url: String) -> Result<String, Error> {
    let response = await http.get(url)?
    Ok(response.body)
}

async fn main() {
    let data = await fetch_data("https://api.example.com")?
    print(data)
}
```

---

## v4.0 Features

See [V4.0_FEATURES.md](V4.0_FEATURES.md) for new features:

- Property-Based Testing
- Reactive Streams
- Edge Deployment
- CRDT Data Structures
- GPU/SIMD Support
- Formal Verification
- Algebraic Effects
- Linear Types

---

*SSL v4.0.0 - December 2024*
