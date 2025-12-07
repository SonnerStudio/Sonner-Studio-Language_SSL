//! CRDT Text for collaborative editing

use super::{NodeId, Timestamp, StateCrdt};
use std::collections::BTreeMap;

/// Unique identifier for a character position
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharId {
    /// Node that inserted the character
    pub node: NodeId,
    /// Sequence number at that node
    pub seq: u64,
}

impl CharId {
    pub fn new(node: &str, seq: u64) -> Self {
        Self {
            node: node.to_string(),
            seq,
        }
    }
}

/// Character with metadata for CRDT text
#[derive(Debug, Clone)]
pub struct CrdtChar {
    /// The character value
    pub value: char,
    /// Unique ID
    pub id: CharId,
    /// ID of the character this comes after
    pub after: Option<CharId>,
    /// Deleted flag (tombstone)
    pub deleted: bool,
}

/// RGA (Replicated Growable Array) based collaborative text
#[derive(Debug, Clone)]
pub struct CrdtText {
    /// All characters (including deleted)
    chars: BTreeMap<CharId, CrdtChar>,
    /// Ordered list of non-deleted char IDs
    order: Vec<CharId>,
    /// Next sequence number per node
    sequences: BTreeMap<NodeId, u64>,
}

impl Default for CrdtText {
    fn default() -> Self {
        Self::new()
    }
}

impl CrdtText {
    /// Create empty CRDT text
    pub fn new() -> Self {
        Self {
            chars: BTreeMap::new(),
            order: Vec::new(),
            sequences: BTreeMap::new(),
        }
    }
    
    /// Create from initial string
    pub fn from_string(node_id: &str, s: &str) -> Self {
        let mut text = Self::new();
        for c in s.chars() {
            text.insert(node_id, text.len(), c);
        }
        text
    }
    
    /// Get next sequence number for a node
    fn next_seq(&mut self, node_id: &str) -> u64 {
        let seq = self.sequences.entry(node_id.to_string()).or_insert(0);
        *seq += 1;
        *seq
    }
    
    /// Insert a character at position
    pub fn insert(&mut self, node_id: &str, position: usize, c: char) {
        let seq = self.next_seq(node_id);
        let id = CharId::new(node_id, seq);
        
        // Find the character we insert after
        let after = if position == 0 {
            None
        } else {
            self.order.get(position - 1).cloned()
        };
        
        let crdt_char = CrdtChar {
            value: c,
            id: id.clone(),
            after,
            deleted: false,
        };
        
        self.chars.insert(id.clone(), crdt_char);
        
        // Insert into order
        if position >= self.order.len() {
            self.order.push(id);
        } else {
            self.order.insert(position, id);
        }
    }
    
    /// Delete character at position
    pub fn delete(&mut self, position: usize) -> Option<char> {
        if position >= self.order.len() {
            return None;
        }
        
        let id = self.order.remove(position);
        if let Some(c) = self.chars.get_mut(&id) {
            c.deleted = true;
            Some(c.value)
        } else {
            None
        }
    }
    
    /// Get the current text
    pub fn text(&self) -> String {
        self.order
            .iter()
            .filter_map(|id| self.chars.get(id))
            .filter(|c| !c.deleted)
            .map(|c| c.value)
            .collect()
    }
    
    /// Get length of visible text
    pub fn len(&self) -> usize {
        self.order.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    
    /// Apply a remote insert operation
    pub fn apply_insert(&mut self, crdt_char: CrdtChar) {
        let id = crdt_char.id.clone();
        
        // Update sequence tracking
        let seq = self.sequences.entry(crdt_char.id.node.clone()).or_insert(0);
        *seq = (*seq).max(crdt_char.id.seq);
        
        // Find insertion point based on 'after' reference
        let position = match &crdt_char.after {
            None => 0,
            Some(after_id) => {
                self.order.iter()
                    .position(|id| id == after_id)
                    .map(|p| p + 1)
                    .unwrap_or(self.order.len())
            }
        };
        
        // Check if already exists
        if self.chars.contains_key(&id) {
            return;
        }
        
        self.chars.insert(id.clone(), crdt_char);
        
        if position >= self.order.len() {
            self.order.push(id);
        } else {
            self.order.insert(position, id);
        }
    }
    
    /// Apply a remote delete operation
    pub fn apply_delete(&mut self, id: &CharId) {
        if let Some(c) = self.chars.get_mut(id) {
            c.deleted = true;
            self.order.retain(|i| i != id);
        }
    }
}

impl StateCrdt for CrdtText {
    fn merge(&mut self, other: &Self) {
        // Merge all characters
        for (id, c) in &other.chars {
            if !self.chars.contains_key(id) {
                self.apply_insert(c.clone());
            } else if c.deleted {
                self.apply_delete(id);
            }
        }
        
        // Merge sequences
        for (node, &seq) in &other.sequences {
            let my_seq = self.sequences.entry(node.clone()).or_insert(0);
            *my_seq = (*my_seq).max(seq);
        }
    }
    
    fn value(&self) -> String {
        self.text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_insert() {
        let mut text = CrdtText::new();
        
        text.insert("node1", 0, 'H');
        text.insert("node1", 1, 'i');
        
        assert_eq!(text.text(), "Hi");
    }
    
    #[test]
    fn test_delete() {
        let mut text = CrdtText::from_string("node1", "Hello");
        
        text.delete(1); // Delete 'e'
        assert_eq!(text.text(), "Hllo");
    }
    
    #[test]
    fn test_concurrent_edits() {
        let mut t1 = CrdtText::from_string("node1", "Hello");
        let mut t2 = t1.clone();
        
        // Concurrent edits
        t1.insert("node1", 5, '!');
        t2.insert("node2", 0, '>');
        
        // Merge
        t1.merge(&t2);
        t2.merge(&t1);
        
        // Both should converge to same state
        assert_eq!(t1.text(), t2.text());
    }
}
