// SSL Intent Tree (SSIT) - Intermediate representation between natural language and SSL AST

use crate::ast::{Statement, Expression};
use serde::{Serialize, Deserialize};

/// Core Intent types representing user intentions in natural language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    /// Function creation: "create a function that..."
    CreateFunction {
        name: Option<String>,
        parameters: Vec<IntentParameter>,
        intent: Box<Intent>,
        return_hint: Option<String>,
    },
    
    /// Data processing: "read data from...", "filter where..."
    DataProcessing {
        operation: DataOp,
        source: Option<String>,
        target: Option<String>,
        filters: Vec<Filter>,
    },
    
    /// Conditional logic: "if..then..else"
    Conditional {
        condition: Box<Intent>,
        then_branch: Box<Intent>,
        else_branch: Option<Box<Intent>>,
    },
    
    /// Iteration: "for each...", "loop over..."
    Loop {
        iterator: Option<String>,
        collection: Box<Intent>,
        body: Box<Intent>,
    },
    
    /// Variable assignment: "let x be..."
    Assignment {
        variable: String,
        value: Box<Intent>,
    },
    
    /// Literal values
    Literal(LiteralValue),
    
    /// Variable reference
    Variable(String),
    
    /// Binary operations: "add", "multiply", "greater than"
    BinaryOp {
        left: Box<Intent>,
        op: String,
        right: Box<Intent>,
    },
    
    /// Function call: "call function with..."
    FunctionCall {
        name: String,
        arguments: Vec<Intent>,
    },
    
    /// Sequence of intents
    Sequence(Vec<Intent>),
    
    /// Ambiguous intent requiring clarification
    Ambiguous {
        text: String,
        candidates: Vec<Intent>,
        confidence: f32,
    },
}

/// Data operations for processing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataOp {
    Read,
    Write,
    Transform,
    Filter,
    Aggregate,
    Sort,
}

/// Filter specification for data operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: String,  // ">", "<", "==", "!=", "contains", etc.
    pub value: String,
}

/// Parameter for function intents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentParameter {
    pub name: String,
    pub type_hint: Option<String>,
}

/// Literal values in intents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<LiteralValue>),
}

impl Intent {
    /// Creates a simple literal intent
    pub fn literal_int(value: i64) -> Self {
        Intent::Literal(LiteralValue::Integer(value))
    }
    
    pub fn literal_string(value: String) -> Self {
        Intent::Literal(LiteralValue::String(value))
    }
    
    pub fn literal_bool(value: bool) -> Self {
        Intent::Literal(LiteralValue::Boolean(value))
    }
    
    /// Creates a variable reference
    pub fn var(name: String) -> Self {
        Intent::Variable(name)
    }
    
    /// Creates a binary operation intent
    pub fn binary(left: Intent, op: String, right: Intent) -> Self {
        Intent::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
    
    /// Checks if intent is ambiguous
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, Intent::Ambiguous { .. })
    }
    
    /// Gets confidence score for ambiguous intents
    pub fn confidence(&self) -> f32 {
        match self {
            Intent::Ambiguous { confidence, .. } => *confidence,
            _ => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_intent_creation() {
        let intent = Intent::literal_int(42);
        assert_eq!(intent, Intent::Literal(LiteralValue::Integer(42)));
    }
    
    #[test]
    fn test_binary_op() {
        let left = Intent::var("x".to_string());
        let right = Intent::literal_int(2);
        let op = Intent::binary(left, "*".to_string(), right);
        
        if let Intent::BinaryOp { op: operator, .. } = op {
            assert_eq!(operator, "*");
        } else {
            panic!("Expected BinaryOp");
        }
    }
    
    #[test]
    fn test_ambiguous_intent() {
        let intent = Intent::Ambiguous {
            text: "do something".to_string(),
            candidates: vec![],
            confidence: 0.5,
        };
        
        assert!(intent.is_ambiguous());
        assert_eq!(intent.confidence(), 0.5);
    }
}
