use crate::ast::Statement;
use crate::interpreter::{Environment, Value};
use crate::parser::Parser;
use std::collections::HashMap;

/// Replaces functions at runtime (hot swap)
pub struct FunctionReplacer {
    /// Track function versions
    versions: HashMap<String, usize>,
}

impl FunctionReplacer {
    /// Create a new function replacer
    pub fn new() -> Self {
        FunctionReplacer {
            versions: HashMap::new(),
        }
    }
    
    /// Parse and extract function definitions from source
    pub fn extract_functions(&self, source: &str) -> Result<Vec<(String, Value)>, String> {
        let mut parser = Parser::new(source);
        let statements = parser.parse()?;
        
        let mut functions = Vec::new();
        
        for stmt in statements {
            if let Statement::FunctionDecl(func) = stmt {
                let name = func.name.clone();
                let params: Vec<String> = func.params.iter().map(|p| p.name.clone()).collect();
                
                let func_value = Value::Function {
                    type_params: func.type_params,
                    params,
                    body: func.body,
                    is_async: func.is_async,
                };
                
                functions.push((name, func_value));
            }
        }
        
        Ok(functions)
    }
    
    /// Replace functions in environment
    pub fn replace_functions(
        &mut self,
        env: &mut Environment,
        new_functions: Vec<(String, Value)>,
    ) -> Vec<String> {
        let mut replaced = Vec::new();
        
        for (name, func) in new_functions {
            // Update version
            let version = self.versions.entry(name.clone()).or_insert(0);
            *version += 1;
            
            // Replace in environment (top-level scope)
            env.define(name.clone(), func);
            replaced.push(format!("{} (v{})", name, version));
        }
        
        replaced
    }
    
    /// Get function version
    pub fn get_version(&self, name: &str) -> usize {
        self.versions.get(name).copied().unwrap_or(0)
    }
    
    /// Hot reload from source file
    pub fn reload_from_source(
        &mut self,
        env: &mut Environment,
        source: &str,
    ) -> Result<Vec<String>, String> {
        let functions = self.extract_functions(source)?;
        Ok(self.replace_functions(env, functions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_extraction() {
        let source = r#"
            fn add(a: Int, b: Int) -> Int {
                return a + b
            }
            
            fn multiply(x: Int, y: Int) -> Int {
                return x * y
            }
        "#;
        
        let replacer = FunctionReplacer::new();
        let functions = replacer.extract_functions(source).unwrap();
        
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].0, "add");
        assert_eq!(functions[1].0, "multiply");
    }
    
    #[test]
    fn test_function_replacement() {
        let mut env = Environment::new();
        let mut replacer = FunctionReplacer::new();
        
        let source = r#"
            fn test() -> Int {
                return 42
            }
        "#;
        
        let replaced = replacer.reload_from_source(&mut env, source).unwrap();
        assert_eq!(replaced.len(), 1);
        assert!(replaced[0].contains("test"));
        
        // Verify function exists in environment
        assert!(env.get("test").is_ok());
    }
}
