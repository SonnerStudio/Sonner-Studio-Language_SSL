use crate::interpreter::{Environment, Value};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents a snapshot of the interpreter state at a specific point in time
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// Unique snapshot ID
    pub id: usize,
    
    /// Line number in source code
    pub line: usize,
    
    /// Current environment state (variables)
    pub environment: HashMap<String, Value>,
    
    /// Call stack depth
    pub stack_depth: usize,
    
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    
    /// Memory usage estimate (bytes)
    pub memory_usage: usize,
}

impl StateSnapshot {
    /// Create a new snapshot from interpreter environment
    pub fn capture(id: usize, line: usize, env: &Environment) -> Self {
        // Capture current scope variables
        let environment = env.scopes.last()
            .map(|scope| {
                scope.iter()
                    .map(|(k, (v, _))| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();
        
        let stack_depth = env.scopes.len();
        
        // Estimate memory usage
        let memory_usage = std::mem::size_of_val(&environment);
        
        StateSnapshot {
            id,
            line,
            environment,
            stack_depth,
            timestamp: std::time::SystemTime::now(),
            memory_usage,
        }
    }
    
    /// Calculate diff from another snapshot (only changed variables)
    pub fn diff(&self, other: &StateSnapshot) -> HashMap<String, VariableChange> {
        let mut changes = HashMap::new();
        
        // Find added/modified variables
        for (key, value) in &self.environment {
            match other.environment.get(key) {
                Some(other_value) if other_value != value => {
                    changes.insert(key.clone(), VariableChange::Modified {
                        old: other_value.clone(),
                        new: value.clone(),
                    });
                }
                None => {
                    changes.insert(key.clone(), VariableChange::Added(value.clone()));
                }
                _ => {} // No change
            }
        }
        
        // Find removed variables
        for key in other.environment.keys() {
            if !self.environment.contains_key(key) {
                changes.insert(key.clone(), VariableChange::Removed);
            }
        }
        
        changes
    }
}

/// Represents a change in a variable between snapshots
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VariableChange {
    Added(Value),
    Modified { old: Value, new: Value },
    Removed,
}

/// Manages state snapshots with compression and efficient storage
pub struct SnapshotManager {
    /// All snapshots (full + diffs)
    snapshots: Vec<StateSnapshot>,
    
    /// Interval for full snapshots (every N steps)
    full_snapshot_interval: usize,
    
    /// Maximum snapshots to keep
    max_snapshots: usize,
    
    /// Current snapshot index
    pub current_index: usize,
}

impl SnapshotManager {
    /// Create a new snapshot manager
    pub fn new() -> Self {
        SnapshotManager {
            snapshots: Vec::new(),
            full_snapshot_interval: 100, // Full snapshot every 100 steps
            max_snapshots: 10000, // Keep last 10k snapshots
            current_index: 0,
        }
    }
    
    /// Record a new snapshot
    pub fn record(&mut self, line: usize, env: &Environment) {
        let id = self.snapshots.len();
        let snapshot = StateSnapshot::capture(id, line, env);
        
        // Add to history
        self.snapshots.push(snapshot);
        self.current_index = id;
        
        // Limit history size
        if self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
            self.current_index = self.current_index.saturating_sub(1);
        }
    }
    
    /// Get current snapshot
    pub fn current(&self) -> Option<&StateSnapshot> {
        self.snapshots.get(self.current_index)
    }
    
    /// Step back in time
    pub fn step_back(&mut self) -> Option<&StateSnapshot> {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
        self.current()
    }
    
    /// Step forward in time
    pub fn step_forward(&mut self) -> Option<&StateSnapshot> {
        if self.current_index < self.snapshots.len() - 1 {
            self.current_index += 1;
        }
        self.current()
    }
    
    /// Jump to a specific snapshot by ID
    pub fn goto(&mut self, id: usize) -> Option<&StateSnapshot> {
        if id < self.snapshots.len() {
            self.current_index = id;
            self.current()
        } else {
            None
        }
    }
    
    /// Jump to a specific line number (finds closest snapshot)
    pub fn goto_line(&mut self, line: usize) -> Option<&StateSnapshot> {
        // Find snapshot closest to target line
        let idx = self.snapshots.iter()
            .enumerate()
            .min_by_key(|(_, snap)| (snap.line as i64 - line as i64).abs())?
            .0;
        
        self.current_index = idx;
        self.current()
    }
    
    /// Get total number of snapshots
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    
    /// Get memory usage estimate
    pub fn memory_usage(&self) -> usize {
        self.snapshots.iter().map(|s| s.memory_usage).sum()
    }
    
    /// Clear all snapshots
    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.current_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_snapshot_capture() {
        let env = Environment::new();
        let snapshot = StateSnapshot::capture(0, 1, &env);
        
        assert_eq!(snapshot.id, 0);
        assert_eq!(snapshot.line, 1);
        assert_eq!(snapshot.stack_depth, 1);
    }
    
    #[test]
    fn test_snapshot_manager() {
        let mut manager = SnapshotManager::new();
        let env = Environment::new();
        
        // Record snapshots
        manager.record(1, &env);
        manager.record(2, &env);
        manager.record(3, &env);
        
        assert_eq!(manager.len(), 3);
        assert_eq!(manager.current().unwrap().line, 3);
        
        // Step back
        manager.step_back();
        assert_eq!(manager.current().unwrap().line, 2);
        
        // Step forward
        manager.step_forward();
        assert_eq!(manager.current().unwrap().line, 3);
    }
    
    #[test]
    fn test_goto_line() {
        let mut manager = SnapshotManager::new();
        let env = Environment::new();
        
        manager.record(10, &env);
        manager.record(20, &env);
        manager.record(30, &env);
        
        manager.goto_line(25);
        assert_eq!(manager.current().unwrap().line, 20); // Closest to 25
    }
}
