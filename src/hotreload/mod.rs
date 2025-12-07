// Live Programming / Hot Reload Module
// Enables editing code while it's running

pub mod watcher;
pub mod replacer;
pub mod state;

pub use watcher::FileWatcher;
pub use replacer::FunctionReplacer;
pub use state::StateMigrator;

/// Hot reload configuration
#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    /// Watch these file patterns
    pub watch_patterns: Vec<String>,
    
    /// Debounce delay (milliseconds)
    pub debounce_ms: u64,
    
    /// Enable automatic reload
    pub auto_reload: bool,
    
    /// Preserve state on reload
    pub preserve_state: bool,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        HotReloadConfig {
            watch_patterns: vec!["*.ssl".to_string()],
            debounce_ms: 500,
            auto_reload: true,
            preserve_state: true,
        }
    }
}
