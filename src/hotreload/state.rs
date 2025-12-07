use crate::interpreter::{Environment, Value};
use std::collections::HashMap;

/// Migrates state between code versions
pub struct StateMigrator {
    /// Preserved variables
    state: HashMap<String, Value>,
}

impl StateMigrator {
    /// Create a new state migrator
    pub fn new() -> Self {
        StateMigrator {
            state: HashMap::new(),
        }
    }
    
    /// Capture current state from environment
    pub fn capture(&mut self, env: &Environment) {
        self.state.clear();
        
        // Capture all scopes (flatten into single map, innermost wins)
        for scope in &env.scopes {
            for (name, (value, _)) in scope {
                self.state.insert(name.clone(), value.clone());
            }
        }
    }
    
    /// Restore state to environment
    pub fn restore(&self, env: &mut Environment) {
        // Restore to current scope
        for (name, value) in &self.state {
            env.define(name.clone(), value.clone());
        }
    }
    
    /// Get preserved state
    pub fn get_state(&self) -> &HashMap<String, Value> {
        &self.state
    }
    
    /// Check if variable exists in preserved state
    pub fn has(&self, name: &str) -> bool {
        self.state.contains_key(name)
    }
    
    /// Get variable from preserved state
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.state.get(name)
    }
    
    /// Clear preserved state
    pub fn clear(&mut self) {
        self.state.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_migration() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Int(42));
        env.define("y".to_string(), Value::String("hello".to_string()));
        
        let mut migrator = StateMigrator::new();
        migrator.capture(&env);
        
        assert!(migrator.has("x"));
        assert!(migrator.has("y"));
        assert_eq!(migrator.get("x"), Some(&Value::Int(42)));
        
        // Create new environment and restore
        let mut new_env = Environment::new();
        migrator.restore(&mut new_env);
        
        assert_eq!(new_env.get("x").unwrap(), Value::Int(42));
        assert_eq!(new_env.get("y").unwrap(), Value::String("hello".to_string()));
    }
}
