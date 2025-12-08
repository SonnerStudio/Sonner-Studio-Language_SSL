# SSL Plugin System - Design Document

**Version**: 1.0  
**Status**: Design Phase  
**Target Release**: Phase 9 (Q1 2026)

---

## 1. Overview

### 1.1 Vision

A **secure, extensible plugin system** allowing developers to extend SSL with:
- Custom syntax transformations
- New built-in functions
- Language server enhancements
- Code generation tools
- Custom linters & formatters

### 1.2 Design Principles

- **Security First**: Sandboxed execution, capability-based permissions
- **Easy to Write**: Simple Rust-based API
- **Fast Loading**: Lazy loading, caching
- **Version Compatible**: ABI stability guarantees

---

## 2. Architecture

### 2.1 Plugin Types

```rust
pub enum PluginType {
    /// Syntax transformation (macro-like)
    SyntaxExtension,
    
    /// Add stdlib functions
    StdlibExtension,
    
    /// LSP enhancements (autocomplete, etc.)
    LspExtension,
    
    /// Code generator (e.g., SQL → SSL)
    CodeGenerator,
    
    /// Linter/Formatter
    Analysis,
}
```

### 2.2 Plugin Lifecycle

```
1. Discovery:  ~/.sslpkg/plugins/*.so
2. Load:       dlopen() → Verify signature
3. Initialize: call plugin_init()
4. Register:   Register hooks with SSL runtime
5. Execute:    Call hooks during compilation/runtime
6. Unload:     Cleanup & dlclose()
```

### 2.3 Plugin Manifest

```toml
# plugin.toml
[plugin]
name = "sql-generator"
version = "1.0.0"
type = "CodeGenerator"
engine_version = "^2.0"  # Compatible SSL versions

[permissions]
filesystem = ["read"]    # Can read files
network = ["https"]      # Can make HTTPS requests
subprocess = false       # Cannot spawn processes

[hooks]
before_compile = true
after_parse = true
```

---

## 3. Plugin API

### 3.1 Core Trait

```rust
// src/plugin/api.rs

pub trait Plugin: Send + Sync {
    /// Plugin metadata
    fn metadata(&self) -> PluginMetadata;
    
    /// Initialize plugin
    fn init(&mut self, ctx: &PluginContext) -> Result<()>;
    
    /// Register hooks
    fn register_hooks(&self, registry: &mut HookRegistry);
    
    /// Cleanup
    fn shutdown(&mut self);
}

pub struct PluginMetadata {
    pub name: String,
    pub version: Version,
    pub plugin_type: PluginType,
    pub author: String,
    pub description: String,
}
```

### 3.2 Hook System

```rust
pub enum Hook {
    /// Before lexing
    BeforeLex(Box<dyn Fn(&str) -> Result<String>>),
    
    /// After parsing, before interpretation
    AfterParse(Box<dyn Fn(&mut Vec<Statement>) -> Result<()>>),
    
    /// Custom function call handler
    CustomFunction {
        name: String,
        handler: Box<dyn Fn(&[Value]) -> Result<Value>>,
    },
    
    /// LSP completion provider
    LspCompletion(Box<dyn Fn(&CompletionContext) -> Vec<CompletionItem>>),
}
```

### 3.3 Example Plugin

```rust
// example_plugin.rs

use ssl_plugin_api::*;

pub struct SqlGeneratorPlugin {
    config: Config,
}

impl Plugin for SqlGeneratorPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "sql-generator".into(),
            version: Version::parse("1.0.0").unwrap(),
            plugin_type: PluginType::CodeGenerator,
            author: "SSL Community".into(),
            description: "Generate SSL code from SQL schemas".into(),
        }
    }
    
    fn init(&mut self, ctx: &PluginContext) -> Result<()> {
        // Load config, connect to DB, etc.
        Ok(())
    }
    
    fn register_hooks(&self, registry: &mut HookRegistry) {
        // Register custom syntax: sql! { SELECT * FROM users }
        registry.register_macro("sql!", |tokens| {
            // Parse SQL, generate SSL code
            generate_ssl_from_sql(tokens)
        });
    }
    
    fn shutdown(&mut self) {
        // Cleanup
    }
}

// Export plugin entry point
#[no_mangle]
pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(SqlGeneratorPlugin::default()))
}
```

---

## 4. Security & Sandboxing

### 4.1 Capability System

