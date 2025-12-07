//! SSL 4.0 WebAssembly Module
//!
//! Enables SSL programs to run in web browsers via WebAssembly.
//! Uses LLVM's WebAssembly backend for optimized code generation.

pub mod backend;
pub mod runtime;
pub mod js_interop;
pub mod bindings;

use crate::ast::TargetPlatform;

/// WebAssembly target configuration
#[derive(Debug, Clone)]
pub struct WasmTarget {
    /// Target triple for WASM
    pub triple: &'static str,
    /// Enable SIMD instructions
    pub enable_simd: bool,
    /// Enable threads (SharedArrayBuffer)
    pub enable_threads: bool,
    /// Enable bulk memory operations
    pub enable_bulk_memory: bool,
    /// Optimization level (0-3)
    pub opt_level: u8,
}

impl Default for WasmTarget {
    fn default() -> Self {
        Self {
            triple: "wasm32-unknown-unknown",
            enable_simd: false,
            enable_threads: false,
            enable_bulk_memory: true,
            opt_level: 3,
        }
    }
}

impl WasmTarget {
    /// Create a new WASM target with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Enable SIMD instructions for better performance
    pub fn with_simd(mut self) -> Self {
        self.enable_simd = true;
        self
    }
    
    /// Enable threading support (requires SharedArrayBuffer)
    pub fn with_threads(mut self) -> Self {
        self.enable_threads = true;
        self
    }
    
    /// Set optimization level
    pub fn with_opt_level(mut self, level: u8) -> Self {
        self.opt_level = level.min(3);
        self
    }
    
    /// Get LLVM target features string
    pub fn target_features(&self) -> String {
        let mut features = Vec::new();
        
        if self.enable_simd {
            features.push("+simd128");
        }
        if self.enable_threads {
            features.push("+atomics");
            features.push("+bulk-memory");
        }
        if self.enable_bulk_memory {
            features.push("+bulk-memory");
        }
        
        features.join(",")
    }
}

/// WASM module output format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WasmOutput {
    /// Raw .wasm binary
    Binary,
    /// WebAssembly Text format (.wat)
    Text,
    /// JavaScript + WASM bundle
    JsBundle,
    /// ES Module with WASM
    EsModule,
}

/// Result of WASM compilation
#[derive(Debug)]
pub struct WasmModule {
    /// Compiled WASM binary
    pub binary: Vec<u8>,
    /// JavaScript bindings (if generated)
    pub js_bindings: Option<String>,
    /// TypeScript declarations (if generated)
    pub ts_declarations: Option<String>,
    /// HTML template (if generated)
    pub html_template: Option<String>,
    /// Exported functions
    pub exports: Vec<WasmExport>,
}

/// An exported function from the WASM module
#[derive(Debug, Clone)]
pub struct WasmExport {
    /// Function name
    pub name: String,
    /// Parameter types
    pub params: Vec<WasmType>,
    /// Return type
    pub return_type: Option<WasmType>,
}

/// WebAssembly value types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,      // SIMD
    FuncRef,
    ExternRef,
}

impl WasmType {
    /// Convert from SSL type name
    pub fn from_ssl_type(type_name: &str) -> Option<Self> {
        match type_name {
            "Int" | "int" | "i32" => Some(WasmType::I32),
            "Int64" | "i64" => Some(WasmType::I64),
            "Float" | "float" | "f32" => Some(WasmType::F32),
            "Float64" | "f64" => Some(WasmType::F64),
            "Bool" | "bool" => Some(WasmType::I32), // Booleans as i32
            _ => None,
        }
    }
    
    /// Get WASM type name
    pub fn wasm_name(&self) -> &'static str {
        match self {
            WasmType::I32 => "i32",
            WasmType::I64 => "i64",
            WasmType::F32 => "f32",
            WasmType::F64 => "f64",
            WasmType::V128 => "v128",
            WasmType::FuncRef => "funcref",
            WasmType::ExternRef => "externref",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasm_target_default() {
        let target = WasmTarget::new();
        assert_eq!(target.triple, "wasm32-unknown-unknown");
        assert_eq!(target.opt_level, 3);
    }
    
    #[test]
    fn test_wasm_type_conversion() {
        assert_eq!(WasmType::from_ssl_type("Int"), Some(WasmType::I32));
        assert_eq!(WasmType::from_ssl_type("Float"), Some(WasmType::F32));
        assert_eq!(WasmType::from_ssl_type("Bool"), Some(WasmType::I32));
    }
    
    #[test]
    fn test_target_features() {
        let target = WasmTarget::new().with_simd().with_threads();
        let features = target.target_features();
        assert!(features.contains("+simd128"));
        assert!(features.contains("+atomics"));
    }
}
