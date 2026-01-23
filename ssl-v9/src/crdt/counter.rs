//! CRDT Counter implementations
//!
//! G-Counter (grow-only) and PN-Counter (positive-negative)

use super::{NodeId, StateCrdt};
use std::collections::HashMap;

/// G-Counter - Grow-only counter
#[derive(Debug, Clone, Default)]
pub struct GCounter {
    counts: HashMap<NodeId, u64>,
}

impl GCounter {
    /// Create a new G-Counter
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
    
    /// Increment the counter for a node
    pub fn increment(&mut self, node_id: &str) {
        let count = self.counts.entry(node_id.to_string()).or_insert(0);
        *count += 1;
    }
    
    /// Increment by a specific amount
    pub fn increment_by(&mut self, node_id: &str, amount: u64) {
        let count = self.counts.entry(node_id.to_string()).or_insert(0);
        *count += amount;
    }
    
    /// Get the total count
    pub fn count(&self) -> u64 {
        self.counts.values().sum()
    }
    
    /// Get count for a specific node
    pub fn node_count(&self, node_id: &str) -> u64 {
        self.counts.get(node_id).copied().unwrap_or(0)
    }
}

impl StateCrdt for GCounter {
    fn merge(&mut self, other: &Self) {
        for (node, &count) in &other.counts {
            let current = self.counts.entry(node.clone()).or_insert(0);
            *current = (*current).max(count);
        }
    }
    
    fn value(&self) -> String {
        self.count().to_string()
    }
}

/// PN-Counter - Positive-Negative counter (supports decrement)
#[derive(Debug, Clone, Default)]
pub struct PNCounter {
    positive: GCounter,
    negative: GCounter,
}

impl PNCounter {
    /// Create a new PN-Counter
    pub fn new() -> Self {
        Self {
            positive: GCounter::new(),
            negative: GCounter::new(),
        }
    }
    
    /// Increment the counter
    pub fn increment(&mut self, node_id: &str) {
        self.positive.increment(node_id);
    }
    
    /// Decrement the counter
    pub fn decrement(&mut self, node_id: &str) {
        self.negative.increment(node_id);
    }
    
    /// Get the current value (can be negative)
    pub fn count(&self) -> i64 {
        self.positive.count() as i64 - self.negative.count() as i64
    }
}

impl StateCrdt for PNCounter {
    fn merge(&mut self, other: &Self) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }
    
    fn value(&self) -> String {
        self.count().to_string()
    }
}

/// Bounded Counter - with limits
#[derive(Debug, Clone)]
pub struct BoundedCounter {
    counter: PNCounter,
    min: i64,
    max: i64,
}

impl BoundedCounter {
    /// Create a bounded counter
    pub fn new(min: i64, max: i64) -> Self {
        Self {
            counter: PNCounter::new(),
            min,
            max,
        }
    }
    
    /// Try to increment (returns false if at max)
    pub fn try_increment(&mut self, node_id: &str) -> bool {
        if self.counter.count() < self.max {
            self.counter.increment(node_id);
            true
        } else {
            false
        }
    }
    
    /// Try to decrement (returns false if at min)
    pub fn try_decrement(&mut self, node_id: &str) -> bool {
        if self.counter.count() > self.min {
            self.counter.decrement(node_id);
            true
        } else {
            false
        }
    }
    
    /// Get current value
    pub fn count(&self) -> i64 {
        self.counter.count().clamp(self.min, self.max)
    }
}

impl StateCrdt for BoundedCounter {
    fn merge(&mut self, other: &Self) {
        self.counter.merge(&other.counter);
    }
    
    fn value(&self) -> String {
        self.count().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_g_counter() {
        let mut c1 = GCounter::new();
        let mut c2 = GCounter::new();
        
        c1.increment("node1");
        c1.increment("node1");
        c2.increment("node2");
        c2.increment("node2");
        c2.increment("node2");
        
        assert_eq!(c1.count(), 2);
        assert_eq!(c2.count(), 3);
        
        c1.merge(&c2);
        assert_eq!(c1.count(), 5);
    }
    
    #[test]
    fn test_pn_counter() {
        let mut c1 = PNCounter::new();
        let mut c2 = PNCounter::new();
        
        c1.increment("node1");
        c1.increment("node1");
        c1.decrement("node1");
        
        c2.increment("node2");
        c2.decrement("node2");
        c2.decrement("node2");
        
        assert_eq!(c1.count(), 1);
        assert_eq!(c2.count(), -1);
        
        c1.merge(&c2);
        assert_eq!(c1.count(), 0);
    }
    
    #[test]
    fn test_bounded_counter() {
        let mut counter = BoundedCounter::new(0, 5);
        
        // Increment to max
        for _ in 0..10 {
            counter.try_increment("node1");
        }
        assert_eq!(counter.count(), 5);
        
        // Try to go over
        assert!(!counter.try_increment("node1"));
    }
}
