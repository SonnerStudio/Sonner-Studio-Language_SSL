// Aurora Native Executor - JIT Compilation to Native Code
// Phase 4B: Mock implementation with performance framework

use crate::aurora::ir::*;
use crate::interpreter::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Statistics for native execution
#[derive(Debug, Clone)]
pub struct NativeStats {
    pub compile_time: Duration,
    pub execution_count: u64,
    pub total_execution_time: Duration,
    pub avg_execution_time_ns: u64,
}

/// Native function pointer type
/// In real implementation: extern "C" fn(...) -> Value
pub type NativeFn = fn(Vec<Value>) -> Result<Value, String>;

/// Native code executor
pub struct NativeExecutor {
    /// Compiled native functions (function pointer cache)
    native_cache: HashMap<String, NativeFn>,
    
    /// Performance statistics per function
    stats: HashMap<String, NativeStats>,
    
    /// Compilation enabled flag
    enabled: bool,
    
    /// LLVM Context (Owner)
    #[cfg(feature = "llvm")]
    llvm_context: Option<Box<inkwell::context::Context>>,
    
    /// LLVM Backend (Dependent)
    /// We use 'static lifetime here via unsafe cast to allow self-referential struct
    #[cfg(feature = "llvm")]
    llvm_backend: Option<Box<crate::aurora::llvm_backend::LLVMBackend<'static>>>,
}

impl NativeExecutor {
    pub fn new() -> Self {
        NativeExecutor {
            native_cache: HashMap::new(),
            stats: HashMap::new(),
            enabled: true,
            #[cfg(feature = "llvm")]
            llvm_context: None,
            #[cfg(feature = "llvm")]
            llvm_backend: None,
        }
    }
    
    /// Initialize LLVM backend if needed
    #[cfg(feature = "llvm")]
    fn init_llvm(&mut self) -> Result<(), String> {
        if self.llvm_context.is_some() {
            return Ok(());
        }
        
        let context = Box::new(inkwell::context::Context::create());
        
        // Create backend with unsafe lifetime extension
        // This is safe because llvm_backend is dropped before llvm_context
        // (fields dropped in declaration order: llvm_context is declared BEFORE llvm_backend... WAIT)
        // FIX: We need llvm_backend to be dropped BEFORE llvm_context.
        // So llvm_backend must be declared BEFORE llvm_context?
        // Rust drops fields in declaration order.
        // 1. llvm_context drops.
        // 2. llvm_backend drops. -> BAD if backend needs context.
        //
        // We need to ensure backend is dropped first.
        // We can do this in Drop impl, or reorder fields.
        // Let's reorder fields in the struct definition above? 
        // No, I can't easily change the struct definition order in this tool call without replacing the whole struct.
        // I will implement Drop to manually drop backend first.
        
        let context_ref: &'static inkwell::context::Context = unsafe {
            std::mem::transmute(&*context)
        };
        
        let backend = crate::aurora::llvm_backend::LLVMBackend::new(context_ref, "jit_module")?;
        
        self.llvm_context = Some(context);
        self.llvm_backend = Some(Box::new(backend));
        
