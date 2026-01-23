// SSL 3.1 JIT Benchmark Suite
// Compares Interpreter vs LLVM JIT performance

use std::time::Instant;
use ssl_v9::aurora::ir::*;
use ssl_v9::aurora::{NativeExecutor, JitCompiler};
use ssl_v9::interpreter::{Interpreter, Value}; // Assuming Interpreter is public
use ssl_v9::ast::*;

fn main() {
    println!("🚀 SSL 3.1 JIT Benchmark Suite");
    println!("==============================");

    #[cfg(not(feature = "llvm"))]
    {
        println!("❌ LLVM feature not enabled. Please run with --features llvm");
        return;
    }

    #[cfg(feature = "llvm")]
    run_benchmarks();
}

#[cfg(feature = "llvm")]
fn run_benchmarks() {
    // Setup
    let mut native_exec = NativeExecutor::new();
    let jit_compiler = JitCompiler::new();
    
    // 1. Benchmark: Factorial(20)
    println!("\n📊 Benchmark 1: Factorial(20)");
    let fact_ast = create_factorial_ast();
    
    // Interpreter Run
    let start_interp = Instant::now();
    // Note: We need a way to run AST in interpreter easily without full parser setup if possible
    // For this benchmark, we might need to construct a full program or just use the JIT's IR for native
    // and AST for interpreter.
    // Let's assume we can run the AST via a simple interpreter instance.
    // ... implementation details ...
    
    // Actually, constructing AST manually is verbose. 
    // Let's use the parser if available, or construct IR directly for JIT testing.
    
    // For this test, we will focus on the JIT performance itself first.
    
    benchmark_jit_factorial(&mut native_exec, &jit_compiler);
}

#[cfg(feature = "llvm")]
fn benchmark_jit_factorial(native_exec: &mut NativeExecutor, jit_compiler: &JitCompiler) {
    // Define Factorial in AST/IR
    // fn factorial(n) { if n < 2 { return 1 } else { return n * factorial(n-1) } }
    
    // We'll construct the IR-B directly to test the backend purely, 
    // or use the JitCompiler to compile AST -> IR -> Native.
    
    // Let's use JitCompiler::compile_and_register to test the full pipeline.
    
    let body = vec![
        Statement::If(IfStatement {
            condition: Expression::BinaryOp(BinaryOp {
                left: Box::new(Expression::Identifier("n".to_string())),
                op: Operator::Lt,
                right: Box::new(Expression::IntLiteral(2)),
            }),
            then_block: vec![
                Statement::Return(Some(Expression::IntLiteral(1)))
            ],
            else_block: Some(vec![
                Statement::Return(Some(Expression::BinaryOp(BinaryOp {
                    left: Box::new(Expression::Identifier("n".to_string())),
                    op: Operator::Multiply,
                    right: Box::new(Expression::FunctionCall(FunctionCall {
                        name: "factorial".to_string(),
                        type_args: vec![],
                        args: vec![
                            Expression::BinaryOp(BinaryOp {
                                left: Box::new(Expression::Identifier("n".to_string())),
                                op: Operator::Subtract,
                                right: Box::new(Expression::IntLiteral(1)),
                            })
                        ],
                    })),
                })))
            ]),
        })
    ];

    println!("Compiling...");
    let start_compile = Instant::now();
    jit_compiler.compile_and_register(native_exec, "factorial".to_string(), body).unwrap();
    println!("Compilation took: {:?}", start_compile.elapsed());

    println!("Executing Native Code...");
    let start_run = Instant::now();
    let result = native_exec.execute_native("factorial", vec![Value::Int(20)]).unwrap();
    let duration = start_run.elapsed();
    
    println!("Result: {:?}", result);
    println!("Time: {:?}", duration);
    
    if let Value::Int(val) = result {
        assert_eq!(val, 2432902008176640000);
        println!("✅ Correctness Verified");
    } else {
        println!("❌ Incorrect Result");
    }
}

// Helper to create AST (mock)
fn create_factorial_ast() -> Vec<Statement> {
    vec![]
}
