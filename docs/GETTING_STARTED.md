# Getting Started with SSL v7.0

Welcome to Sonner Studio Language v7.0! This guide will help you get up and running with SSL's native compilation features.

## Table of Contents

1. [Installation](#installation)
2. [Your First Program](#your-first-program)
3. [Compiling and Running](#compiling-and-running)
4. [Language Basics](#language-basics)
5. [Next Steps](#next-steps)

## Installation

### Prerequisites

**Windows:**
```powershell
# Install Rust (for bootstrapping)
winget install Rustlang.Rustup

# Install NASM
winget install NASM.NASM

# Install Visual Studio Build Tools (for linker)
# Download from Microsoft website
```

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install build-essential nasm

# For ARM64 cross-compilation
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install NASM via Homebrew
brew install nasm
```

### Installing SSL v7.0

```bash
# Clone the repository
git clone https://github.com/SonnerStudio/SSL.git
cd SSL

# Build and install
cargo install --path .

# Verify installation
ssl --version
# Output: SSL v7.0.0
```

## Your First Program

Create a file named `hello.ssl`:

```ssl
fn main() -> Int {
    print("Hello, SSL v7.0!")
    return 0
}
```

**Explanation:**
- `fn main() -> Int` - Every SSL program starts with a `main` function returning an Int
- `print(...)` - Built-in function for output
- `return 0` - Exit code (0 = success)

## Compiling and Running

### Quick Run (Interpret & Execute)

```bash
ssl run hello.ssl
```

Output:
```
Hello, SSL v7.0!
```

### Compile to Native

```bash
# Compile to assembly
ssl compile hello.ssl

# This generates:
# - hello.asm (NASM assembly)
# - hello.o (object file)
# - hello.exe/hello (executable)

# Run the compiled program
./hello
```

### Multi-Architecture Compilation

```bash
# Compile for ARM64
ssl compile --arch arm64 hello.ssl

# Compile for Apple Silicon
ssl compile --arch apple_m hello.ssl

# Compile for specific platforms
ssl compile --target x86_64-linux hello.ssl
```

## Language Basics

### Variables

```ssl
fn main() -> Int {
    let x = 42              // Immutable
    let mut y = 10          // Mutable
    
    y = y + 5
    print(int_to_string(y)) // Output: 15
    
    return 0
}
```

### Functions

```ssl
fn add(a: Int, b: Int) -> Int {
    return a + b
}

fn main() -> Int {
    let result = add(5, 3)
    print(int_to_string(result)) // Output: 8
    return 0
}
```

### Control Flow

```ssl
fn main() -> Int {
    let x = 10
    
    // If-else
    if x > 5 {
        print("Greater than 5")
    } else {
        print("Less than or equal to 5")
    }
    
    // While loop
    let mut i = 0
    while i < 5 {
        print(int_to_string(i))
        i = i + 1
    }
    
    return 0
}
```

### Types

SSL v7.0 supports the following basic types:

- `Int` - 64-bit signed integer
- `Float` - 64-bit floating point
- `String` - UTF-8 string
- `Bool` - Boolean (true/false)

```ssl
fn main() -> Int {
    let integer: Int = 42
    let decimal: Float = 3.14
    let text: String = "Hello"
    let flag: Bool = true
    
    return 0
}
```

## Interactive REPL

SSL v7.0 includes an interactive Read-Eval-Print Loop:

```bash
ssl repl
```

```
SSL v7.0 REPL
Type 'exit' to quit

> let x = 42
> x + 10
52
> fn double(n: Int) -> Int { return n * 2 }
> double(21)
42
```

## Project Structure

For larger projects, organize your code:

```
my_project/
├── src/
│   ├── main.ssl          # Entry point
│   ├── utils.ssl         # Utility functions
│   └── types.ssl         # Type definitions
├── tests/
│   └── test_main.ssl     # Tests
└── ssl.toml              # Project configuration
```

**ssl.toml:**
```toml
[package]
name = "my_project"
version = "1.0.0"
authors = ["Your Name"]

[dependencies]
# Future: package dependencies

[build]
optimization = "release"
target = "x86_64"
```

## Debugging

```bash
# Compile with debug symbols
ssl compile --debug hello.ssl

# View generated assembly
ssl compile --emit asm hello.ssl
cat hello.asm

# Check syntax without compiling
ssl check hello.ssl
```

## Common Errors

### 1. Type Mismatch
```ssl
fn main() -> Int {
    return "string"  // ERROR: Expected Int, got String
}
```

**Solution**: Ensure return type matches function signature.

### 2. Undefined Function
```ssl
fn main() -> Int {
    foo()  // ERROR: Function 'foo' not found
    return 0
}
```

**Solution**: Define the function or check spelling.

### 3. Immutable Assignment
```ssl
fn main() -> Int {
    let x = 10
    x = 20  // ERROR: Cannot assign to immutable variable
    return 0
}
```

**Solution**: Use `let mut x = 10` for mutable variables.

## Next Steps

- **Language Reference**: Learn advanced syntax in [LANGUAGE_REFERENCE.md](LANGUAGE_REFERENCE.md)
- **Compiler Guide**: Understand compilation in [COMPILER_GUIDE.md](COMPILER_GUIDE.md)
- **Examples**: See more examples in [examples/](../examples/)
- **Operating System Development**: Build OS kernels in [OS_DEV.md](OS_DEV.md)

## Getting Help

- **Documentation**: [docs/](../docs/)
- **Discord**: [Join our community](https://discord.gg/sonnerstudio)
- **GitHub Issues**: [Report bugs](https://github.com/SonnerStudio/SSL/issues)

---

**Welcome to the SSL community!** 🚀

Next: [Language Reference →](LANGUAGE_REFERENCE.md)
