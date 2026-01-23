// SSL 3.0 Performance Benchmarks
// Compare Interpreter vs JIT-compiled native execution

use std::time::Instant;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

/// Benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub interpreter_time_ms: f64,
    pub jit_time_ms: f64,
    pub speedup: f64,
}

/// Run a benchmark comparing interpreter vs JIT
pub fn benchmark(name: &str, source: &str, iterations: usize) -> BenchmarkResult {
    // Warm up + compile
    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Failed to parse");
    
    // Benchmark interpreter
    let start = Instant::now();
    for _ in 0..iterations {
        let mut interp = Interpreter::new();
        let _ = interp.interpret(ast.clone());
    }
    let interpreter_time = start.elapsed().as_secs_f64() * 1000.0;
    
    // Benchmark JIT (mock - same as interpreter for now)
    let start = Instant::now();
    for _ in 0..iterations {
        let mut interp = Interpreter::new();
        let _ = interp.interpret(ast.clone());
    }
    let jit_time = start.elapsed().as_secs_f64() * 1000.0;
    
    let speedup = interpreter_time / jit_time.max(0.001);
    
    BenchmarkResult {
        name: name.to_string(),
        interpreter_time_ms: interpreter_time,
        jit_time_ms: jit_time,
        speedup,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bench_factorial() {
        let source = r#"
            fn factorial(n: Int) -> Int {
                if n <= 1 {
                    return 1
                }
                return n * factorial(n - 1)
            }
            
            factorial(10)
        "#;
        
        let result = benchmark("factorial", source, 100);
        println!("Factorial Benchmark:");
        println!("  Interpreter: {:.3}ms", result.interpreter_time_ms);
        println!("  JIT: {:.3}ms", result.jit_time_ms);
        println!("  Speedup: {:.2}x", result.speedup);
        
        // For now, speedup should be ~1x since we're not really JIT-compiling yet
        assert!(result.speedup > 0.5 && result.speedup < 2.0);
    }
    
    #[test]
    fn bench_fibonacci() {
        let source = r#"
            fn fib(n: Int) -> Int {
                if n <= 1 {
                    return n
                }
                return fib(n - 1) + fib(n - 2)
            }
            
            fib(15)
        "#;
        
        let result = benchmark("fibonacci", source, 10);
        println!("Fibonacci Benchmark:");
        println!("  Interpreter: {:.3}ms", result.interpreter_time_ms);
        println!("  JIT: {:.3}ms", result.jit_time_ms);
        println!("  Speedup: {:.2}x", result.speedup);
    }
    
    #[test]
    fn bench_map_operations() {
        let source = r#"
            let map = {"a": 1, "b": 2, "c": 3}
            let result = map_with(map, "d", 4)
            result
        "#;
        
        let result = benchmark("map_operations", source, 1000);
        println!("Map Operations Benchmark:");
        println!("  Interpreter: {:.3}ms", result.interpreter_time_ms);
        println!("  JIT: {:.3}ms", result.jit_time_ms);
        println!("  Speedup: {:.2}x", result.speedup);
    }
}
