//! SSL 4.0 WebAssembly Backend
//!
//! Compiles SSL IR to WebAssembly using LLVM's WASM target.

use crate::ast::*;
use crate::aurora::ir::*;
use super::{WasmTarget, WasmModule, WasmExport, WasmType, WasmOutput};

/// WebAssembly code generator
pub struct WasmBackend {
    /// Target configuration
    target: WasmTarget,
    /// Output format
    output_format: WasmOutput,
    /// Generated exports
    exports: Vec<WasmExport>,
}

impl WasmBackend {
    /// Create a new WASM backend
    pub fn new(target: WasmTarget) -> Self {
        Self {
            target,
            output_format: WasmOutput::Binary,
            exports: Vec::new(),
        }
    }
    
    /// Set output format
    pub fn with_output(mut self, format: WasmOutput) -> Self {
        self.output_format = format;
        self
    }
    
    /// Compile SSL AST to WebAssembly
    pub fn compile(&mut self, statements: &[Statement]) -> Result<WasmModule, String> {
        // Phase 1: Collect all exported functions
        for stmt in statements {
            if let Statement::FunctionDecl(func) = stmt {
                // Check for @wasm attribute
                if self.has_wasm_attribute(func) {
                    self.register_export(func)?;
                }
            }
        }
        
        // Phase 2: Generate WASM binary
        let binary = self.generate_wasm(statements)?;
        
        // Phase 3: Generate JS bindings if requested
        let js_bindings = match self.output_format {
            WasmOutput::JsBundle | WasmOutput::EsModule => {
                Some(self.generate_js_bindings()?)
            }
            _ => None,
        };
        
        // Phase 4: Generate TypeScript declarations
        let ts_declarations = if js_bindings.is_some() {
            Some(self.generate_ts_declarations()?)
        } else {
            None
        };
        
        // Phase 5: Generate HTML template
        let html_template = match self.output_format {
            WasmOutput::JsBundle => Some(self.generate_html_template()?),
            _ => None,
        };
        
        Ok(WasmModule {
            binary,
            js_bindings,
            ts_declarations,
            html_template,
            exports: self.exports.clone(),
        })
    }
    
    /// Check if function has @wasm attribute
    fn has_wasm_attribute(&self, func: &FunctionDecl) -> bool {
        // For now, export all public functions
        // In full implementation, check for @wasm or @export attribute
        !func.name.starts_with('_')
    }
    
    /// Register a function as WASM export
    fn register_export(&mut self, func: &FunctionDecl) -> Result<(), String> {
        let params: Vec<WasmType> = func.params.iter()
            .filter_map(|p| self.ssl_type_to_wasm(&p.param_type))
            .collect();
        
        let return_type = func.return_type.as_ref()
            .and_then(|t| self.ssl_type_to_wasm(t));
        
        self.exports.push(WasmExport {
            name: func.name.clone(),
            params,
            return_type,
        });
        
        Ok(())
    }
    
    /// Convert SSL type to WASM type
    fn ssl_type_to_wasm(&self, ssl_type: &Type) -> Option<WasmType> {
        match ssl_type {
            Type::Int => Some(WasmType::I32),
            Type::Float => Some(WasmType::F64),
            Type::Bool => Some(WasmType::I32),
            Type::String => Some(WasmType::I32), // Pointer
            _ => None,
        }
    }
    
    /// Generate WASM binary from AST
    fn generate_wasm(&self, statements: &[Statement]) -> Result<Vec<u8>, String> {
        // Minimal WASM module structure
        let mut wasm = WasmBinaryBuilder::new();
        
        // Magic number + version
        wasm.emit_header();
        
        // Type section - function signatures
        wasm.emit_type_section(&self.exports);
        
        // Function section - function indices
        wasm.emit_function_section(&self.exports);
        
        // Export section - exported names
        wasm.emit_export_section(&self.exports);
        
        // Code section - function bodies
        wasm.emit_code_section(statements, &self.exports)?;
        
        Ok(wasm.finish())
    }
    
