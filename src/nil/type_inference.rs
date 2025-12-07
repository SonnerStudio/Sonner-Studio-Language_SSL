// Type Inference Engine for NIL

use crate::nil::intent::*;
use crate::ast::Type;

/// Simple type inference engine
pub struct TypeInferenceEngine;

impl TypeInferenceEngine {
    pub fn new() -> Self {
        TypeInferenceEngine
    }
    
    /// Infers return type from function body
    pub fn infer_return(&self, intent: &Intent) -> Option<Type> {
        match intent {
            Intent::Literal(lit) => Some(self.literal_type(lit)),
            Intent::BinaryOp { left, right, .. } => {
                // Binary ops typically return same type as operands
                self.infer_return(left).or_else(|| self.infer_return(right))
            }
            Intent::Sequence(intents) => {
                // Return type of last intent
                intents.last().and_then(|i| self.infer_return(i))
            }
            Intent::Conditional { then_branch, .. } => {
                self.infer_return(then_branch)
            }
            _ => Some(Type::Custom("Any".to_string())),
        }
    }
    
    /// Gets type from literal value
    fn literal_type(&self, lit: &LiteralValue) -> Type {
        match lit {
            LiteralValue::Integer(_) => Type::Int,
            LiteralValue::Float(_) => Type::Float,
            LiteralValue::String(_) => Type::String,
            LiteralValue::Boolean(_) => Type::Bool,
            LiteralValue::List(_) => Type::List(Box::new(Type::Custom("Any".to_string()))),
        }
    }
    
    /// Infers parameter type from usage in intent
    pub fn infer_parameter_type(&self, _param_name: &str, _intent: &Intent) -> Type {
        // Simple default - could be enhanced with usage analysis
        Type::Int
    }
}

impl Default for TypeInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_infer_literal_types() {
        let engine = TypeInferenceEngine::new();
        
        let int_intent = Intent::literal_int(42);
        assert_eq!(engine.infer_return(&int_intent), Some(Type::Int));
        
        let str_intent = Intent::literal_string("hello".to_string());
        assert_eq!(engine.infer_return(&str_intent), Some(Type::String));
        
        let bool_intent = Intent::literal_bool(true);
        assert_eq!(engine.infer_return(&bool_intent), Some(Type::Bool));
    }
    
    #[test]
    fn test_infer_binary_op() {
        let engine = TypeInferenceEngine::new();
        
        let add = Intent::binary(
            Intent::literal_int(1),
            "+".to_string(),
            Intent::literal_int(2),
        );
        
        assert_eq!(engine.infer_return(&add), Some(Type::Int));
    }
}
