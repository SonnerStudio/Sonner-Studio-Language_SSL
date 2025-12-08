// Example SSL Plugin - demonstrates dynamic loading
// Build with: cargo build --lib --release
// Output: target/release/libexample_plugin.so (or .dll/.dylib)

use std::os::raw::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct PluginMetadata {
    pub name: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char,
    pub description: *const c_char,
}

impl Clone for PluginMetadata {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            version: self.version,
            author: self.author,
            description: self.description,
        }
    }
}

/// Entry point for plugin - returns metadata
#[no_mangle]
pub unsafe extern "C" fn plugin_metadata() -> *mut PluginMetadata {
    let name = CString::new("example-plugin").unwrap().into_raw();
    let version = CString::new("1.0.0").unwrap().into_raw();
    let author = CString::new("SSL Community").unwrap().into_raw();
    let description = CString::new("Example plugin demonstrating SSL plugin API").unwrap().into_raw();
    
    Box::into_raw(Box::new(PluginMetadata {
        name,
        version,
        author,
        description,
    }))
}

/// Custom function registered by this plugin
#[no_mangle]
pub extern "C" fn custom_function(x: i64) -> i64 {
    // Example: double the input
    x * 2
}

/// Plugin initialization
#[no_mangle]
pub extern "C" fn plugin_init() -> i32 {
    // Return 0 for success, non-zero for error
    println!("Example plugin initialized!");
    0
}

/// Plugin cleanup
#[no_mangle]
pub extern "C" fn plugin_shutdown() {
    println!("Example plugin shutting down");
}
