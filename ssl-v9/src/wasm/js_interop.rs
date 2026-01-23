//! SSL 4.0 JavaScript Interop
//!
//! FFI bindings for JavaScript APIs in the browser.

use std::collections::HashMap;

/// JavaScript API bindings available in SSL
#[derive(Debug, Clone)]
pub struct JsInterop {
    /// Registered callbacks
    callbacks: HashMap<String, JsCallback>,
}

/// A JavaScript callback function
#[derive(Debug, Clone)]
pub struct JsCallback {
    /// Callback ID
    pub id: u32,
    /// Function name
    pub name: String,
    /// Parameter types
    pub params: Vec<JsType>,
}

/// JavaScript value types for interop
#[derive(Debug, Clone, PartialEq)]
pub enum JsType {
    Undefined,
    Null,
    Boolean,
    Number,
    BigInt,
    String,
    Object,
    Function,
    Array,
    Promise,
}

impl JsInterop {
    /// Create new JavaScript interop layer
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }
    
    /// Register a callback
    pub fn register_callback(&mut self, name: String, params: Vec<JsType>) -> u32 {
        let id = self.callbacks.len() as u32;
        self.callbacks.insert(name.clone(), JsCallback {
            id,
            name,
            params,
        });
        id
    }
    
    /// Get a registered callback
    pub fn get_callback(&self, name: &str) -> Option<&JsCallback> {
        self.callbacks.get(name)
    }
}

impl Default for JsInterop {
    fn default() -> Self {
        Self::new()
    }
}

/// Browser Console API
pub mod console {
    /// Generate JS import for console.log
    pub fn console_log_import() -> super::super::runtime::WasmImport {
        super::super::runtime::WasmImport {
            module: "js".to_string(),
            name: "console_log".to_string(),
            kind: super::super::runtime::WasmImportKind::Function {
                params: vec![super::super::WasmType::I32, super::super::WasmType::I32],
                results: vec![],
            },
        }
    }
    
    /// Generate JS import for console.error
    pub fn console_error_import() -> super::super::runtime::WasmImport {
        super::super::runtime::WasmImport {
            module: "js".to_string(),
            name: "console_error".to_string(),
            kind: super::super::runtime::WasmImportKind::Function {
                params: vec![super::super::WasmType::I32, super::super::WasmType::I32],
                results: vec![],
            },
        }
    }
    
    /// Generate JS import for console.warn
    pub fn console_warn_import() -> super::super::runtime::WasmImport {
        super::super::runtime::WasmImport {
            module: "js".to_string(),
            name: "console_warn".to_string(),
            kind: super::super::runtime::WasmImportKind::Function {
                params: vec![super::super::WasmType::I32, super::super::WasmType::I32],
                results: vec![],
            },
        }
    }
}

/// Browser DOM API
pub mod dom {
    use super::super::runtime::WasmImport;
    use super::super::WasmType;
    
    /// Generate DOM API imports
    pub fn dom_imports() -> Vec<WasmImport> {
        vec![
            // document.getElementById
            WasmImport {
                module: "dom".to_string(),
                name: "get_element_by_id".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32], // id_ptr, id_len
                    results: vec![WasmType::I32], // element handle
                },
            },
            // element.innerHTML setter
            WasmImport {
                module: "dom".to_string(),
                name: "set_inner_html".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32], // handle, html_ptr, html_len
                    results: vec![],
                },
            },
            // element.textContent setter
            WasmImport {
                module: "dom".to_string(),
                name: "set_text_content".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32], // handle, text_ptr, text_len
                    results: vec![],
                },
            },
            // element.addEventListener
            WasmImport {
                module: "dom".to_string(),
                name: "add_event_listener".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32], // handle, event_ptr, event_len, callback_id
                    results: vec![],
                },
            },
            // document.createElement
            WasmImport {
                module: "dom".to_string(),
                name: "create_element".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32], // tag_ptr, tag_len
                    results: vec![WasmType::I32], // element handle
                },
            },
            // element.appendChild
            WasmImport {
                module: "dom".to_string(),
                name: "append_child".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32], // parent, child
                    results: vec![],
                },
            },
            // element.setAttribute
            WasmImport {
                module: "dom".to_string(),
                name: "set_attribute".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32, WasmType::I32], // handle, name_ptr, name_len, val_ptr, val_len
                    results: vec![],
                },
            },
        ]
    }
}

/// Browser Fetch API
pub mod fetch {
    use super::super::runtime::WasmImport;
    use super::super::WasmType;
    
    /// Generate Fetch API imports
    pub fn fetch_imports() -> Vec<WasmImport> {
        vec![
            // fetch(url)
            WasmImport {
                module: "http".to_string(),
                name: "fetch".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32, WasmType::I32], // url_ptr, url_len
                    results: vec![WasmType::I32], // request_id
                },
            },
            // fetch_with_options(url, method, body, headers)
            WasmImport {
                module: "http".to_string(),
                name: "fetch_with_options".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![
                        WasmType::I32, WasmType::I32, // url
                        WasmType::I32, WasmType::I32, // method
                        WasmType::I32, WasmType::I32, // body
                        WasmType::I32, WasmType::I32, // headers
                    ],
                    results: vec![WasmType::I32], // request_id
                },
            },
            // get_response_status(request_id)
            WasmImport {
                module: "http".to_string(),
                name: "get_response_status".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32], // request_id
                    results: vec![WasmType::I32], // status code
                },
            },
            // get_response_body(request_id) -> ptr, len
            WasmImport {
                module: "http".to_string(),
                name: "get_response_body".to_string(),
                kind: super::super::runtime::WasmImportKind::Function {
                    params: vec![WasmType::I32], // request_id
                    results: vec![WasmType::I32, WasmType::I32], // body_ptr, body_len
                },
            },
        ]
    }
}