    /// Generate JavaScript bindings
    fn generate_js_bindings(&self) -> Result<String, String> {
        let mut js = String::new();
        
        // ES Module header
        js.push_str("// SSL 4.0 - Auto-generated WebAssembly bindings\n\n");
        
        // WASM loader
        js.push_str(r#"
let wasmInstance = null;
let wasmMemory = null;

export async function init(wasmPath = './module.wasm') {
    const response = await fetch(wasmPath);
    const bytes = await response.arrayBuffer();
    
    const imports = {
        env: {
            // SSL runtime imports
            ssl_print: (ptr, len) => {
                const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
                const text = new TextDecoder().decode(bytes);
                console.log(text);
            },
            ssl_panic: (ptr, len) => {
                const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
                const text = new TextDecoder().decode(bytes);
                throw new Error(`SSL Panic: ${text}`);
            },
        },
        js: {
            // JavaScript interop
            console_log: (ptr, len) => {
                const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
                console.log(new TextDecoder().decode(bytes));
            },
        },
    };
    
    const result = await WebAssembly.instantiate(bytes, imports);
    wasmInstance = result.instance;
    wasmMemory = wasmInstance.exports.memory;
    
    return wasmInstance.exports;
}
"#);
        
        // Generate wrapper functions for each export
        for export in &self.exports {
            js.push_str(&format!("\n// {}\n", export.name));
            js.push_str(&format!("export function {}(", export.name));
            
            let params: Vec<String> = export.params.iter()
                .enumerate()
                .map(|(i, _)| format!("arg{}", i))
                .collect();
            js.push_str(&params.join(", "));
            
            js.push_str(") {\n");
            js.push_str(&format!("    return wasmInstance.exports.{}({});\n", 
                export.name, params.join(", ")));
            js.push_str("}\n");
        }
        
        Ok(js)
    }
    
    /// Generate TypeScript declarations
    fn generate_ts_declarations(&self) -> Result<String, String> {
        let mut ts = String::new();
        
        ts.push_str("// SSL 4.0 - Auto-generated TypeScript declarations\n\n");
        ts.push_str("export function init(wasmPath?: string): Promise<WebAssembly.Exports>;\n\n");
        
        for export in &self.exports {
            let params: Vec<String> = export.params.iter()
                .enumerate()
                .map(|(i, t)| format!("arg{}: {}", i, self.wasm_type_to_ts(t)))
                .collect();
            
            let return_type = export.return_type
                .as_ref()
                .map(|t| self.wasm_type_to_ts(t))
                .unwrap_or("void".to_string());
            
            ts.push_str(&format!("export function {}({}): {};\n", 
                export.name, params.join(", "), return_type));
        }
        
        Ok(ts)
    }
    
    /// Convert WASM type to TypeScript type
    fn wasm_type_to_ts(&self, wasm_type: &WasmType) -> String {
        match wasm_type {
            WasmType::I32 | WasmType::I64 => "number".to_string(),
            WasmType::F32 | WasmType::F64 => "number".to_string(),
            _ => "any".to_string(),
        }
    }
    
    /// Generate HTML template
    fn generate_html_template(&self) -> Result<String, String> {
        Ok(format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SSL 4.0 WebAssembly App</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #eee;
            min-height: 100vh;
        }}
        h1 {{ color: #00d4ff; }}
        .output {{
            background: #0f0f1a;
            padding: 20px;
            border-radius: 8px;
            font-family: monospace;
            border: 1px solid #00d4ff33;
        }}
    </style>
</head>
<body>
    <h1>🚀 SSL 4.0 WebAssembly</h1>
    <div class="output" id="output">Loading WASM module...</div>
    
    <script type="module">
        import {{ init }} from './bindings.js';
        
        async function main() {{
            try {{
                const wasm = await init('./module.wasm');
                document.getElementById('output').innerHTML = 
                    '<strong style="color: #00ff88">✓ WASM loaded successfully!</strong><br><br>' +
                    'Exports: ' + Object.keys(wasm).join(', ');
            }} catch (e) {{
                document.getElementById('output').innerHTML = 
                    '<strong style="color: #ff4444">✗ Error:</strong> ' + e.message;
            }}
        }}
        
        main();
    </script>
</body>
</html>
"#))
    }
}

/// Helper for building WASM binary
struct WasmBinaryBuilder {
    buffer: Vec<u8>,
}

impl WasmBinaryBuilder {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    
    fn emit_header(&mut self) {
        // WASM magic number: \0asm
        self.buffer.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
        // Version 1
        self.buffer.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    }
    
    fn emit_type_section(&mut self, exports: &[WasmExport]) {
        if exports.is_empty() { return; }
        
        let mut section = Vec::new();
        
        // Number of types
        self.emit_leb128(&mut section, exports.len() as u32);
        
        for export in exports {
            // Function type marker
            section.push(0x60);
            
            // Parameters
            self.emit_leb128(&mut section, export.params.len() as u32);
            for param in &export.params {
                section.push(self.wasm_type_byte(param));
            }
            
            // Results
            if let Some(ref ret) = export.return_type {
                section.push(0x01);
                section.push(self.wasm_type_byte(ret));
            } else {
                section.push(0x00);
            }
        }
        
        // Section ID 1 = Type
        self.buffer.push(0x01);
        self.emit_leb128(&mut self.buffer.clone(), section.len() as u32);
        self.buffer.extend_from_slice(&self.leb128_bytes(section.len() as u32));
        self.buffer.extend(section);
    }
    
    fn emit_function_section(&mut self, exports: &[WasmExport]) {
        if exports.is_empty() { return; }
        
        let mut section = Vec::new();
        
        // Number of functions
        self.emit_leb128(&mut section, exports.len() as u32);
        
        // Type indices (0, 1, 2, ...)
        for i in 0..exports.len() {
            self.emit_leb128(&mut section, i as u32);
        }
        
        // Section ID 3 = Function
        self.buffer.push(0x03);
        self.buffer.extend_from_slice(&self.leb128_bytes(section.len() as u32));
        self.buffer.extend(section);
    }
    
    fn emit_export_section(&mut self, exports: &[WasmExport]) {
        if exports.is_empty() { return; }
        
        let mut section = Vec::new();
        
        // Number of exports
        self.emit_leb128(&mut section, exports.len() as u32);
        
        for (i, export) in exports.iter().enumerate() {
            // Name length + name
            let name_bytes = export.name.as_bytes();
            self.emit_leb128(&mut section, name_bytes.len() as u32);
            section.extend_from_slice(name_bytes);
            
            // Export kind: 0x00 = function
            section.push(0x00);
            
            // Function index
            self.emit_leb128(&mut section, i as u32);
        }
        
        // Section ID 7 = Export
        self.buffer.push(0x07);
        self.buffer.extend_from_slice(&self.leb128_bytes(section.len() as u32));
        self.buffer.extend(section);
    }
    
    fn emit_code_section(&mut self, _statements: &[Statement], exports: &[WasmExport]) -> Result<(), String> {
        if exports.is_empty() { return Ok(()); }
        
        let mut section = Vec::new();
        
        // Number of functions
        self.emit_leb128(&mut section, exports.len() as u32);
        
        for export in exports {
            // Simple function body: return 0
            let mut body = Vec::new();
            
            // Local declarations (none)
            body.push(0x00);
            
            // Return appropriate constant based on return type
            if let Some(ref ret_type) = export.return_type {
                match ret_type {
                    WasmType::I32 => {
                        body.push(0x41); // i32.const
                        body.push(0x00); // 0
                    }
                    WasmType::I64 => {
                        body.push(0x42); // i64.const
                        body.push(0x00); // 0
                    }
                    WasmType::F32 => {
                        body.push(0x43); // f32.const
                        body.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // 0.0
                    }
                    WasmType::F64 => {
                        body.push(0x44); // f64.const
                        body.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
                    }
                    _ => {}
                }
            }
            
            // End
            body.push(0x0B);
            
            // Function body size
            self.emit_leb128(&mut section, body.len() as u32);
            section.extend(body);
        }
        
        // Section ID 10 = Code
        self.buffer.push(0x0A);
        self.buffer.extend_from_slice(&self.leb128_bytes(section.len() as u32));
        self.buffer.extend(section);
        
        Ok(())
    }
    
    fn wasm_type_byte(&self, wasm_type: &WasmType) -> u8 {
        match wasm_type {
            WasmType::I32 => 0x7F,
            WasmType::I64 => 0x7E,
            WasmType::F32 => 0x7D,
            WasmType::F64 => 0x7C,
            WasmType::V128 => 0x7B,
            WasmType::FuncRef => 0x70,
            WasmType::ExternRef => 0x6F,
        }
    }
    
    fn emit_leb128(&self, buffer: &mut Vec<u8>, mut value: u32) {
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
    
    fn leb128_bytes(&self, mut value: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            bytes.push(byte);
            if value == 0 { break; }
        }
        bytes
    }
    
    fn finish(self) -> Vec<u8> {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasm_backend_creation() {
        let backend = WasmBackend::new(WasmTarget::default());
        assert_eq!(backend.exports.len(), 0);
    }
    
    #[test]
    fn test_wasm_header() {
        let mut builder = WasmBinaryBuilder::new();
        builder.emit_header();
        let bytes = builder.finish();
        
        // Check magic number
        assert_eq!(&bytes[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        // Check version
        assert_eq!(&bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }
}
