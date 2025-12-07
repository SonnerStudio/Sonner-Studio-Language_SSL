//! SSL 4.0 Linear Types
//!
//! Ownership and borrowing system (Rust-inspired).

use std::collections::HashMap;

/// Ownership qualifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ownership {
    /// Owned - exclusive access, will be dropped
    Owned,
    /// Shared borrow - immutable reference
    Borrowed,
    /// Mutable borrow - exclusive mutable reference
    BorrowedMut,
    /// Copy - can be freely copied
    Copy,
}

/// Linear type annotation
#[derive(Debug, Clone)]
pub struct LinearType {
    pub base_type: String,
    pub ownership: Ownership,
    pub lifetime: Option<String>,
}

impl LinearType {
    pub fn owned(ty: &str) -> Self {
        Self {
            base_type: ty.to_string(),
            ownership: Ownership::Owned,
            lifetime: None,
        }
    }
    
    pub fn borrowed(ty: &str, lifetime: Option<&str>) -> Self {
        Self {
            base_type: ty.to_string(),
            ownership: Ownership::Borrowed,
            lifetime: lifetime.map(String::from),
        }
    }
    
    pub fn borrowed_mut(ty: &str, lifetime: Option<&str>) -> Self {
        Self {
            base_type: ty.to_string(),
            ownership: Ownership::BorrowedMut,
            lifetime: lifetime.map(String::from),
        }
    }
}

/// Borrow state for a variable
#[derive(Debug, Clone)]
pub struct BorrowState {
    /// Number of active shared borrows
    pub shared_borrows: usize,
    /// Has active mutable borrow
    pub mutable_borrow: bool,
    /// Is the value moved
    pub moved: bool,
}

impl Default for BorrowState {
    fn default() -> Self {
        Self {
            shared_borrows: 0,
            mutable_borrow: false,
            moved: false,
        }
    }
}

/// Borrow checker
pub struct BorrowChecker {
    /// Variable states
    states: HashMap<String, BorrowState>,
    /// Errors found
    errors: Vec<BorrowError>,
}

/// Borrow checker errors
#[derive(Debug, Clone)]
pub enum BorrowError {
    UseAfterMove { variable: String, location: String },
    BorrowWhileMutablyBorrowed { variable: String },
    MutBorrowWhileBorrowed { variable: String },
    DoubleMutBorrow { variable: String },
    MoveWhileBorrowed { variable: String },
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            errors: Vec::new(),
        }
    }
    
    /// Declare a new variable
    pub fn declare(&mut self, name: &str) {
        self.states.insert(name.to_string(), BorrowState::default());
    }
    
    /// Use a variable (read)
    pub fn use_var(&mut self, name: &str, location: &str) -> bool {
        if let Some(state) = self.states.get(name) {
            if state.moved {
                self.errors.push(BorrowError::UseAfterMove {
                    variable: name.to_string(),
                    location: location.to_string(),
                });
                return false;
            }
        }
        true
    }
    
    /// Move a variable
    pub fn move_var(&mut self, name: &str) -> bool {
        if let Some(state) = self.states.get_mut(name) {
            if state.shared_borrows > 0 || state.mutable_borrow {
                self.errors.push(BorrowError::MoveWhileBorrowed {
                    variable: name.to_string(),
                });
                return false;
            }
            state.moved = true;
        }
        true
    }
    
    /// Create a shared borrow
    pub fn borrow(&mut self, name: &str) -> bool {
        if let Some(state) = self.states.get_mut(name) {
            if state.moved {
                return false;
            }
            if state.mutable_borrow {
                self.errors.push(BorrowError::BorrowWhileMutablyBorrowed {
                    variable: name.to_string(),
                });
                return false;
            }
            state.shared_borrows += 1;
        }
        true
    }
    
    /// Create a mutable borrow
    pub fn borrow_mut(&mut self, name: &str) -> bool {
        if let Some(state) = self.states.get_mut(name) {
            if state.moved {
                return false;
            }
            if state.shared_borrows > 0 {
                self.errors.push(BorrowError::MutBorrowWhileBorrowed {
                    variable: name.to_string(),
                });
                return false;
            }
            if state.mutable_borrow {
                self.errors.push(BorrowError::DoubleMutBorrow {
                    variable: name.to_string(),
                });
                return false;
            }
            state.mutable_borrow = true;
        }
        true
    }
    
    /// Release a shared borrow
    pub fn release_borrow(&mut self, name: &str) {
        if let Some(state) = self.states.get_mut(name) {
            if state.shared_borrows > 0 {
                state.shared_borrows -= 1;
            }
        }
    }
    
    /// Release a mutable borrow
    pub fn release_mut_borrow(&mut self, name: &str) {
        if let Some(state) = self.states.get_mut(name) {
            state.mutable_borrow = false;
        }
    }
    
    /// Get all errors
    pub fn errors(&self) -> &[BorrowError] {
        &self.errors
    }
    
    /// Check if there are errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Drop trait for linear types
pub trait Drop {
    fn drop(&mut self);
}

/// Resource that must be explicitly closed
pub struct LinearResource<T> {
    value: Option<T>,
    dropped: bool,
}

impl<T> LinearResource<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            dropped: false,
        }
    }
    
    pub fn consume(mut self) -> T {
        self.dropped = true;
        self.value.take().expect("Resource already consumed")
    }
    
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> std::ops::Drop for LinearResource<T> {
    fn drop(&mut self) {
        if !self.dropped && self.value.is_some() {
            // In a real implementation, this would be a compile-time error
            eprintln!("Warning: LinearResource dropped without being consumed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_borrow_checker() {
        let mut checker = BorrowChecker::new();
        
        checker.declare("x");
        
        // Can borrow multiple times
        assert!(checker.borrow("x"));
        assert!(checker.borrow("x"));
        
        // Can't mut borrow while borrowed
        assert!(!checker.borrow_mut("x"));
        
        checker.release_borrow("x");
        checker.release_borrow("x");
        
        // Now can mut borrow
        assert!(checker.borrow_mut("x"));
    }
    
    #[test]
    fn test_move_detection() {
        let mut checker = BorrowChecker::new();
        
        checker.declare("x");
        checker.move_var("x");
        
        assert!(!checker.use_var("x", "line 10"));
        assert!(checker.has_errors());
    }
    
    #[test]
    fn test_linear_resource() {
        let resource = LinearResource::new(42);
        let value = resource.consume();
        assert_eq!(value, 42);
    }
}