/// Generate complete JavaScript glue code for all apis
pub fn generate_js_glue() -> String {
    r#"
// SSL 4.0 WebAssembly JavaScript Glue Code
// Auto-generated - Do not edit

const SSL = {
    memory: null,
    instance: null,
    elements: new Map(),
    nextElementId: 1,
    callbacks: new Map(),
    nextCallbackId: 1,
    pendingFetches: new Map(),
    nextFetchId: 1,

    // String helpers
    getString(ptr, len) {
        const bytes = new Uint8Array(this.memory.buffer, ptr, len);
        return new TextDecoder().decode(bytes);
    },
    
    setString(str) {
        const bytes = new TextEncoder().encode(str);
        const ptr = this.instance.exports.ssl_alloc(bytes.length);
        new Uint8Array(this.memory.buffer, ptr, bytes.length).set(bytes);
        return [ptr, bytes.length];
    },

    // Environment imports
    env: {
        ssl_print(ptr, len) {
            console.log(SSL.getString(ptr, len));
        },
        ssl_panic(ptr, len) {
            throw new Error(`SSL Panic: ${SSL.getString(ptr, len)}`);
        },
        ssl_alloc(size) {
            // Simple bump allocator - in production use a proper allocator
            return SSL.instance.exports.__heap_base || 1024;
        },
        ssl_free(ptr) {
            // No-op for bump allocator
        },
        ssl_time_ms() {
            return BigInt(Date.now());
        },
        ssl_random() {
            return Math.random();
        },
    },

    // JavaScript console imports
    js: {
        console_log(ptr, len) {
            console.log(SSL.getString(ptr, len));
        },
        console_error(ptr, len) {
            console.error(SSL.getString(ptr, len));
        },
        console_warn(ptr, len) {
            console.warn(SSL.getString(ptr, len));
        },
    },

    // DOM imports
    dom: {
        get_element_by_id(ptr, len) {
            const id = SSL.getString(ptr, len);
            const el = document.getElementById(id);
            if (!el) return 0;
            
            const handle = SSL.nextElementId++;
            SSL.elements.set(handle, el);
            return handle;
        },
        set_inner_html(handle, ptr, len) {
            const el = SSL.elements.get(handle);
            if (el) el.innerHTML = SSL.getString(ptr, len);
        },
        set_text_content(handle, ptr, len) {
            const el = SSL.elements.get(handle);
            if (el) el.textContent = SSL.getString(ptr, len);
        },
        add_event_listener(handle, eventPtr, eventLen, callbackId) {
            const el = SSL.elements.get(handle);
            const event = SSL.getString(eventPtr, eventLen);
            if (el) {
                el.addEventListener(event, () => {
                    SSL.instance.exports.__ssl_callback(callbackId);
                });
            }
        },
        create_element(ptr, len) {
            const tag = SSL.getString(ptr, len);
            const el = document.createElement(tag);
            const handle = SSL.nextElementId++;
            SSL.elements.set(handle, el);
            return handle;
        },
        append_child(parentHandle, childHandle) {
            const parent = SSL.elements.get(parentHandle);
            const child = SSL.elements.get(childHandle);
            if (parent && child) parent.appendChild(child);
        },
        set_attribute(handle, namePtr, nameLen, valPtr, valLen) {
            const el = SSL.elements.get(handle);
            if (el) {
                el.setAttribute(SSL.getString(namePtr, nameLen), SSL.getString(valPtr, valLen));
            }
        },
    },

    // HTTP Fetch imports
    http: {
        async fetch(urlPtr, urlLen) {
            const url = SSL.getString(urlPtr, urlLen);
            const id = SSL.nextFetchId++;
            SSL.pendingFetches.set(id, fetch(url).then(r => r.text()));
            return id;
        },
        get_response_status(id) {
            // Simplified - in production handle async properly
            return 200;
        },
        get_response_body(id) {
            // Simplified - returns placeholder
            return [0, 0];
        },
    },

    // Initialize WASM module
    async init(wasmPath) {
        const response = await fetch(wasmPath);
        const bytes = await response.arrayBuffer();
        
        const result = await WebAssembly.instantiate(bytes, {
            env: this.env,
            js: this.js,
            dom: this.dom,
            http: this.http,
        });
        
        this.instance = result.instance;
        this.memory = this.instance.exports.memory;
        
        return this.instance.exports;
    }
};

// Export for ES modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = SSL;
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_js_interop_creation() {
        let interop = JsInterop::new();
        assert!(interop.callbacks.is_empty());
    }
    
    #[test]
    fn test_callback_registration() {
        let mut interop = JsInterop::new();
        let id = interop.register_callback("onClick".to_string(), vec![JsType::Object]);
        
        assert_eq!(id, 0);
        assert!(interop.get_callback("onClick").is_some());
    }
    
    #[test]
    fn test_dom_imports() {
        let imports = dom::dom_imports();
        assert!(!imports.is_empty());
        
        let get_element = imports.iter().find(|i| i.name == "get_element_by_id");
        assert!(get_element.is_some());
    }
}
