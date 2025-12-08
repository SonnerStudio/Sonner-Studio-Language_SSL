# SSL v6.0 Specification: The Ideal Modern Language

## Vision
SSL v6.0 aims to be the "ideal" programming language for the modern era (2025+), addressing the common shortcomings of existing languages (memory insecurity, "function coloring", incidental complexity) while treating AI and Concurrency as first-class citizens.

## 1. AI-Native Primitives
Most languages treat AI as a library. SSL v6.0 treats it as a primitive.

### 1.1 Tensor Types
Built-in n-dimensional array support with hardware acceleration (GPU/TPU) transparently handled by the runtime/kernel.
```ssl
// Compile-time shape checking where possible
let signal: Tensor<Float32, [1024]> = ...
let weights: Tensor<Float32, [1024, 512]> = ...

// Native operators for matrix multiplication
let output = signal @ weights 
```

### 1.2 Model Construct
A `model` is a special class-like construct optimized for stateful inference and parameter management.
```ssl
model LLM {
    // Parameters are distinct from regular fields
    param weights: Tensor<Float32, [D_MODEL, VOCAB_SIZE]>
    
    // built-in 'forward' method for inference
    fn forward(input: Tensor<Int32>) -> Tensor<Float32> {
        // ...
    }
}
```

## 2. Memory Safety (The "Owner-Borrow" Model)
To allow safe OS development without a Garbage Collector (GC), SSL v6.0 adopts a simplified Ownership model similar to Rust but with more ergonomic defaults.

*   **Ownership**: Every value has a single owner. When the owner goes out of scope, the value is dropped.
*   **Borrowing**: References (`&T`) are borrowed. The compiler ensures references do not outlive the owner.
*   **No Lifetime Annotations**: The compiler infers lifetimes in 99% of cases. Explicit lifetimes are only needed for complex structural sub-borrowing.

```ssl
fn process(data: String) { 
    // 'data' is moved here
}

let s = "hello"
process(s) // s is moved
// print(s) // Compile Error: Use of moved value
```

## 3. Structured Concurrency
Avoid "Function Coloring" (Async/Await) which splits languages into sync and async worlds.
SSL v6.0 uses **Green Threads** (fibers) managed by the runtime, similar to Go but with safer communication primitives.

*   **Blocking is cheap**: Blocking a fiber does not block an OS thread.
*   **Channels**: Typed message passing.

```ssl
spawn {
    // This runs in parallel
    perform_heavy_task()
}
```

## 4. Native Compiler Architecture
The compiler must produce standalone machine code (ASM -> Binary) to reduce dependency on massive toolchains (like LLVM) for the base OS.

*   **Target**: x86_64, ARM64, RISC-V.
*   **Output format**: ELF/PE or raw binary (for Kernel).
*   **Intermediate Representation (IR)**: A Linear SSA-based IR optimized for register allocation.

## 5. Modern Syntax Improvements
*   **No Semicolons**: Optional, newline terminates statements.
*   **Everything is an Expression**: `if`, `match`, blocks return values.
*   **Result<T, E>**: Error handling via algebraic types, no Exceptions.
*   **Unit Type `()`**: Replaces `void`.

## 6. Bare-Metal & Kernel Architecture (Novel Design)
### 6.1 Single Address Space (SASOS) Support
SSL v6.0 is designed for a **language-based operating system**.
*   **Software Fault Isolation (SFI)**: The compiler guarantees memory safety, allowing all processes to share a single 64-bit address space.
*   **Zero-Copy IPC**: Communication between processes is pointer passing (proven safe by the borrow checker).

### 6.2 AI-Driven Kernel Resources
Native support for replacing static kernel heuristics with trained models.
```ssl
// The kernel scheduler can be a Model
model NeuralScheduler {
    fn schedule(tasks: List<Task>) -> Task { ... }
}
```

*   `access(asm)` blocks: Inline assembly.
*   `#[no_std]`: Disable standard library (for Kernel).
*   `#[layout(packed)]`: Control memory layout.
