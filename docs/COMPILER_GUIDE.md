# SSL v7.0 Compiler Guide

Complete guide to the SSL v7.0 native compiler architecture and usage.

## Table of Contents

1. [Compiler Overview](#compiler-overview)
2. [Compilation Pipeline](#compilation-pipeline)
3. [Command-Line Interface](#command-line-interface)
4. [Optimization Levels](#optimization-levels)
5. [Cross-Compilation](#cross-compilation)
6. [Debugging](#debugging)
7. [Internals](#compiler-internals)

---

## Compiler Overview

SSL v7.0 features a production-grade native compiler that generates optimized x64 assembly code.

### Key Features

- **Native Compilation**: Direct x64 assembly output
- **Multi-Architecture**: x86_64, ARM64, Apple Silicon
- **AOT (Ahead-of-Time)**: No JIT, pure native code
- **SSA-Based IR**: Modern intermediate representation
- **Multi-Pass Optimizer**: Constant folding, DCE, inlining
- **ABI Compliant**: Windows x64, System V AMD64, AAPCS64

### Architecture

```
Source Code (.ssl)
    ↓
Lexer → Tokens
    ↓
Parser → AST
    ↓
Type Checker → Typed AST
    ↓
IR Generator → SSA IR
    ↓
Optimizer → Optimized IR
    ↓
Code Generator → Assembly (.asm)
    ↓
Assembler (NASM) → Object File (.o)
    ↓
Linker (ld) → Executable
```

---

## Compilation Pipeline

### Stage 1: Lexical Analysis

**Input**: Source code text  
**Output**: Token stream

```ssl
let x = 42 + 10
```

**Tokens**:
```
[LET, IDENTIFIER("x"), EQUALS, INT_LITERAL(42), PLUS, INT_LITERAL(10)]
```

### Stage 2: Parsing

**Input**: Token stream  
**Output**: Abstract Syntax Tree (AST)

**AST**:
```
VarDecl
├─ name: "x"
├─ type: Int
└─ value: BinaryOp
    ├─ op: PLUS
    ├─ left: 42
    └─ right: 10
```

### Stage 3: Type Checking

**Input**: AST  
**Output**: Typed AST

Verifies:
- Type compatibility
- Function signatures
- Variable scoping

### Stage 4: IR Generation

**Input**: Typed AST  
**Output**: SSA-form Intermediate Representation

```
%0 = const 42
%1 = const 10
%2 = add %0, %1
store x, %2
```

### Stage 5: Optimization

**Input**: IR  
**Output**: Optimized IR

Transformations:
- Constant folding: `42 + 10` → `52`
- Dead code elimination
- Common subexpression elimination

**Optimized IR**:
```
%0 = const 52
store x, %0
```

### Stage 6: Code Generation

**Input**: Optimized IR  
**Output**: x64 Assembly

```nasm
section .text
global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    
    mov QWORD [rbp-8], 52  ; x = 52
    
    mov rax, 0
    add rsp, 32
    pop rbp
    ret
```

---

## Command-Line Interface

### Basic Usage

```bash
# Compile and run
ssl run program.ssl

# Compile to executable
ssl compile program.ssl

# Check syntax only
ssl check program.ssl

# Show compiler version
ssl --version

# Help information
ssl --help
```

### Compile Options

```bash
# Specify output file
ssl compile program.ssl -o myprogram

# Emit assembly only
ssl compile --emit asm program.ssl

# Emit IR
ssl compile --emit ir program.ssl

# Debug build
ssl compile --debug program.ssl

# Release build (optimized)
ssl compile --release program.ssl
```

### Architecture Selection

```bash
# Compile for specific architecture
ssl compile --arch x86_64 program.ssl
ssl compile --arch arm64 program.ssl
ssl compile --arch apple_m program.ssl

# Specify target triple
ssl compile --target x86_64-linux-gnu program.ssl
ssl compile --target aarch64-linux-gnu program.ssl
```

---

## Optimization Levels

### -O0 (No Optimization)

Default for debug builds.

**Features**:
- No optimizations
- Fast compilation
- Easy debugging
- Preserve all variables

**Use For**: Development, debugging

### -O1 (Basic Optimization)

Light optimizations with minimal compile time impact.

**Features**:
- Constant folding
- Simple dead code elimination
- Basic inlining

**Use For**: Quick testing with some performance

### -O2 (Standard Optimization)

Default for release builds.

**Features**:
- Full constant propagation
- Dead code elimination
- Loop optimizations
- Inlining
- Register allocation optimization

**Use For**: Production builds

### -O3 (Aggressive Optimization)

Maximum performance, longer compile time.

**Features**:
- All -O2 optimizations
- Aggressive inlining
- Vectorization (SIMD)
- Function specialization

**Use For**: Performance-critical applications

**Example**:
```bash
ssl compile --release -O3 program.ssl
```

---

## Cross-Compilation

### Setup

**Install ARM64 Toolchain** (Linux/WSL):
```bash
sudo apt install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

### Cross-Compile for ARM64

```bash
# Compile for ARM64 from x86_64
ssl compile --arch arm64 program.ssl

# Output files:
# - program.asm (ARM64 assembly)
# - program.o (ARM64 object file)
# - program (ARM64 executable)
```

### Testing with QEMU

```bash
# Install QEMU
sudo apt install qemu-system-arm

# Test ARM64 binary
qemu-aarch64 ./program
```

### Cross-Compile for Apple Silicon

```bash
# From x86_64 to Apple M-series
ssl compile --arch apple_m --target aarch64-apple-darwin program.ssl
```

---

## Debugging

### Debug Symbols

```bash
# Compile with debug info
ssl compile --debug program.ssl
```

**Generates**:
- DWARF debug symbols
- Source line mapping
- Variable names preserved

### View Generated Assembly

```bash
ssl compile --emit asm program.ssl
cat program.asm
```

### View IR

```bash
ssl compile --emit ir program.ssl
cat program.ir
```

### Verbose Mode

```bash
ssl compile --verbose program.ssl
```

**Output**:
```
[LEXER] Processing tokens...
[PARSER] Building AST...
[TYPECHECK] Checking types...
[IR] Generating intermediate representation...
[OPTIMIZE] Running optimizations (pass 1/3)...
[CODEGEN] Generating x64 assembly...
[ASSEMBLE] Running NASM...
[LINK] Linking with runtime library...
[SUCCESS] Executable: program
```

---

## Compiler Internals

### Lexer

**File**: `src/lexer.ssl`

**Responsibilities**:
- Tokenize source code
- Handle string escapes
- Recognize keywords
- Support literals (hex, float, binary)

**Key Functions**:
```ssl
fn lex(source: String) -> List<Token>
fn next_token(lexer: Lexer) -> Token
```

### Parser

**File**: `src/parser.ssl`

**Algorithm**: Recursive descent with Pratt expression parsing

**Productions**:
```
program     ::= function*
function    ::= 'fn' IDENT '(' params ')' '->' type block
statement   ::= let_stmt | if_stmt | while_stmt | return_stmt
expression  ::= primary | binary_op | unary_op
```

### Type Checker

**File**: `src/types.ssl`

**Type Rules**:
```
Γ ⊢ e : τ   (Expression e has type τ in context Γ)

Γ ⊢ n : Int          (Integer literal)
Γ ⊢ 3.14 : Float     (Float literal)
Γ, x:τ ⊢ x : τ       (Variable)
```

### IR Generator

**File**: `src/ir.ssl`

**SSA Construction**:
```ssl
fn generate_ir(ast: AST) -> IR {
    // Convert AST to SSA form
    // φ-functions for merges
    // Basic block construction
}
```

### Optimizer

**File**: `src/optimize.ssl`

**Passes**:
1. **Constant Folding**: `2 + 3` → `5`
2. **Dead Code Elimination**: Remove unreachable code
3. **Common Subexpression Elimination**: Reuse computed values

### Code Generator

**File**: `src/codegen.ssl`

**Target**: x64 Assembly (NASM syntax)

**Register Allocation**:
- RAX: Return value, temporaries
- RBX-R15: General purpose (callee-saved)
- RDI, RSI, RDX, RCX, R8, R9: Function arguments

**Stack Frame**:
```
rbp + 16:  Argument 1
rbp + 8:   Argument 2
rbp + 0:   Saved RBP
rbp - 8:   Local variable 1
rbp - 16:  Local variable 2
...
```

---

## Runtime Library

SSL v7.0 includes a  minimal runtime library:

**Features**:
- Memory allocation (malloc/free wrappers)
- String operations
- I/O functions (print, read)
- Math functions

**Linkage**:
```bash
# Automatically linked during compilation
ssl compile program.ssl
# Links with ssl_runtime.lib
```

---

## Performance Tips

### 1. Use Release Builds

```bash
ssl compile --release -O3 program.ssl
```

### 2. Profile Your Code

```bash
# Use system profilers
perf record ./program
perf report
```

### 3. Minimize Allocations

```ssl
// Bad: New string allocation
let result = string_concat(a, b)

// Better: Reuse buffers (when API available)
```

### 4. Use Appropriate Types

```ssl
// Use Int for counters, not Float
let mut count: Int = 0  // Good
let mut count: Float = 0.0  // Bad (slower)
```

---

## Troubleshooting

### Common Errors

**1. NASM Not Found**
```
Error: NASM assembler not found in PATH
```
**Solution**: Install NASM and add to PATH

**2. Linker Error**
```
Error: undefined reference to `_start`
```
**Solution**: Ensure runtime library is linked

**3. Cross-Compiler Not Found**
```
Error: aarch64-linux-gnu-gcc not found
```
**Solution**: Install ARM64 toolchain

---

**Next**: [Operating System Development →](OS_DEV.md)

**See Also**:
- [Language Reference](LANGUAGE_REFERENCE.md)
- [Getting Started](GETTING_STARTED.md)
- [Optimization Guide](OPTIMIZATION.md)
