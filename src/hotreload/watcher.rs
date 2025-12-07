use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event, EventKind};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

/// File watcher that detects source code changes
pub struct FileWatcher {
    watcher: Option<Box<dyn Watcher>>,
    receiver: Receiver<NotifyResult<Event>>,
    sender: Sender<NotifyResult<Event>>,
    debounce_ms: u64,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(debounce_ms: u64) -> Self {
        let (sender, receiver) = channel();
        
        FileWatcher {
            watcher: None,
            receiver,
            sender,
            debounce_ms,
        }
    }
    
    /// Start watching a directory
    pub fn watch(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let sender = self.sender.clone();
        
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = sender.send(res);
        }).map_err(|e| format!("Failed to create watcher: {}", e))?;
        
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch path: {}", e))?;
        
        self.watcher = Some(Box::new(watcher));
        Ok(())
    }
    
    /// Check for file changes (non-blocking)
    pub fn check_changes(&self) -> Option<Vec<String>> {
        let mut changed_files = Vec::new();
        
        // Collect all pending events
        while let Ok(event) = self.receiver.try_recv() {
            if let Ok(event) = event {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        for path in event.paths {
                            if let Some(ext) = path.extension() {
                                if ext == "ssl" {
                                    if let Some(path_str) = path.to_str() {
                                        changed_files.push(path_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        if changed_files.is_empty() {
            None
        } else {
            // Deduplicate
            changed_files.sort();
            changed_files.dedup();
            Some(changed_files)
        }
    }
    
    /// Wait for file changes (blocking with timeout)
    pub fn wait_for_changes(&self, timeout: Duration) -> Option<Vec<String>> {
        if let Ok(event) = self.receiver.recv_timeout(timeout) {
            if let Ok(event) = event {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        let mut files: Vec<String> = event.paths.iter()
                            .filter_map(|p| {
                                if p.extension()? == "ssl" {
                                    Some(p.to_str()?.to_string())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        
                        if !files.is_empty() {
                            files.sort();
                            files.dedup();
                            return Some(files);
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    
    #[test]
    fn test_file_watcher_creation() {
        let watcher = FileWatcher::new(500);
        assert_eq!(watcher.debounce_ms, 500);
    }
    
    // Note: Full integration tests require temp directories and are OS-specific
}
