// JIT Compiler Manager
// Manages hot-path detection and compilation caching

use crate::aurora::{Compiler as AuroraCompiler, LLVMGenerator};
use crate::ast::Statement;
use crate::telemetry::{FunctionStats, Metric};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Holds compiled LLVM IR for a function
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub llvm_ir: String,
    pub compilation_timestamp: u64,
}

/// JIT Compiler Manager
/// Detects hot paths and triggers compilation
#[derive(Clone)]
pub struct JitCompiler {
    compiled_cache: Arc<Mutex<HashMap<String, CompiledFunction>>>,
    hot_path_threshold_calls: u64,
    hot_path_threshold_avg_time_us: u64,
}

impl JitCompiler {
    pub fn new() -> Self {
        JitCompiler {
            compiled_cache: Arc::new(Mutex::new(HashMap::new())),
            hot_path_threshold_calls: 100,        // Compile after 100 calls
            hot_path_threshold_avg_time_us: 1000, // Or avg 1ms execution
        }
    }

    /// Check if function should be JIT compiled based on telemetry
    pub fn should_compile(&self, stats: &FunctionStats) -> bool {
        stats.total_calls >= self.hot_path_threshold_calls
            || stats.avg_execution_time_us >= self.hot_path_threshold_avg_time_us
    }

    /// Check if function is already compiled
    pub fn is_compiled(&self, function_name: &str) -> bool {
        let cache = self.compiled_cache.lock().unwrap();
        cache.contains_key(function_name)
    }

    /// Get compiled LLVM IR for function
    pub fn get_compiled(&self, function_name: &str) -> Option<CompiledFunction> {
        let cache = self.compiled_cache.lock().unwrap();
        cache.get(function_name).cloned()
    }

    /// Compile function to LLVM IR and cache it
    pub fn compile_function(
        &self,
        function_name: String,
        function_body: Vec<Statement>,
    ) -> Result<String, String> {
        // Create Aurora compiler
        let mut compiler = AuroraCompiler::new(&function_name);
        
        // Compile AST to IR-B
        let mut module = compiler.compile(function_body);
        
        // SSL 3.0: Run optimization passes
        let mut optimizer = crate::aurora::optimizer::Optimizer::new();
        optimizer.optimize(&mut module);
        
        // Generate LLVM IR
        let mut llvm_gen = LLVMGenerator::new();
        let llvm_ir = llvm_gen.generate(&module);
        
        // Cache the compiled function
        let compiled = CompiledFunction {
            name: function_name.clone(),
            llvm_ir: llvm_ir.clone(),
            compilation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        };
        
        let mut cache = self.compiled_cache.lock().unwrap();
        cache.insert(function_name, compiled);
        
        Ok(llvm_ir)
    }

    /// Compile function and register with NativeExecutor (SSL 3.0)
    pub fn compile_and_register(
        &self,
        native_executor: &mut crate::aurora::NativeExecutor,
        function_name: String,
        function_body: Vec<Statement>,
    ) -> Result<(), String> {
        // Create Aurora compiler
        let mut compiler = AuroraCompiler::new(&function_name);
        
        // Compile AST to IR-B
        let mut module = compiler.compile(function_body);
        
        // SSL 3.0: Run optimization passes
        let mut optimizer = crate::aurora::optimizer::Optimizer::new();
        optimizer.optimize(&mut module);
        
        // Register with NativeExecutor (which handles LLVM compilation)
        native_executor.compile_function(function_name.clone(), &module)?;
        
        // Also generate LLVM IR string for cache/debugging
        let mut llvm_gen = LLVMGenerator::new();
        let llvm_ir = llvm_gen.generate(&module);
        
        let compiled = CompiledFunction {
            name: function_name,
            llvm_ir,
            compilation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        };
        
        let mut cache = self.compiled_cache.lock().unwrap();
        cache.insert(compiled.name.clone(), compiled);
        
        Ok(())
    }

    /// Get all compiled functions
    pub fn get_all_compiled(&self) -> Vec<CompiledFunction> {
        let cache = self.compiled_cache.lock().unwrap();
        cache.values().cloned().collect()
    }

    /// Clear compilation cache
    pub fn clear_cache(&self) {
        let mut cache = self.compiled_cache.lock().unwrap();
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_path_detection() {
        let jit = JitCompiler::new();
        
        // Cold path - few calls
        let cold_stats = FunctionStats {
            name: "cold_func".to_string(),
            total_calls: 10,
            total_execution_time_us: 1000,
            avg_execution_time_us: 100,
        };
        assert!(!jit.should_compile(&cold_stats));
        
        // Hot path - many calls
        let hot_stats = FunctionStats {
            name: "hot_func".to_string(),
            total_calls: 150,
            total_execution_time_us: 15000,
            avg_execution_time_us: 100,
        };
        assert!(jit.should_compile(&hot_stats));
        
        // Hot path - slow execution
        let slow_stats = FunctionStats {
            name: "slow_func".to_string(),
            total_calls: 10,
            total_execution_time_us: 20000,
            avg_execution_time_us: 2000,
        };
        assert!(jit.should_compile(&slow_stats));
    }
    
    #[test]
    fn test_compilation_cache() {
        let jit = JitCompiler::new();
        
        assert!(!jit.is_compiled("test_func"));
        
        // Simulate compilation
        let body = vec![];
        jit.compile_function("test_func".to_string(), body).ok();
        
        assert!(jit.is_compiled("test_func"));
        
        let compiled = jit.get_compiled("test_func");
        assert!(compiled.is_some());
        assert_eq!(compiled.unwrap().name, "test_func");
    }
}