        Ok(())
    }

    #[cfg(feature = "llvm")]
    pub fn compile_with_llvm(&mut self, func: &Function) -> Result<(), String> {
        self.init_llvm()?;
        
        if let Some(backend) = &mut self.llvm_backend {
            backend.compile_function(func)?;
            Ok(())
        } else {
            Err("LLVM backend not initialized".to_string())
        }
    }
    
    #[cfg(feature = "llvm")]
    pub fn execute_with_llvm(&mut self, func_name: &str, _args: Vec<Value>) -> Result<Value, String> {
        // Note: Arguments are currently ignored in this phase, assuming 0-arg function or fixed args
        // In real implementation, we need to map Value to LLVM args
        
        if let Some(backend) = &self.llvm_backend {
            // Unsafe call to JIT code
            let result: i64 = unsafe { backend.execute_function(func_name)? };
            Ok(Value::Int(result))
        } else {
            Err("LLVM backend not initialized".to_string())
        }
    }

    /// Check if a function has been compiled to native code
    pub fn has_native(&self, func_name: &str) -> bool {
        self.native_cache.contains_key(func_name)
    }
    
    /// Compile Aurora IR-B to native code
    pub fn compile_function(
        &mut self,
        func_name: String,
        module: &Module,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err("Native execution disabled".to_string());
        }
        
        let start = Instant::now();
        
        // Try LLVM compilation if enabled
        #[cfg(feature = "llvm")]
        {
            // Find function in module
            if let Some(func) = module.functions.iter().find(|f| f.name == func_name) {
                match self.compile_with_llvm(func) {
                    Ok(_) => {
                        // Success!
                    },
                    Err(e) => {
                        // Fallback to mock
                        eprintln!("LLVM compilation failed: {}", e);
                    }
                }
            }
        }
        
        // Mock compilation (fallback or primary if no LLVM)
        let native_fn: NativeFn = |_args| {
            Ok(Value::Int(42))
        };
        
        self.native_cache.insert(func_name.clone(), native_fn);
        
        let compile_time = start.elapsed();
        
        // Record compilation stats
        self.stats.insert(func_name, NativeStats {
            compile_time,
            execution_count: 0,
            total_execution_time: Duration::ZERO,
            avg_execution_time_ns: 0,
        });
        
        Ok(())
    }
    
    /// Execute a native function
    pub fn execute_native(
        &mut self,
        func_name: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        // Try LLVM execution first if available
        #[cfg(feature = "llvm")]
        {
            if self.llvm_backend.is_some() {
                // Check if function exists in LLVM module (simple check)
                // In reality we should track which functions are compiled where
                // For Phase 3, we assume if backend is init, we try it
                match self.execute_with_llvm(func_name, args.clone()) {
                    Ok(val) => return Ok(val),
                    Err(_) => {
                        // Function might not be in LLVM, fall through to mock
                    }
                }
            }
        }

        let native_fn = self.native_cache.get(func_name)
            .ok_or_else(|| format!("No native version of '{}'", func_name))?;
        
        let start = Instant::now();
        let result = native_fn(args)?;
        let elapsed = start.elapsed();
        
        // Update statistics
        if let Some(stats) = self.stats.get_mut(func_name) {
            stats.execution_count += 1;
            stats.total_execution_time += elapsed;
            stats.avg_execution_time_ns = 
                (stats.total_execution_time.as_nanos() / stats.execution_count as u128) as u64;
        }
        
        Ok(result)
    }
    
    /// Get performance statistics for a function
    pub fn get_stats(&self, func_name: &str) -> Option<&NativeStats> {
        self.stats.get(func_name)
    }
    
    /// Get all statistics
    pub fn all_stats(&self) -> &HashMap<String, NativeStats> {
        &self.stats
    }
    
    /// Enable/disable native execution
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Clear all compiled functions
    pub fn clear_cache(&mut self) {
        self.native_cache.clear();
        self.stats.clear();
        #[cfg(feature = "llvm")]
        {
            self.llvm_backend = None;
            self.llvm_context = None;
        }
    }
}

#[cfg(feature = "llvm")]
impl Drop for NativeExecutor {
    fn drop(&mut self) {
        // Ensure backend is dropped BEFORE context
        self.llvm_backend = None;
        self.llvm_context = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aurora::builder::Builder;
    
    #[test]
    fn test_native_executor_creation() {
        let executor = NativeExecutor::new();
        assert!(executor.enabled);
        assert_eq!(executor.native_cache.len(), 0);
    }
    
    #[test]
    fn test_mock_compilation() {
        let mut executor = NativeExecutor::new();
        let mut builder = Builder::new("test");
        builder.create_function("add", vec![], "Int");
        
        let module = builder.module;
        
        let result = executor.compile_function("add".to_string(), &module);
        assert!(result.is_ok());
        assert!(executor.has_native("add"));
    }
    
    #[test]
    fn test_statistics_tracking() {
        let mut executor = NativeExecutor::new();
        let mut builder = Builder::new("test");
        builder.create_function("test_fn", vec![], "Int");
        
        executor.compile_function("test_fn".to_string(), &builder.module).unwrap();
        
        // Execute multiple times
        for _ in 0..10 {
            let _ = executor.execute_native("test_fn", vec![]);
        }
        
        let stats = executor.get_stats("test_fn").unwrap();
        assert_eq!(stats.execution_count, 10);
        assert!(stats.avg_execution_time_ns > 0);
    }
}
