//! SSL 4.0 WebAssembly Bindings
//!
//! High-level bindings for browser APIs in SSL code.

use super::WasmType;

/// SSL attribute for WASM-exported functions
#[derive(Debug, Clone, PartialEq)]
pub enum WasmAttribute {
    /// Export function to JavaScript
    Export,
    /// Import function from JavaScript
    Import { module: String, name: String },
    /// Mark as async (returns Promise)
    Async,
    /// No side effects (pure function)
    Pure,
}

/// Parse @wasm attributes from SSL source
pub fn parse_wasm_attribute(attr: &str) -> Option<WasmAttribute> {
    let attr = attr.trim();
    
    if attr == "wasm" || attr == "export" {
        return Some(WasmAttribute::Export);
    }
    
    if attr == "async" {
        return Some(WasmAttribute::Async);
    }
    
    if attr == "pure" {
        return Some(WasmAttribute::Pure);
    }
    
    // Parse @import(module, name)
    if attr.starts_with("import(") && attr.ends_with(')') {
        let inner = &attr[7..attr.len()-1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim().trim_matches('"')).collect();
        if parts.len() == 2 {
            return Some(WasmAttribute::Import {
                module: parts[0].to_string(),
                name: parts[1].to_string(),
            });
        }
    }
    
    None
}

/// SSL Standard Library bindings for WASM
pub mod stdlib {
    /// Print function binding
    pub fn print_binding() -> String {
        r#"
// SSL print() -> console.log
function ssl_print_impl(wasmMemory, ptr, len) {
    const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
    console.log(new TextDecoder().decode(bytes));
}
"#.to_string()
    }
    
    /// HTTP GET binding
    pub fn http_get_binding() -> String {
        r#"
// SSL http_get(url) -> Promise<String>
async function ssl_http_get_impl(wasmInstance, urlPtr, urlLen) {
    const memory = wasmInstance.exports.memory;
    const bytes = new Uint8Array(memory.buffer, urlPtr, urlLen);
    const url = new TextDecoder().decode(bytes);
    
    const response = await fetch(url);
    return await response.text();
}
"#.to_string()
    }
    
    /// JSON parse binding
    pub fn json_parse_binding() -> String {
        r#"
// SSL json_parse(str) -> Object
function ssl_json_parse_impl(wasmMemory, ptr, len) {
    const bytes = new Uint8Array(wasmMemory.buffer, ptr, len);
    const text = new TextDecoder().decode(bytes);
    return JSON.parse(text);
}
"#.to_string()
    }
    
    /// Local storage binding
    pub fn storage_binding() -> String {
        r#"
// SSL storage_get(key) -> String | null
function ssl_storage_get_impl(wasmMemory, keyPtr, keyLen) {
    const bytes = new Uint8Array(wasmMemory.buffer, keyPtr, keyLen);
    const key = new TextDecoder().decode(bytes);
    return localStorage.getItem(key);
}

// SSL storage_set(key, value)
function ssl_storage_set_impl(wasmMemory, keyPtr, keyLen, valPtr, valLen) {
    const keyBytes = new Uint8Array(wasmMemory.buffer, keyPtr, keyLen);
    const valBytes = new Uint8Array(wasmMemory.buffer, valPtr, valLen);
    const key = new TextDecoder().decode(keyBytes);
    const value = new TextDecoder().decode(valBytes);
    localStorage.setItem(key, value);
}
"#.to_string()
    }
}

/// Generate TypeScript type for SSL type
pub fn ssl_type_to_typescript(ssl_type: &str) -> &'static str {
    match ssl_type {
        "Int" | "Int32" | "Int64" => "number",
        "Float" | "Float32" | "Float64" => "number",
        "Bool" => "boolean",
        "String" => "string",
        "List" => "Array<any>",
        "Map" => "Record<string, any>",
        _ => "any",
    }
}

/// Generate JavaScript type conversion
pub fn js_type_conversion(from: &WasmType, var_name: &str) -> String {
    match from {
        WasmType::I32 | WasmType::I64 => format!("Number({})", var_name),
        WasmType::F32 | WasmType::F64 => format!("Number({})", var_name),
        _ => var_name.to_string(),
    }
}

/// WebAssembly instantiation helper code
pub fn wasm_instantiation_code() -> String {
    r#"
/**
 * SSL 4.0 WebAssembly Loader
 * 
 * Usage:
 *   import { loadSSL } from './ssl-loader.js';
 *   const ssl = await loadSSL('./my-app.wasm');
 *   ssl.myFunction();
 */

export async function loadSSL(wasmPath, options = {}) {
    const {
        onReady = () => {},
        onError = (e) => console.error('SSL Error:', e),
        debug = false,
    } = options;

    try {
        // Fetch WASM binary
        const response = await fetch(wasmPath);
        if (!response.ok) {
            throw new Error(`Failed to load WASM: ${response.status}`);
        }
        
        const bytes = await response.arrayBuffer();
        
        // Create import object with SSL runtime
        const memory = new WebAssembly.Memory({ initial: 256, maximum: 1024 });
        
        const imports = {
            env: {
                memory,
                ssl_print: (ptr, len) => {
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    console.log(new TextDecoder().decode(bytes));
                },
                ssl_panic: (ptr, len) => {
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    throw new Error(`SSL Panic: ${new TextDecoder().decode(bytes)}`);
                },
                ssl_time_ms: () => BigInt(Date.now()),
                ssl_random: () => Math.random(),
            },
        };

        // Instantiate WASM
        const result = await WebAssembly.instantiate(bytes, imports);
        const instance = result.instance;
        
        if (debug) {
            console.log('SSL WASM loaded:', Object.keys(instance.exports));
        }
        
        onReady(instance.exports);
        return instance.exports;
        
    } catch (error) {
        onError(error);
        throw error;
    }
}

/**
 * Helper to create a typed wrapper around WASM exports
 */
export function createSSLModule(exports) {
    return new Proxy(exports, {
        get(target, prop) {
            if (typeof target[prop] === 'function') {
                return (...args) => {
                    try {
                        return target[prop](...args);
                    } catch (e) {
                        console.error(`SSL call failed: ${String(prop)}`, e);
                        throw e;
                    }
                };
            }
            return target[prop];
        }
    });
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_wasm_export() {
        assert_eq!(parse_wasm_attribute("wasm"), Some(WasmAttribute::Export));
        assert_eq!(parse_wasm_attribute("export"), Some(WasmAttribute::Export));
    }
    
    #[test]
    fn test_parse_wasm_import() {
        let attr = parse_wasm_attribute(r#"import("env", "log")"#);
        assert!(matches!(attr, Some(WasmAttribute::Import { .. })));
    }
    
    #[test]
    fn test_typescript_types() {
        assert_eq!(ssl_type_to_typescript("Int"), "number");
        assert_eq!(ssl_type_to_typescript("String"), "string");
        assert_eq!(ssl_type_to_typescript("Bool"), "boolean");
    }
}