```rust
pub struct Permissions {
    pub filesystem: FilePermissions,
    pub network: NetworkPermissions,
    pub subprocess: bool,
    pub env_vars: Vec<String>,
}

pub enum FilePermissions {
    None,
    Read(Vec<PathBuf>),       // Whitelist of read paths
    ReadWrite(Vec<PathBuf>),  // Whitelist of write paths
}

pub enum NetworkPermissions {
    None,
    Http(Vec<String>),        // Allowed domains
    Https(Vec<String>),
}
```

### 4.2 Sandboxing Strategy

```
1. Separate process per plugin (via fork/spawn)
2. IPC via Unix sockets / pipes
3. Resource limits (CPU, memory, time)
4. Capability-based permission checks
5. Code signing verification (GPG)
```

---

## 5. Plugin Discovery & Loading

### 5.1 Discovery Paths

```
1. ~/.sslpkg/plugins/     # User-installed
2. /usr/lib/ssl/plugins/  # System-wide
3. ./ssl_plugins/         # Project-local
```

### 5.2 Loading Process

```rust
pub struct PluginLoader {
    loaded: HashMap<String, Box<dyn Plugin>>,
}

impl PluginLoader {
    pub fn discover(&mut self) -> Vec<PathBuf> {
        // Scan discovery paths for *.so/*.dylib/*.dll
    }
    
    pub fn load(&mut self, path: &Path) -> Result<()> {
        // 1. Verify signature
        verify_plugin_signature(path)?;
        
        // 2. Load shared library
        let lib = unsafe { Library::new(path)? };
        
        // 3. Get entry point
        let create_fn: Symbol<extern "C" fn() -> *mut dyn Plugin> = 
            unsafe { lib.get(b"_plugin_create")? };
        
        // 4. Create plugin instance
        let plugin = unsafe { Box::from_raw(create_fn()) };
        
        // 5. Initialize
        plugin.init(&PluginContext::new())?;
        
        // 6. Store
        self.loaded.insert(plugin.metadata().name.clone(), plugin);
        
        Ok(())
    }
}
```

---

## 6. Use Cases & Examples

### 6.1 Syntax Extension: `sql!` Macro

```ssl
// User code
let users = sql! {
    SELECT id, name, email FROM users WHERE active = true
}

// Expanded by plugin to:
let users = db_query("SELECT id, name, email FROM users WHERE active = true")
    .map(|row| User { 
        id: row.get("id"), 
        name: row.get("name"), 
        email: row.get("email") 
    })
```

### 6.2 Custom Stdlib Function

```rust
// Plugin registers: http_post_json()
registry.register_function("http_post_json", |args| {
    let url = args[0].as_str()?;
    let body = args[1].as_map()?;
    let json = serde_json::to_string(&body)?;
    let response = reqwest::blocking::Client::new()
        .post(url)
        .json(&json)
        .send()?;
    Ok(Value::String(response.text()?))
});
```

```ssl
// User code
let response = http_post_json("https://api.example.com/data", {
    "name": "SSL",
    "version": "2.0.0"
})
```

### 6.3 LSP Completion Enhancement

```rust
// Plugin provides completions for SQL keywords
registry.register_lsp_completion(|ctx| {
    if ctx.in_macro("sql!") {
        vec![
            CompletionItem::new("SELECT"),
            CompletionItem::new("FROM"),
            CompletionItem::new("WHERE"),
            // ...
        ]
    } else {
        vec![]
    }
});
```

---

## 7. Implementation Roadmap

### Phase 9.1: Core Plugin System (Week 1-4)

- [ ] Plugin trait & API design
- [ ] Hook registry implementation
- [ ] Plugin loader (dlopen/dlsym)
- [ ] Basic sandboxing (separate process)

### Phase 9.2: Security (Week 5-6)

- [ ] Capability system
- [ ] Code signing verification
- [ ] Resource limits enforcement

### Phase 9.3: Plugin SDK (Week 7-8)

- [ ] `ssl-plugin-api` crate
- [ ] Example plugins
- [ ] Documentation & guides

### Phase 9.4: Integration (Week 9-10)

- [ ] CLI plugin management
- [ ] LSP integration
- [ ] Compiler hooks

---

## 8. Code Stubs

See `src/plugin/mod.rs` for implementation stubs.

---

## 9. References

- **Rust dylib**: https://doc.rust-lang.org/reference/linkage.html
- **Cargo plugins**: https://doc.rust-lang.org/cargo/reference/unstable.html#custom-plugins
- **VSCode extensions**: https://code.visualstudio.com/api
