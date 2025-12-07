# SSL 3.1 Performance Benchmarks

## Overview
This document outlines the performance benchmarking strategy for the Sonner Studio Language (SSL) v3.1, specifically focusing on the new **LLVM-based JIT Compiler**.

The goal is to verify the performance improvements achieved by compiling SSL code to native machine code versus interpreting the AST directly.

## Methodology

We compare two execution modes:
1.  **Interpreter (Baseline):** Tree-walk interpreter traversing the AST.
2.  **LLVM JIT (Aurora):** Compiles AST to LLVM IR, optimizes it, and executes native machine code.

### Test Cases

#### 1. Factorial (Recursive)
Calculates `factorial(20)`.
-   **Focus:** Function call overhead and recursion.
-   **Code:**
    ```rust
    fn factorial(n: Int) -> Int {
        if n <= 1 { return 1 }
        return n * factorial(n - 1)
    }
    ```

#### 2. Fibonacci (Recursive)
Calculates `fib(30)`.
-   **Focus:** Heavy branching and recursion.
-   **Code:**
    ```rust
    fn fib(n: Int) -> Int {
        if n <= 1 { return n }
        return fib(n - 1) + fib(n - 2)
    }
    ```

#### 3. Arithmetic Loop
Performs 1,000,000 iterations of simple arithmetic.
-   **Focus:** Loop optimization and variable access.

## Expected Results

Based on the architecture (Tree-walk vs Native Code), we expect the following performance characteristics:

| Benchmark | Interpreter (Est.) | LLVM JIT (Est.) | Speedup Target |
| :--- | :--- | :--- | :--- |
| **Factorial(20)** | ~0.5 ms | ~0.05 ms | **10x** |
| **Fibonacci(30)** | ~150 ms | ~15 ms | **10x** |
| **Loop (1M)** | ~200 ms | ~20 ms | **10x** |

## How to Run

The benchmark suite is implemented as a separate binary in `src/bin/benchmark_jit.rs`.

### Prerequisites
-   Rust Toolchain
-   LLVM 17.0 Development Libraries (libxml2s.lib, etc.)

### Command
```bash
cargo run --release --bin benchmark_jit --features llvm
```

## Current Status (v3.1)

-   ✅ **Implementation:** Benchmark code is fully implemented and compiles.
-   ✅ **JIT Backend:** LLVM backend (Phase 1-3) is fully integrated.
-   ⚠️ **Execution:** Currently blocked by a linking error on the host environment (`LNK1181: libxml2s.lib`).
-   **Next Step:** Fix host environment to verify the expected speedup.
