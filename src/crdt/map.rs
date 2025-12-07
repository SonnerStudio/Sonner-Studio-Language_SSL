//! CRDT Map implementations
//!
//! LWW-Map (Last-Write-Wins) and MV-Map (Multi-Value)

use super::{NodeId, Timestamp, StateCrdt};
use std::collections::HashMap;

/// LWW-Map Entry
#[derive(Debug, Clone)]
pub struct LWWEntry<V> {
    /// The value
    pub value: Option<V>,
    /// Timestamp of last write
    pub timestamp: Timestamp,
    /// Node that made the write
    pub node: NodeId,
}

/// LWW-Map - Last-Write-Wins Map
#[derive(Debug, Clone)]
pub struct LWWMap<K: Clone + Eq + std::hash::Hash, V: Clone> {
    entries: HashMap<K, LWWEntry<V>>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> Default for LWWMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> LWWMap<K, V> {
    /// Create a new LWW-Map
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    /// Set a value
    pub fn set(&mut self, key: K, value: V, node_id: &str, timestamp: Timestamp) {
        let should_update = self.entries.get(&key)
            .map(|e| timestamp > e.timestamp || 
                (timestamp == e.timestamp && node_id > e.node.as_str()))
            .unwrap_or(true);
        
        if should_update {
            self.entries.insert(key, LWWEntry {
                value: Some(value),
                timestamp,
                node: node_id.to_string(),
            });
        }
    }
    
    /// Remove a key (tombstone)
    pub fn remove(&mut self, key: &K, node_id: &str, timestamp: Timestamp) {
        let should_update = self.entries.get(key)
            .map(|e| timestamp > e.timestamp ||
                (timestamp == e.timestamp && node_id > e.node.as_str()))
            .unwrap_or(true);
        
        if should_update {
            self.entries.insert(key.clone(), LWWEntry {
                value: None,
                timestamp,
                node: node_id.to_string(),
            });
        }
    }
    
    /// Get a value
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key).and_then(|e| e.value.as_ref())
    }
    
    /// Check if key exists
    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.get(key)
            .map(|e| e.value.is_some())
            .unwrap_or(false)
    }
    
    /// Get all keys
    pub fn keys(&self) -> Vec<&K> {
        self.entries
            .iter()
            .filter(|(_, v)| v.value.is_some())
            .map(|(k, _)| k)
            .collect()
    }
    
    /// Get all entries
    pub fn entries(&self) -> Vec<(&K, &V)> {
        self.entries
            .iter()
            .filter_map(|(k, v)| v.value.as_ref().map(|val| (k, val)))
            .collect()
    }
    
    /// Get length
    pub fn len(&self) -> usize {
        self.entries
            .values()
            .filter(|v| v.value.is_some())
            .count()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> StateCrdt for LWWMap<K, V> {
    fn merge(&mut self, other: &Self) {
        for (key, entry) in &other.entries {
            let should_update = self.entries.get(key)
                .map(|e| entry.timestamp > e.timestamp ||
                    (entry.timestamp == e.timestamp && entry.node > e.node))
                .unwrap_or(true);
            
            if should_update {
                self.entries.insert(key.clone(), entry.clone());
            }
        }
    }
    
    fn value(&self) -> String {
        format!("LWWMap({} entries)", self.len())
    }
}

/// Multi-Value Map Entry
#[derive(Debug, Clone)]
pub struct MVEntry<V> {
    /// All concurrent values with their version vectors
    values: Vec<(V, HashMap<NodeId, Timestamp>)>,
}

/// MV-Map - Multi-Value Map (keeps all concurrent values)
#[derive(Debug, Clone)]
pub struct MVMap<K: Clone + Eq + std::hash::Hash, V: Clone + PartialEq> {
    entries: HashMap<K, MVEntry<V>>,
    clocks: HashMap<NodeId, Timestamp>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + PartialEq> Default for MVMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + PartialEq> MVMap<K, V> {
    /// Create a new MV-Map
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            clocks: HashMap::new(),
        }
    }
    
    /// Set a value (replaces all values this node has observed)
    pub fn set(&mut self, key: K, value: V, node_id: &str) {
        // Increment local clock
        let ts = self.clocks.entry(node_id.to_string()).or_insert(0);
        *ts += 1;
        
        // Create new version vector
        let mut version = self.clocks.clone();
        
        self.entries.insert(key, MVEntry {
            values: vec![(value, version)],
        });
    }
    
    /// Get all values for a key (may be multiple if concurrent writes)
    pub fn get(&self, key: &K) -> Vec<&V> {
        self.entries.get(key)
            .map(|e| e.values.iter().map(|(v, _)| v).collect())
            .unwrap_or_default()
    }
    
    /// Get single value (if only one, or None if conflict)
    pub fn get_single(&self, key: &K) -> Option<&V> {
        let values = self.get(key);
        if values.len() == 1 {
            values.into_iter().next()
        } else {
            None
        }
    }
    
    /// Check if key has conflict (multiple values)
    pub fn has_conflict(&self, key: &K) -> bool {
        self.get(key).len() > 1
    }
    
    /// Resolve conflict by choosing a value
    pub fn resolve(&mut self, key: &K, value: V, node_id: &str) {
        self.set(key.clone(), value, node_id);
    }
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone + PartialEq> StateCrdt for MVMap<K, V> {
    fn merge(&mut self, other: &Self) {
        // Merge clocks
        for (node, &ts) in &other.clocks {
            let my_ts = self.clocks.entry(node.clone()).or_insert(0);
            *my_ts = (*my_ts).max(ts);
        }
        
        // Merge entries
        for (key, other_entry) in &other.entries {
            if let Some(my_entry) = self.entries.get_mut(key) {
                // Add non-dominated values from other
                for (other_val, other_ver) in &other_entry.values {
                    let dominated = my_entry.values.iter().any(|(_, my_ver)| {
                        dominates(my_ver, other_ver)
                    });
                    
                    if !dominated {
                        // Remove values dominated by this one
                        my_entry.values.retain(|(_, my_ver)| {
                            !dominates(other_ver, my_ver)
                        });
                        
                        // Add if not already present
                        if !my_entry.values.iter().any(|(v, _)| v == other_val) {
                            my_entry.values.push((other_val.clone(), other_ver.clone()));
                        }
                    }
                }
            } else {
                self.entries.insert(key.clone(), other_entry.clone());
            }
        }
    }
    
    fn value(&self) -> String {
        format!("MVMap({} entries)", self.entries.len())
    }
}

/// Check if version a dominates version b
fn dominates(a: &HashMap<NodeId, Timestamp>, b: &HashMap<NodeId, Timestamp>) -> bool {
    if a == b {
        return false;
    }
    
    // Check all entries in b are <= corresponding entries in a
    for (node, &ts_b) in b {
        let ts_a = a.get(node).copied().unwrap_or(0);
        if ts_b > ts_a {
            return false;
        }
    }
    
    // Check at least one entry in a is > b
    for (node, &ts_a) in a {
        let ts_b = b.get(node).copied().unwrap_or(0);
        if ts_a > ts_b {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lww_map() {
        let mut m1: LWWMap<String, i32> = LWWMap::new();
        let mut m2: LWWMap<String, i32> = LWWMap::new();
        
        m1.set("x".to_string(), 1, "node1", 1);
        m2.set("x".to_string(), 2, "node2", 2);
        
        m1.merge(&m2);
        
        // node2 wins (higher timestamp)
        assert_eq!(m1.get(&"x".to_string()), Some(&2));
    }
    
    #[test]
    fn test_lww_map_same_timestamp() {
        let mut m1: LWWMap<String, i32> = LWWMap::new();
        let mut m2: LWWMap<String, i32> = LWWMap::new();
        
        m1.set("x".to_string(), 1, "node1", 1);
        m2.set("x".to_string(), 2, "node2", 1);
        
        m1.merge(&m2);
        
        // node2 wins (lexicographically later)
        assert_eq!(m1.get(&"x".to_string()), Some(&2));
    }
    
    #[test]
    fn test_mv_map_conflict() {
        let mut m1: MVMap<String, i32> = MVMap::new();
        let mut m2: MVMap<String, i32> = MVMap::new();
        
        m1.set("x".to_string(), 1, "node1");
        m2.set("x".to_string(), 2, "node2");
        
        m1.merge(&m2);
        
        // Both values should be present (conflict)
        let values = m1.get(&"x".to_string());
        assert_eq!(values.len(), 2);
    }
}
