// Name Generator for creating unique identifiers

use std::sync::atomic::{AtomicUsize, Ordering};

/// Generates unique variable/function names
pub struct NameGenerator {
    counter: AtomicUsize,
}

impl NameGenerator {
    pub fn new() -> Self {
        NameGenerator {
            counter: AtomicUsize::new(0),
        }
    }
    
    /// Generates a unique function name
    pub fn generate_function_name(&self) -> String {
        let id = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("fn_{}", id)
    }
    
    /// Generates a unique variable name
    pub fn generate_variable_name(&self) -> String {
        let id = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("var_{}", id)
    }
    
    /// Generates a unique temporary name
    pub fn generate_temp_name(&self) -> String {
        let id = self.counter.fetch_add(1, Ordering::SeqCst);
        format!("tmp_{}", id)
    }
    
    /// Resets the counter (useful for testing)
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
    }
}

impl Default for NameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_function_names() {
        let gen = NameGenerator::new();
        
        let name1 = gen.generate_function_name();
        let name2 = gen.generate_function_name();
        
        assert_eq!(name1, "fn_0");
        assert_eq!(name2, "fn_1");
        assert_ne!(name1, name2);
    }
    
    #[test]
    fn test_generate_variable_names() {
        let gen = NameGenerator::new();
        
        let name1 = gen.generate_variable_name();
        let name2 = gen.generate_variable_name();
        
        assert_eq!(name1, "var_0");
        assert_eq!(name2, "var_1");
    }
    
    #[test]
    fn test_reset() {
        let gen = NameGenerator::new();
        
        gen.generate_function_name();
        gen.generate_function_name();
        gen.reset();
        
        let name = gen.generate_function_name();
        assert_eq!(name, "fn_0");
    }
}
