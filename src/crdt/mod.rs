//! SSL 4.0 CRDT - Conflict-free Replicated Data Types
//!
//! Data structures that automatically resolve conflicts in distributed systems.

pub mod counter;
pub mod set;
pub mod text;
pub mod map;

use std::collections::HashMap;

/// Node ID for distributed systems
pub type NodeId = String;

/// Timestamp for ordering events
pub type Timestamp = u64;

/// Vector clock for causality tracking
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorClock {
    clocks: HashMap<NodeId, Timestamp>,
}

impl VectorClock {
    /// Create a new vector clock
    pub fn new() -> Self {
        Self {
            clocks: HashMap::new(),
        }
    }
    
    /// Increment clock for a node
    pub fn increment(&mut self, node_id: &str) {
        let count = self.clocks.entry(node_id.to_string()).or_insert(0);
        *count += 1;
    }
    
    /// Get timestamp for a node
    pub fn get(&self, node_id: &str) -> Timestamp {
        self.clocks.get(node_id).copied().unwrap_or(0)
    }
    
    /// Merge with another vector clock (take max)
    pub fn merge(&mut self, other: &VectorClock) {
        for (node, &time) in &other.clocks {
            let current = self.clocks.entry(node.clone()).or_insert(0);
            *current = (*current).max(time);
        }
    }
    
    /// Check if this clock happens-before another
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        let mut dominated = false;
        
        for (node, &time) in &self.clocks {
            let other_time = other.get(node);
            if time > other_time {
                return false;
            }
            if time < other_time {
                dominated = true;
            }
        }
        
        // Check for nodes only in other
        for (node, &time) in &other.clocks {
            if !self.clocks.contains_key(node) && time > 0 {
                dominated = true;
            }
        }
        
        dominated
    }
    
    /// Check if clocks are concurrent (neither happens-before)
    pub fn concurrent(&self, other: &VectorClock) -> bool {
        !self.happens_before(other) && !other.happens_before(self) && self != other
    }
}

/// CRDT operation for sync
#[derive(Debug, Clone)]
pub struct CrdtOperation<T> {
    /// Source node
    pub node_id: NodeId,
    /// Timestamp
    pub timestamp: Timestamp,
    /// Operation type
    pub op_type: OperationType<T>,
}

/// Types of CRDT operations
#[derive(Debug, Clone)]
pub enum OperationType<T> {
    /// Insert a value
    Insert(T),
    /// Delete a value
    Delete(T),
    /// Increment
    Increment(i64),
    /// Decrement
    Decrement(i64),
    /// Set value
    Set(T),
}

/// Trait for state-based CRDTs (CvRDT)
pub trait StateCrdt: Sized {
    /// Merge another replica's state
    fn merge(&mut self, other: &Self);
    
    /// Get the current value
    fn value(&self) -> String;
}

/// Trait for operation-based CRDTs (CmRDT)
pub trait OpCrdt<Op> {
    /// Apply an operation
    fn apply(&mut self, op: Op);
    
    /// Generate an operation for an action
    fn generate(&mut self, action: &str) -> Op;
}

/// Replica wrapper for CRDTs
pub struct Replica<T: StateCrdt> {
    /// Node ID
    pub node_id: NodeId,
    /// The CRDT data
    pub data: T,
    /// Vector clock
    pub clock: VectorClock,
    /// Pending operations for sync
    pending_ops: Vec<CrdtOperation<String>>,
}

impl<T: StateCrdt + Clone> Replica<T> {
    /// Create a new replica
    pub fn new(node_id: &str, initial: T) -> Self {
        Self {
            node_id: node_id.to_string(),
            data: initial,
            clock: VectorClock::new(),
            pending_ops: Vec::new(),
        }
    }
    
    /// Sync with another replica
    pub fn sync(&mut self, other: &mut Replica<T>) {
        self.data.merge(&other.data);
        other.data.merge(&self.data);
        
        self.clock.merge(&other.clock);
        other.clock.merge(&self.clock);
    }
    
    /// Get the current value
    pub fn value(&self) -> String {
        self.data.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_clock() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();
        
        vc1.increment("node1");
        vc1.increment("node1");
        vc2.increment("node2");
        
        assert!(vc1.concurrent(&vc2));
        
        vc1.merge(&vc2);
        assert_eq!(vc1.get("node1"), 2);
        assert_eq!(vc1.get("node2"), 1);
    }
    
    #[test]
    fn test_happens_before() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();
        
        vc1.increment("node1");
        vc2.merge(&vc1);
        vc2.increment("node2");
        
        assert!(vc1.happens_before(&vc2));
        assert!(!vc2.happens_before(&vc1));
    }
}
