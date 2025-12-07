//! Arbitrary trait for property-based testing
//!
//! Common test property patterns for SSL programs.

/// Common test property patterns
pub mod properties {
    /// Identity: f(f(x)) == x
    pub fn is_involution<T: Clone + PartialEq, F>(f: F, x: &T) -> bool
    where
        F: Fn(&T) -> T,
    {
        f(&f(x)) == *x
    }
    
    /// Idempotent: f(f(x)) == f(x)
    pub fn is_idempotent<T: Clone + PartialEq, F>(f: F, x: &T) -> bool
    where
        F: Fn(&T) -> T,
    {
        let fx = f(x);
        f(&fx) == fx
    }
    
    /// Preserves length
    pub fn preserves_length<T: Clone, F>(f: F, list: &Vec<T>) -> bool
    where
        F: Fn(&Vec<T>) -> Vec<T>,
    {
        f(list).len() == list.len()
    }
    
    /// Commutative: f(a, b) == f(b, a)
    pub fn is_commutative<T: Clone, R: PartialEq, F>(f: F, a: &T, b: &T) -> bool
    where
        F: Fn(&T, &T) -> R,
    {
        f(a, b) == f(b, a)
    }
    
    /// Associative: f(f(a, b), c) == f(a, f(b, c))
    pub fn is_associative<T: Clone, F>(f: F, a: &T, b: &T, c: &T) -> bool
    where
        F: Fn(&T, &T) -> T,
        T: PartialEq,
    {
        let left = f(&f(a, b), c);
        let right = f(a, &f(b, c));
        left == right
    }
    
    /// Monotonic: a <= b implies f(a) <= f(b)
    pub fn is_monotonic<T: Ord, R: Ord, F>(f: F, a: &T, b: &T) -> bool
    where
        F: Fn(&T) -> R,
    {
        if a <= b {
            f(a) <= f(b)
        } else {
            true // Only check when a <= b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::properties::*;
    
    #[test]
    fn test_involution() {
        // Negation is an involution
        assert!(is_involution(|x: &i64| -x, &5));
        assert!(is_involution(|x: &i64| -x, &-3));
    }
    
    #[test]
    fn test_idempotent() {
        // abs is idempotent
        assert!(is_idempotent(|x: &i64| x.abs(), &-5));
        assert!(is_idempotent(|x: &i64| x.abs(), &5));
    }
    
    #[test]
    fn test_commutative() {
        // Addition is commutative
        assert!(is_commutative(|a: &i64, b: &i64| a + b, &3, &5));
    }
    
    #[test]
    fn test_associative() {
        // Addition is associative
        assert!(is_associative(|a: &i64, b: &i64| a + b, &1, &2, &3));
    }
}
