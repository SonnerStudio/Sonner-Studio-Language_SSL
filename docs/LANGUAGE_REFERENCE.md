# SSL v7.0 Language Reference

Complete reference for the Sonner Studio Language v7.0 Native Compilation Edition.

## Table of Contents

1. [Lexical Structure](#lexical-structure)
2. [Types](#types)
3. [Variables](#variables)
4. [Functions](#functions)
5. [Control Flow](#control-flow)
6. [Operators](#operators)
7. [Standard Library](#standard-library)
8. [Memory Model](#memory-model)

---

## Lexical Structure

### Comments

```ssl
// Single-line comment

/* 
   Multi-line comment
   Can span multiple lines
*/
```

### Keywords

Reserved words in SSL v7.0:

```
fn      let     mut     if      else    while   
return  Int     Float   String  Bool    true    
false   struct  enum    impl    trait   mod
use     pub     priv    const   static  extern
```

### Identifiers

Valid identifier formats:
- Start with letter or underscore: `a-z`, `A-Z`, `_`
- Continue with letters, digits, underscores: `a-z`, `A-Z`, `0-9`, `_`

```ssl
let valid_name = 42
let _privateVar = true
let MyType = 100
```

**Invalid:**
```ssl
let 123invalid = 0  // Cannot start with digit
let my-var = 0      // Hyphen not allowed
```

### Literals

#### Integer Literals

```ssl
let decimal = 42
let hex = 0x2A
let binary = 0b101010
let octal = 0o52
```

#### Floating-Point Literals

```ssl
let pi = 3.14159
let scientific = 1.5e10
let negative_exp = 2.5e-3
```

#### String Literals

```ssl
let simple = "Hello, World!"
let escaped = "Line 1\nLine 2\tTabbed"
let quote = "She said \"Hello\""
let backslash = "Path: C:\\Users\\Name"
```

**Escape Sequences:**
- `\n` - Newline
- `\t` - Tab
- `\\` - Backslash
- `\"` - Double quote
- `\r` - Carriage return

#### Boolean Literals

```ssl
let flag_true = true
let flag_false = false
```

---

## Types

### Primitive Types

| Type | Size | Description | Range |
|------|------|-------------|-------|
| `Int` | 64-bit | Signed integer | -2^63 to 2^63-1 |
| `Float` | 64-bit | IEEE 754 double | ±1.7E±308 |
| `Bool` | 1-bit | Boolean | true, false |
| `String` | Variable | UTF-8 string | Heap-allocated |

### Type Annotations

```ssl
let x: Int = 42
let y: Float = 3.14
let name: String = "SSL"
let flag: Bool = true
```

### Type Inference

SSL v7.0 can infer types:

```ssl
let x = 42          // Inferred as Int
let y = 3.14        // Inferred as Float
let text = "Hello"  // Inferred as String
```

---

## Variables

### Immutable Variables

Default variables are immutable:

```ssl
let x = 42
x = 50  // ERROR: Cannot assign to immutable variable
```

### Mutable Variables

Use `mut` keyword:

```ssl
let mut counter = 0
counter = counter + 1  // OK
counter = 10           // OK
```

### Constants

Compile-time constants:

```ssl
const PI: Float = 3.14159
const MAX_SIZE: Int = 1024
```

---

## Functions

### Function Declaration

```ssl
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // Function body
    return value
}
```

**Example:**
```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}
```

### Function Calls

```ssl
let result = add(5, 3)
print(int_to_string(result))
```

### Return Values

```ssl
fn get_value() -> Int {
    return 42
}

fn no_return() {
    print("No return value")
    // Implicit return
}
```

### Multiple Parameters

```ssl
fn calculate(x: Int, y: Int, z: Int) -> Int {
    return (x + y) * z
}

let result = calculate(2, 3, 4)  // (2 + 3) * 4 = 20
```

---

## Control Flow

### If-Else Statements

```ssl
if condition {
    // Execute if true
} else {
    // Execute if false
}
```

**Example:**
```ssl
let x = 10

if x > 5 {
    print("Greater than 5")
} else {
    print("Less than or equal to 5")
}
```

### Nested If-Else

```ssl
if x > 10 {
    print("Large")
} else {
    if x > 5 {
        print("Medium")
    } else {
        print("Small")
    }
}
```

### While Loops

```ssl
while condition {
    // Loop body
}
```

**Example:**
```ssl
let mut i = 0
while i < 10 {
    print(int_to_string(i))
    i = i + 1
}
```

### Infinite Loops

```ssl
while 1 > 0 {
    // Infinite loop
    // Use return or break to exit
}
```

---

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `5 + 3` = 8 |
| `-` | Subtraction | `5 - 3` = 2 |
| `*` | Multiplication | `5 * 3` = 15 |
| `/` | Division | `6 / 3` = 2 |
| `%` | Modulo | `7 % 3` = 1 |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `5 == 5` = true |
| `!=` | Not equal | `5 != 3` = true |
| `>` | Greater than | `5 > 3` = true |
| `<` | Less than | `3 < 5` = true |
| `>=` | Greater or equal | `5 >= 5` = true |
| `<=` | Less or equal | `3 <= 5` = true |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | Logical AND | `true && false` = false |
| `\|\|` | Logical OR | `true \|\| false` = true |
| `!` | Logical NOT | `!true` = false |

### Operator Precedence

From highest to lowest:
1. `()` - Parentheses
2. `!`, `-` (unary)
3. `*`, `/`, `%`
4. `+`, `-`
5. `>`, `<`, `>=`, `<=`
6. `==`, `!=`
7. `&&`
8. `||`

---

## Standard Library

### Built-in Functions

#### I/O Functions

```ssl
print(text: String)              // Print to stdout
println(text: String)            // Print with newline
read_line() -> String            // Read from stdin
```

#### String Functions

```ssl
string_length(s: String) -> Int
string_concat(s1: String, s2: String) -> String
substring(s: String, start: Int, len: Int) -> String
```

#### Conversion Functions

```ssl
int_to_string(n: Int) -> String
float_to_string(f: Float) -> String
string_to_int(s: String) -> Int
string_to_float(s: String) -> Float
```

#### Math Functions

```ssl
abs(n: Int) -> Int
pow(base: Float, exp: Float) -> Float
sqrt(n: Float) -> Float
floor(n: Float) -> Int
ceil(n: Float) -> Int
```

### Example Usage

```ssl
fn main() -> Int {
    let name = read_line()
    print("Hello, " + name + "!")
    
    let num = 42
    let text = int_to_string(num)
    println(text)
    
    return 0
}
```

---

## Memory Model

### Stack Allocation

Local variables are stack-allocated:

```ssl
fn example() -> Int {
    let x = 42      // Stack-allocated
    let y = 3.14    // Stack-allocated
    return x
    // x and y automatically deallocated
}
```

### String Allocation

Strings are heap-allocated with automatic management:

```ssl
fn example() -> String {
    let text = "Hello, World!"  // Heap-allocated
    return text
    // Memory managed automatically
}
```

### Function Call Stack

```ssl
fn main() -> Int {
    let a = foo(10)  // Stack frame: main
    return a
}

fn foo(x: Int) -> Int {  // Stack frame: foo
    let y = bar(x * 2)
    return y
}

fn bar(z: Int) -> Int {  // Stack frame: bar
    return z + 5
}
```

**Stack Layout:**
```
[main] -> [foo] -> [bar]
```

---

## Compilation Model

### Compile-Time vs Runtime

**Compile-Time:**
- Type checking
- Constant folding
- Dead code elimination

**Runtime:**
- Variable assignment
- Function calls
- I/O operations

### ABI Compliance

SSL v7.0 follows platform ABIs:
- **Windows**: Microsoft x64 calling convention
- **Linux/macOS**: System V AMD64 ABI
- **ARM64**: AAPCS64

---

## Advanced Topics

### External Functions

```ssl
extern fn c_function(x: Int) -> Int

fn main() -> Int {
    let result = c_function(42)
    return result
}
```

### Inline Assembly (Planned for v7.1)

```ssl
fn low_level() -> Int {
    asm {
        "mov rax, 42"
        "ret"
    }
}
```

---

## Version Differences

### SSL v7.0 vs v5.0

| Feature | v5.0 | v7.0 |
|---------|------|------|
| Compilation | Bytecode | Native x64 |
| Pattern Matching | ✅ | ⚠️ Planned v7.1 |
| Generics | ✅ | ⚠️ Planned v7.2 |
| Algebraic Effects | ✅ | ⚠️ Planned v7.3 |
| Multi-Architecture | ❌ | ✅ |
| Performance | 9x Python | 25x Python |

---

**Next**: [Compiler Guide →](COMPILER_GUIDE.md)

**See Also**:
- [Getting Started](GETTING_STARTED.md)
- [Examples](../examples/)
- [Standard Library API](STDLIB.md)
