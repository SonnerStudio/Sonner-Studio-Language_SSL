// SSL Plugin System - Foundation Module
// Target: Phase 9 (Q1 2026)
// See docs/design/PLUGIN_SYSTEM_DESIGN.md for full specification

pub mod api;
pub mod loader;
pub mod sandbox;
pub mod hooks;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy)]
pub enum PluginType {
    SyntaxExtension,
    StdlibExtension,
    LspExtension,
    CodeGenerator,
    Analysis,
}

/// Core plugin trait
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;
    
    /// Initialize plugin
    fn init(&mut self, ctx: &PluginContext) -> Result<(), String>;
    
    /// Register hooks with SSL runtime
    fn register_hooks(&self, registry: &mut HookRegistry);
    
    /// Cleanup on shutdown
    fn shutdown(&mut self);
}

/// Context passed to plugins during initialization
pub struct PluginContext {
    pub config_dir: PathBuf,
    pub permissions: Permissions,
}

/// Permission system for plugin sandboxing
#[derive(Debug, Clone)]
pub struct Permissions {
    pub filesystem: FilePermissions,
    pub network: NetworkPermissions,
    pub subprocess: bool,
    pub env_vars: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum FilePermissions {
    None,
    Read(Vec<PathBuf>),
    ReadWrite(Vec<PathBuf>),
}

#[derive(Debug, Clone)]
pub enum NetworkPermissions {
    None,
    Http(Vec<String>),
    Https(Vec<String>),
}

/// Hook registry for plugin capabilities
pub struct HookRegistry {
    hooks: Vec<Hook>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }
    
    pub fn register_hook(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }
    
    pub fn get_hooks(&self, hook_type: HookType) -> Vec<&Hook> {
        // TODO: Filter hooks by type
        todo!("Phase 9.1: Implement hook filtering")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    BeforeLex,
    AfterParse,
    CustomFunction,
    LspCompletion,
}

pub enum Hook {
    BeforeLex(Box<dyn Fn(&str) -> Result<String, String> + Send + Sync>),
    AfterParse(Box<dyn Fn(&mut Vec<crate::ast::Statement>) -> Result<(), String> + Send + Sync>),
    CustomFunction {
        name: String,
        handler: Box<dyn Fn(&[crate::interpreter::Value]) -> Result<crate::interpreter::Value, String> + Send + Sync>,
    },
    LspCompletion(Box<dyn Fn(&CompletionContext) -> Vec<CompletionItem> + Send + Sync>),
}

#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum CompletionKind {
    Function,
    Variable,
    Keyword,
    Type,
}

/// Plugin loader - handles discovery and loading
pub struct PluginLoader {
    loaded_plugins: HashMap<String, LoadedPlugin>,
    discovery_paths: Vec<PathBuf>,
}

struct LoadedPlugin {
    name: String,
    _library: libloading::Library,  // Keep library alive
    metadata: PluginMetadata,
}

impl PluginLoader {
    pub fn new() -> Self {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_default();
        
        Self {
            loaded_plugins: HashMap::new(),
            discovery_paths: vec![
                PathBuf::from(format!("{}/.sslpkg/plugins", home)),
                PathBuf::from("/usr/lib/ssl/plugins"),
                PathBuf::from("./ssl_plugins"),
            ],
        }
    }
    
    /// Discover plugins in standard directories
    pub fn discover_plugins(&self) -> Vec<PathBuf> {
        let mut plugins = Vec::new();
        
        #[cfg(target_os = "windows")]
        let extension = "dll";
        #[cfg(target_os = "macos")]
        let extension = "dylib";
        #[cfg(target_os = "linux")]
        let extension = "so";
        
        for dir in &self.discovery_paths {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                        plugins.push(path);
                    }
                }
            }
        }
        
        plugins
    }
    
    /// Load plugin from shared library
    pub fn load_plugin(&mut self, path: &Path) -> Result<(), String> {
        // Safety: Loading external code is inherently unsafe
        unsafe {
            // Load the library
            let library = libloading::Library::new(path)
                .map_err(|e| format!("Failed to load plugin: {}", e))?;
            
            // Get the plugin_create function
            let create_fn: libloading::Symbol<unsafe extern "C" fn() -> *mut PluginMetadata> = 
                library.get(b"plugin_metadata\0")
                .map_err(|e| format!("Plugin missing 'plugin_metadata' symbol: {}", e))?;
            
            // Call the function to get metadata
            let metadata_ptr = create_fn();
            let metadata = (*metadata_ptr).clone();
            
            // Store the plugin
            let loaded = LoadedPlugin {
                name: metadata.name.clone(),
                _library: library,
                metadata,
            };
            
            self.loaded_plugins.insert(loaded.name.clone(), loaded);
            
            Ok(())
        }
    }
    
    /// Load all plugins from discovery paths
    pub fn load_all_plugins(&mut self) -> Vec<String> {
        let mut errors = Vec::new();
        
        for path in self.discover_plugins() {
            if let Err(e) = self.load_plugin(&path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
        
        errors
    }
    
    /// Get plugin metadata by name
    pub fn get_plugin(&self, name: &str) -> Option<&PluginMetadata> {
        self.loaded_plugins.get(name).map(|p| &p.metadata)
    }
    
    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.loaded_plugins.values().map(|p| &p.metadata).collect()
    }
}

/// Sandbox executor for plugin isolation
pub struct PluginSandbox {
    // TODO: Process isolation, IPC
}

impl PluginSandbox {
    pub fn new(permissions: Permissions) -> Self {
        Self {}
    }
    
    pub fn execute<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce() -> Result<R, String>,
    {
        // TODO: Execute in isolated process with permission checks
        todo!("Phase 9.2: Implement sandboxing")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_loading() {
        // TODO: Test plugin discovery and loading
    }
    
    #[test]
    fn test_hook_registration() {
        // TODO: Test hook registry
    }
}
