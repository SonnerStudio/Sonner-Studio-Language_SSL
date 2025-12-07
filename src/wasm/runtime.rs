//! SSL 4.0 WebAssembly Runtime
//!
//! Minimal runtime for SSL programs running in browsers.

use std::collections::HashMap;

/// WASM Memory allocator for string handling
pub struct WasmRuntime {
    /// Memory pointer (simulated heap)
    heap_pointer: u32,
    /// Allocated strings
    strings: HashMap<u32, String>,
    /// Memory size in pages (64KB each)
    memory_pages: u32,
}

impl WasmRuntime {
    /// Create a new runtime with specified memory pages
    pub fn new(memory_pages: u32) -> Self {
        Self {
            heap_pointer: 1024, // Start after null and reserved space
            strings: HashMap::new(),
            memory_pages,
        }
    }
    
    /// Total memory size in bytes
    pub fn memory_size(&self) -> u32 {
        self.memory_pages * 65536
    }
    
    /// Allocate memory on the heap
    pub fn alloc(&mut self, size: u32) -> u32 {
        let ptr = self.heap_pointer;
        self.heap_pointer += size;
        
        // Align to 8 bytes
        self.heap_pointer = (self.heap_pointer + 7) & !7;
        
        ptr
    }
    
    /// Store a string and return its pointer
    pub fn store_string(&mut self, s: String) -> (u32, u32) {
        let len = s.len() as u32;
        let ptr = self.alloc(len);
        self.strings.insert(ptr, s);
        (ptr, len)
    }
    
    /// Get a stored string
    pub fn get_string(&self, ptr: u32) -> Option<&String> {
        self.strings.get(&ptr)
    }
    
    /// Free allocated memory (simple bump allocator - no-op)
    pub fn free(&mut self, _ptr: u32) {
        // Simple allocator doesn't support freeing
    }
}

/// WASM Import function signatures
#[derive(Debug, Clone)]
pub struct WasmImport {
    /// Module name (e.g., "env", "js")
    pub module: String,
    /// Function name
    pub name: String,
    /// Import kind
    pub kind: WasmImportKind,
}

/// Types of WASM imports
#[derive(Debug, Clone)]
pub enum WasmImportKind {
    /// Function import
    Function {
        params: Vec<super::WasmType>,
        results: Vec<super::WasmType>,
    },
    /// Memory import
    Memory { min_pages: u32, max_pages: Option<u32> },
    /// Table import
    Table { min: u32, max: Option<u32> },
    /// Global import
    Global { value_type: super::WasmType, mutable: bool },
}

/// Standard SSL runtime imports for WASM
pub fn ssl_runtime_imports() -> Vec<WasmImport> {
    use super::WasmType::*;
    
    vec![
        // Console output
        WasmImport {
            module: "env".to_string(),
            name: "ssl_print".to_string(),
            kind: WasmImportKind::Function {
                params: vec![I32, I32], // ptr, len
                results: vec![],
            },
        },
        // Panic handler
        WasmImport {
            module: "env".to_string(),
            name: "ssl_panic".to_string(),
            kind: WasmImportKind::Function {
                params: vec![I32, I32], // ptr, len
                results: vec![],
            },
        },
        // Memory allocation (via JavaScript)
        WasmImport {
            module: "env".to_string(),
            name: "ssl_alloc".to_string(),
            kind: WasmImportKind::Function {
                params: vec![I32], // size
                results: vec![I32], // ptr
            },
        },
        // Memory deallocation
        WasmImport {
            module: "env".to_string(),
            name: "ssl_free".to_string(),
            kind: WasmImportKind::Function {
                params: vec![I32], // ptr
                results: vec![],
            },
        },
        // Get current time (milliseconds)
        WasmImport {
            module: "env".to_string(),
            name: "ssl_time_ms".to_string(),
            kind: WasmImportKind::Function {
                params: vec![],
                results: vec![I64],
            },
        },
        // Random number generation
        WasmImport {
            module: "env".to_string(),
            name: "ssl_random".to_string(),
            kind: WasmImportKind::Function {
                params: vec![],
                results: vec![F64],
            },
        },
    ]
}

/// Generate WASM memory section
pub fn memory_section(min_pages: u32, max_pages: Option<u32>) -> Vec<u8> {
    let mut section = Vec::new();
    
    // Number of memories (always 1 for now)
    section.push(0x01);
    
    // Limits type
    if let Some(max) = max_pages {
        section.push(0x01); // has max
        leb128_encode(&mut section, min_pages);
        leb128_encode(&mut section, max);
    } else {
        section.push(0x00); // no max
        leb128_encode(&mut section, min_pages);
    }
    
    // Section ID 5 = Memory
    let mut result = vec![0x05];
    leb128_encode(&mut result, section.len() as u32);
    result.extend(section);
    
    result
}

/// LEB128 encoding helper
fn leb128_encode(buffer: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buffer.push(byte);
        if value == 0 { break; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_creation() {
        let runtime = WasmRuntime::new(1);
        assert_eq!(runtime.memory_size(), 65536);
    }
    
    #[test]
    fn test_string_allocation() {
        let mut runtime = WasmRuntime::new(1);
        let (ptr, len) = runtime.store_string("Hello".to_string());
        
        assert!(ptr > 0);
        assert_eq!(len, 5);
        assert_eq!(runtime.get_string(ptr), Some(&"Hello".to_string()));
    }
    
    #[test]
    fn test_ssl_runtime_imports() {
        let imports = ssl_runtime_imports();
        assert!(!imports.is_empty());
        
        let print_import = imports.iter().find(|i| i.name == "ssl_print");
        assert!(print_import.is_some());
    }
}
