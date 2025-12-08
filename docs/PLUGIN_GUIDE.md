# SSL Plugin System - Usage Guide

## Creating a Plugin

### 1. Create Plugin Library

```rust
// my_plugin/src/lib.rs

use std::os::raw::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct PluginMetadata {
    pub name: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char,
    pub description: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn plugin_metadata() -> *mut PluginMetadata {
    let name = CString::new("my-plugin").unwrap().into_raw();
    let version = CString::new("1.0.0").unwrap().into_raw();
    let author = CString::new("Your Name").unwrap().into_raw();
    let description = CString::new("My awesome plugin").unwrap().into_raw();
    
    Box::into_raw(Box::new(PluginMetadata {
        name, version, author, description,
    }))
}

#[no_mangle]
pub extern "C" fn plugin_init() -> i32 {
    println!("Plugin initialized!");
    0  // Success
}
```

### 2. Build Plugin

```bash
# In plugin directory
cargo build --lib --release

# Output: target/release/libmy_plugin.so (Linux)
#         target/release/my_plugin.dll (Windows)
#         target/release/libmy_plugin.dylib (macOS)
```

### 3. Install Plugin

```bash
# Copy to plugin directory
cp target/release/libmy_plugin.so ~/.sslpkg/plugins/
```

## Using Plugins in SSL

### Automatic Loading

Plugins in standard directories are loaded automatically:
- `~/.sslpkg/plugins/` (user plugins)
- `/usr/lib/ssl/plugins/` (system plugins)  
- `./ssl_plugins/` (project plugins)

### Manual Loading

```rust
use ssl::plugin::PluginLoader;

let mut loader = PluginLoader::new();

// Discover all plugins
let plugins = loader.discover_plugins();
println!("Found {} plugins", plugins.len());

// Load specific plugin
loader.load_plugin("path/to/plugin.so")?;

// List loaded plugins
for metadata in loader.list_plugins() {
    println!("- {} v{}", metadata.name, metadata.version);
}
```

## Example Plugins

See `examples/plugins/` for:
- `example_plugin.rs` - Basic plugin template
- `sql_generator.rs` - SQL to SSL code generator (coming soon)
- `http_macro.rs` - HTTP request macro (coming soon)

## Plugin API

### Required Exports

Every plugin must export:

```rust
#[no_mangle]
pub unsafe extern "C" fn plugin_metadata() -> *mut PluginMetadata;

#[no_mangle]
pub extern "C" fn plugin_init() -> i32;
```

### Optional Exports

```rust
#[no_mangle]
pub extern "C" fn plugin_shutdown();

#[no_mangle]
pub extern "C" fn register_hooks(registry: *mut HookRegistry);
```

## Security

Plugins run in the same process as SSL. Only load plugins from trusted sources!

Future versions will include:
- Code signing verification
- Sandboxed execution
- Permission system
