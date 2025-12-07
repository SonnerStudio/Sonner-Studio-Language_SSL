//! CRDT Set implementations
//!
//! G-Set (grow-only), 2P-Set, OR-Set

use super::{NodeId, Timestamp, StateCrdt};
use std::collections::{HashSet, HashMap};

/// G-Set - Grow-only set
#[derive(Debug, Clone, Default)]
pub struct GSet<T: Clone + Eq + std::hash::Hash> {
    elements: HashSet<T>,
}

impl<T: Clone + Eq + std::hash::Hash> GSet<T> {
    /// Create a new G-Set
    pub fn new() -> Self {
        Self {
            elements: HashSet::new(),
        }
    }
    
    /// Add an element
    pub fn insert(&mut self, value: T) {
        self.elements.insert(value);
    }
    
    /// Check if element exists
    pub fn contains(&self, value: &T) -> bool {
        self.elements.contains(value)
    }
    
    /// Get all elements
    pub fn elements(&self) -> Vec<&T> {
        self.elements.iter().collect()
    }
    
    /// Get size
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T: Clone + Eq + std::hash::Hash> StateCrdt for GSet<T> {
    fn merge(&mut self, other: &Self) {
        for elem in &other.elements {
            self.elements.insert(elem.clone());
        }
    }
    
    fn value(&self) -> String {
        format!("GSet({} elements)", self.len())
    }
}

/// 2P-Set - Two-Phase Set (supports removal, but once removed, can't be re-added)
#[derive(Debug, Clone, Default)]
pub struct TwoPSet<T: Clone + Eq + std::hash::Hash> {
    added: GSet<T>,
    removed: GSet<T>,
}

impl<T: Clone + Eq + std::hash::Hash> TwoPSet<T> {
    /// Create a new 2P-Set
    pub fn new() -> Self {
        Self {
            added: GSet::new(),
            removed: GSet::new(),
        }
    }
    
    /// Add an element (only if not previously removed)
    pub fn insert(&mut self, value: T) -> bool {
        if !self.removed.contains(&value) {
            self.added.insert(value);
            true
        } else {
            false
        }
    }
    
    /// Remove an element
    pub fn remove(&mut self, value: T) -> bool {
        if self.added.contains(&value) {
            self.removed.insert(value);
            true
        } else {
            false
        }
    }
    
    /// Check if element exists
    pub fn contains(&self, value: &T) -> bool {
        self.added.contains(value) && !self.removed.contains(value)
    }
    
    /// Get all current elements
    pub fn elements(&self) -> Vec<&T> {
        self.added.elements()
            .into_iter()
            .filter(|e| !self.removed.contains(e))
            .collect()
    }
}

impl<T: Clone + Eq + std::hash::Hash> StateCrdt for TwoPSet<T> {
    fn merge(&mut self, other: &Self) {
        self.added.merge(&other.added);
        self.removed.merge(&other.removed);
    }
    
    fn value(&self) -> String {
        format!("2PSet({} elements)", self.elements().len())
    }
}

/// OR-Set - Observed-Remove Set (supports add/remove with proper semantics)
#[derive(Debug, Clone)]
pub struct ORSet<T: Clone + Eq + std::hash::Hash> {
    /// Elements with their unique tags
    elements: HashMap<T, HashSet<(NodeId, Timestamp)>>,
    /// Next timestamp for this node
    next_timestamp: HashMap<NodeId, Timestamp>,
}

impl<T: Clone + Eq + std::hash::Hash> Default for ORSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Eq + std::hash::Hash> ORSet<T> {
    /// Create a new OR-Set
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            next_timestamp: HashMap::new(),
        }
    }
    
    /// Add an element
    pub fn insert(&mut self, node_id: &str, value: T) {
        let ts = self.next_timestamp.entry(node_id.to_string()).or_insert(0);
        *ts += 1;
        
        let tags = self.elements.entry(value).or_insert_with(HashSet::new);
        tags.insert((node_id.to_string(), *ts));
    }
    
    /// Remove an element (removes all observed tags)
    pub fn remove(&mut self, value: &T) {
        self.elements.remove(value);
    }
    
    /// Check if element exists
    pub fn contains(&self, value: &T) -> bool {
        self.elements.get(value)
            .map(|tags| !tags.is_empty())
            .unwrap_or(false)
    }
    
    /// Get all elements
    pub fn elements(&self) -> Vec<&T> {
        self.elements
            .iter()
            .filter(|(_, tags)| !tags.is_empty())
            .map(|(elem, _)| elem)
            .collect()
    }
    
    /// Get size
    pub fn len(&self) -> usize {
        self.elements().len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone + Eq + std::hash::Hash> StateCrdt for ORSet<T> {
    fn merge(&mut self, other: &Self) {
        // Merge elements
        for (elem, tags) in &other.elements {
            let my_tags = self.elements.entry(elem.clone()).or_insert_with(HashSet::new);
            for tag in tags {
                my_tags.insert(tag.clone());
            }
        }
        
        // Merge timestamps
        for (node, &ts) in &other.next_timestamp {
            let my_ts = self.next_timestamp.entry(node.clone()).or_insert(0);
            *my_ts = (*my_ts).max(ts);
        }
    }
    
    fn value(&self) -> String {
        format!("ORSet({} elements)", self.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_g_set() {
        let mut s1: GSet<i32> = GSet::new();
        let mut s2: GSet<i32> = GSet::new();
        
        s1.insert(1);
        s1.insert(2);
        s2.insert(2);
        s2.insert(3);
        
        s1.merge(&s2);
        
        assert!(s1.contains(&1));
        assert!(s1.contains(&2));
        assert!(s1.contains(&3));
        assert_eq!(s1.len(), 3);
    }
    
    #[test]
    fn test_2p_set() {
        let mut set: TwoPSet<String> = TwoPSet::new();
        
        set.insert("a".to_string());
        set.insert("b".to_string());
        assert!(set.contains(&"a".to_string()));
        
        set.remove("a".to_string());
        assert!(!set.contains(&"a".to_string()));
        
        // Can't re-add after removal
        assert!(!set.insert("a".to_string()));
    }
    
    #[test]
    fn test_or_set() {
        let mut s1: ORSet<String> = ORSet::new();
        let mut s2: ORSet<String> = ORSet::new();
        
        s1.insert("node1", "a".to_string());
        s2.insert("node2", "b".to_string());
        
        s1.merge(&s2);
        
        assert!(s1.contains(&"a".to_string()));
        assert!(s1.contains(&"b".to_string()));
        
        s1.remove(&"a".to_string());
        assert!(!s1.contains(&"a".to_string()));
        
        // Can re-add after removal
        s1.insert("node1", "a".to_string());
        assert!(s1.contains(&"a".to_string()));
    }
}
