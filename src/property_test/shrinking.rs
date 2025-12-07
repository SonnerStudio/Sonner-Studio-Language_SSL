//! Shrinking strategies for property-based testing
//!
//! When a property test fails, shrinking finds the minimal
//! failing example to make debugging easier.

/// Shrink trait for reducing values to simpler forms
pub trait Shrink: Sized {
    /// Generate smaller versions of this value
    fn shrink(&self) -> Vec<Self>;
}

impl Shrink for i64 {
    fn shrink(&self) -> Vec<Self> {
        let mut results = Vec::new();
        let v = *self;
        
        if v == 0 {
            return results;
        }
        
        // Try 0
        results.push(0);
        
        // Try halving
        if v.abs() > 1 {
            results.push(v / 2);
        }
        
        // Try subtracting 1
        results.push(v - v.signum());
        
        // Try negating
        if v != 0 {
            results.push(-v);
        }
        
        results
    }
}

impl Shrink for i32 {
    fn shrink(&self) -> Vec<Self> {
        (*self as i64).shrink().into_iter().map(|x| x as i32).collect()
    }
}

impl Shrink for f64 {
    fn shrink(&self) -> Vec<Self> {
        let mut results = Vec::new();
        let v = *self;
        
        if v == 0.0 {
            return results;
        }
        
        // Try 0
        results.push(0.0);
        
        // Try truncating to integer
        results.push(v.trunc());
        
        // Try halving
        if v.abs() > 0.001 {
            results.push(v / 2.0);
        }
        
        results
    }
}

impl Shrink for bool {
    fn shrink(&self) -> Vec<Self> {
        if *self {
            vec![false]
        } else {
            vec![]
        }
    }
}

impl Shrink for String {
    fn shrink(&self) -> Vec<Self> {
        let mut results = Vec::new();
        
        if self.is_empty() {
            return results;
        }
        
        // Try empty
        results.push(String::new());
        
        // Try removing each character
        for i in 0..self.len() {
            let shrunk: String = self.chars()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, c)| c)
                .collect();
            results.push(shrunk);
        }
        
        // Try first half
        if self.len() > 1 {
            results.push(self.chars().take(self.len() / 2).collect());
        }
        
        // Try second half
        if self.len() > 1 {
            results.push(self.chars().skip(self.len() / 2).collect());
        }
        
        results
    }
}

impl<T: Shrink + Clone> Shrink for Vec<T> {
    fn shrink(&self) -> Vec<Self> {
        let mut results = Vec::new();
        
        if self.is_empty() {
            return results;
        }
        
        // Try empty
        results.push(Vec::new());
        
        // Try removing each element
        for i in 0..self.len() {
            let mut shrunk = self.clone();
            shrunk.remove(i);
            results.push(shrunk);
        }
        
        // Try shrinking each element
        for i in 0..self.len() {
            for shrunk_elem in self[i].shrink() {
                let mut shrunk = self.clone();
                shrunk[i] = shrunk_elem;
                results.push(shrunk);
            }
        }
        
        // Try first half
        if self.len() > 1 {
            results.push(self[..self.len()/2].to_vec());
        }
        
        // Try second half
        if self.len() > 1 {
            results.push(self[self.len()/2..].to_vec());
        }
        
        results
    }
}

impl<T: Shrink + Clone> Shrink for Option<T> {
    fn shrink(&self) -> Vec<Self> {
        match self {
            None => vec![],
            Some(v) => {
                let mut results = vec![None];
                for shrunk in v.shrink() {
                    results.push(Some(shrunk));
                }
                results
            }
        }
    }
}

impl<A: Shrink + Clone, B: Shrink + Clone> Shrink for (A, B) {
    fn shrink(&self) -> Vec<Self> {
        let mut results = Vec::new();
        
        // Shrink first element
        for a in self.0.shrink() {
            results.push((a, self.1.clone()));
        }
        
        // Shrink second element
        for b in self.1.shrink() {
            results.push((self.0.clone(), b));
        }
        
        results
    }
}

/// Shrink runner - finds minimal failing case
pub struct Shrinker {
    pub max_steps: usize,
}

impl Default for Shrinker {
    fn default() -> Self {
        Self { max_steps: 100 }
    }
}

impl Shrinker {
    /// Find the smallest value that still fails the property
    pub fn shrink_to_minimal<T, F>(&self, initial: T, property: F) -> (T, usize)
    where
        T: Shrink + Clone,
        F: Fn(&T) -> bool,
    {
        let mut current = initial;
        let mut steps = 0;
        
        for _ in 0..self.max_steps {
            let candidates = current.shrink();
            let mut found_smaller = false;
            
            for candidate in candidates {
                if !property(&candidate) {
                    current = candidate;
                    steps += 1;
                    found_smaller = true;
                    break;
                }
            }
            
            if !found_smaller {
                break;
            }
        }
        
        (current, steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_int_shrink() {
        let shrinks = 100i64.shrink();
        assert!(shrinks.contains(&0));
        assert!(shrinks.contains(&50));
    }
    
    #[test]
    fn test_string_shrink() {
        let shrinks = "hello".to_string().shrink();
        assert!(shrinks.contains(&String::new()));
        assert!(shrinks.iter().any(|s| s.len() == 4));
    }
    
    #[test]
    fn test_list_shrink() {
        let list = vec![1i64, 2, 3];
        let shrinks = list.shrink();
        assert!(shrinks.contains(&vec![]));
        assert!(shrinks.contains(&vec![1, 2]));
    }
    
    #[test]
    fn test_shrinker() {
        let shrinker = Shrinker::default();
        
        // Find smallest x where x > 5
        let (minimal, steps) = shrinker.shrink_to_minimal(100i64, |x| *x <= 5);
        assert_eq!(minimal, 6);
        assert!(steps > 0);
    }
}
