//! SSL 4.0 Algebraic Effects
//!
//! Effect system for controlled side effects (Koka-inspired).

use std::collections::HashMap;
use std::any::Any;

/// Effect definition
#[derive(Debug, Clone)]
pub struct Effect {
    /// Effect name
    pub name: String,
    /// Operations in this effect
    pub operations: Vec<EffectOperation>,
}

/// An operation within an effect
#[derive(Debug, Clone)]
pub struct EffectOperation {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}

impl Effect {
    /// Create a new effect
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            operations: Vec::new(),
        }
    }
    
    /// Add an operation
    pub fn with_op(mut self, name: &str, params: Vec<(&str, &str)>, ret: &str) -> Self {
        self.operations.push(EffectOperation {
            name: name.to_string(),
            params: params.into_iter().map(|(n, t)| (n.to_string(), t.to_string())).collect(),
            return_type: ret.to_string(),
        });
        self
    }
}

/// Built-in effects
pub mod builtin {
    use super::*;
    
    /// Console effect for I/O
    pub fn console_effect() -> Effect {
        Effect::new("Console")
            .with_op("print", vec![("msg", "String")], "Unit")
            .with_op("read", vec![], "String")
    }
    
    /// State effect
    pub fn state_effect(t: &str) -> Effect {
        Effect::new(&format!("State<{}>", t))
            .with_op("get", vec![], t)
            .with_op("set", vec![("value", t)], "Unit")
    }
    
    /// Error effect
    pub fn error_effect(e: &str) -> Effect {
        Effect::new(&format!("Error<{}>", e))
            .with_op("raise", vec![("error", e)], "Nothing")
    }
    
    /// Async effect
    pub fn async_effect() -> Effect {
        Effect::new("Async")
            .with_op("await", vec![("future", "Future<T>")], "T")
            .with_op("spawn", vec![("action", "() -> T")], "Future<T>")
    }
    
    /// Random effect
    pub fn random_effect() -> Effect {
        Effect::new("Random")
            .with_op("random", vec![], "Float")
            .with_op("random_int", vec![("min", "Int"), ("max", "Int")], "Int")
    }
}

/// Effect handler
pub trait EffectHandler: Send + Sync {
    /// Handle an effect operation
    fn handle(&self, op: &str, args: Vec<Box<dyn Any>>) -> Box<dyn Any>;
    
    /// Get the effect name this handler handles
    fn effect_name(&self) -> &str;
}

/// Console handler implementation
pub struct ConsoleHandler;

impl EffectHandler for ConsoleHandler {
    fn handle(&self, op: &str, args: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        match op {
            "print" => {
                if let Some(msg) = args.first().and_then(|a| a.downcast_ref::<String>()) {
                    println!("{}", msg);
                }
                Box::new(())
            }
            "read" => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok();
                Box::new(input.trim().to_string())
            }
            _ => Box::new(())
        }
    }
    
    fn effect_name(&self) -> &str {
        "Console"
    }
}

/// State handler implementation
pub struct StateHandler<T: Clone + Send + Sync + 'static> {
    state: std::sync::Mutex<T>,
}

impl<T: Clone + Send + Sync + 'static> StateHandler<T> {
    pub fn new(initial: T) -> Self {
        Self {
            state: std::sync::Mutex::new(initial),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> EffectHandler for StateHandler<T> {
    fn handle(&self, op: &str, args: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        match op {
            "get" => {
                let value = self.state.lock().unwrap().clone();
                Box::new(value)
            }
            "set" => {
                if let Some(value) = args.into_iter().next().and_then(|a| a.downcast::<T>().ok()) {
                    *self.state.lock().unwrap() = *value;
                }
                Box::new(())
            }
            _ => Box::new(())
        }
    }
    
    fn effect_name(&self) -> &str {
        "State"
    }
}

/// Effect runtime
pub struct EffectRuntime {
    handlers: HashMap<String, Box<dyn EffectHandler>>,
}

impl Default for EffectRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectRuntime {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a handler
    pub fn register<H: EffectHandler + 'static>(&mut self, handler: H) {
        self.handlers.insert(handler.effect_name().to_string(), Box::new(handler));
    }
    
    /// Perform an effect operation
    pub fn perform(&self, effect: &str, op: &str, args: Vec<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        self.handlers.get(effect).map(|h| h.handle(op, args))
    }
}

/// Function signature with effects
#[derive(Debug, Clone)]
pub struct EffectfulFunction {
    pub name: String,
    pub effects: Vec<String>,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}

impl EffectfulFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            effects: Vec::new(),
            params: Vec::new(),
            return_type: "Unit".to_string(),
        }
    }
    
    pub fn with_effect(mut self, effect: &str) -> Self {
        self.effects.push(effect.to_string());
        self
    }
    
    pub fn with_param(mut self, name: &str, ty: &str) -> Self {
        self.params.push((name.to_string(), ty.to_string()));
        self
    }
    
    pub fn returns(mut self, ty: &str) -> Self {
        self.return_type = ty.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_effect_definition() {
        let effect = builtin::console_effect();
        assert_eq!(effect.name, "Console");
        assert_eq!(effect.operations.len(), 2);
    }
    
    #[test]
    fn test_effectful_function() {
        let func = EffectfulFunction::new("greet")
            .with_effect("Console")
            .with_param("name", "String")
            .returns("Unit");
        
        assert_eq!(func.effects, vec!["Console"]);
    }
}
