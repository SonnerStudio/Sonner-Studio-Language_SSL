// Pattern Matching Engine - Converts NL Tokens → SSIT Intents

use crate::nil::intent::*;
use crate::nil::tokenizer::{NLToken, TokenKind};
use crate::nil::patterns::{Pattern, IntentType, default_patterns};

/// Pattern matching engine
pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

impl PatternMatcher {
    /// Creates a new matcher with default patterns
    pub fn new() -> Self {
        let mut patterns = default_patterns();
        // Sort by priority (descending)
        patterns.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        PatternMatcher { patterns }
    }
    
    /// Matches tokens to an intent
    pub fn match_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        if tokens.is_empty() {
            return Err("No tokens to match".to_string());
        }
        
        // Try each pattern in priority order
        for pattern in &self.patterns {
            let (matches, confidence) = pattern.matches(tokens);
            
            if matches {
                let intent = self.build_intent(&pattern.intent_type, tokens, confidence)?;
                return Ok(intent);
            }
        }
        
        // No pattern matched - return ambiguous intent
        Ok(Intent::Ambiguous {
            text: tokens.iter().map(|t| t.text.clone()).collect::<Vec<_>>().join(" "),
            candidates: vec![],
            confidence: 0.3,
        })
    }
    
    /// Builds an intent from matched pattern
    fn build_intent(&self, intent_type: &IntentType, tokens: &[NLToken], confidence: f32) -> Result<Intent, String> {
        match intent_type {
            IntentType::Function => self.build_function_intent(tokens),
            IntentType::Loop => self.build_loop_intent(tokens),
            IntentType::Conditional => self.build_conditional_intent(tokens),
            IntentType::Assignment => self.build_assignment_intent(tokens),
            IntentType::DataProcessing => self.build_data_processing_intent(tokens),
            IntentType::Return => self.build_return_intent(tokens),
        }
    }
    
    /// Builds a function creation intent
    fn build_function_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        // Extract function name if present
        let name = tokens.iter()
            .find(|t| t.kind == TokenKind::Identifier)
            .map(|t| t.text.clone());
        
        // Extract operators to determine what the function does
        let operators: Vec<&NLToken> = tokens.iter()
            .filter(|t| t.kind == TokenKind::Operator)
            .collect();
        
        // Build a simple intent based on operators found
        let intent = if let Some(op) = operators.first() {
            // Create binary operation intent
            Intent::BinaryOp {
                left: Box::new(Intent::var("x".to_string())),
                op: self.normalize_operator(&op.text),
                right: Box::new(Intent::literal_int(1)), // Placeholder
            }
        } else {
            Intent::Variable("x".to_string()) // Placeholder
        };
        
        Ok(Intent::CreateFunction {
            name,
            parameters: vec![IntentParameter {
                name: "x".to_string(),
                type_hint: Some("Int".to_string()),
            }],
            intent: Box::new(intent),
            return_hint: Some("Int".to_string()),
        })
    }
    
    /// Builds a loop intent
    fn build_loop_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        // Extract iterator variable if present
        let iterator = tokens.iter()
            .find(|t| t.kind == TokenKind::Identifier)
            .map(|t| t.text.clone());
        
        Ok(Intent::Loop {
            iterator,
            collection: Box::new(Intent::Variable("items".to_string())),
            body: Box::new(Intent::Sequence(vec![])),
        })
    }
    
    /// Builds a conditional intent
    fn build_conditional_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        // Find operators for condition
        let operators: Vec<&NLToken> = tokens.iter()
            .filter(|t| t.kind == TokenKind::Operator)
            .collect();
        
        let condition = if let Some(op) = operators.first() {
            Intent::BinaryOp {
                left: Box::new(Intent::var("x".to_string())),
                op: self.normalize_operator(&op.text),
                right: Box::new(Intent::literal_int(0)),
            }
        } else {
            Intent::var("condition".to_string())
        };
        
        Ok(Intent::Conditional {
            condition: Box::new(condition),
            then_branch: Box::new(Intent::Sequence(vec![])),
            else_branch: None,
        })
    }
    
    /// Builds an assignment intent
    fn build_assignment_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        let variable = tokens.iter()
            .find(|t| t.kind == TokenKind::Identifier)
            .map(|t| t.text.clone())
            .unwrap_or_else(|| "x".to_string());
        
        // Find literal value if present
        let value = tokens.iter()
            .find(|t| t.kind == TokenKind::Literal)
            .and_then(|t| t.text.parse::<i64>().ok())
            .map(Intent::literal_int)
            .unwrap_or_else(|| Intent::literal_int(0));
        
        Ok(Intent::Assignment {
            variable,
            value: Box::new(value),
        })
    }
    
    /// Builds a data processing intent
    fn build_data_processing_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        // Determine operation type from keywords
        let operation = if tokens.iter().any(|t| t.text == "read" || t.text == "load") {
            DataOp::Read
        } else if tokens.iter().any(|t| t.text == "write" || t.text == "save") {
            DataOp::Write
        } else if tokens.iter().any(|t| t.text == "filter" || t.text == "where") {
            DataOp::Filter
        } else {
            DataOp::Read
        };
        
        // Extract source from identifiers
        let source = tokens.iter()
            .find(|t| t.kind == TokenKind::Identifier)
            .map(|t| t.text.clone());
        
        Ok(Intent::DataProcessing {
            operation,
            source,
            target: None,
            filters: vec![],
        })
    }
    
    /// Builds a return intent
    fn build_return_intent(&self, tokens: &[NLToken]) -> Result<Intent, String> {
        // Find what to return
        let value = tokens.iter()
            .find(|t| t.kind == TokenKind::Literal || t.kind == TokenKind::Identifier)
            .map(|t| {
                if let Ok(num) = t.text.parse::<i64>() {
                    Intent::literal_int(num)
                } else {
                    Intent::var(t.text.clone())
                }
            })
            .unwrap_or_else(|| Intent::literal_int(0));
        
        // Wrap in sequence for now
        Ok(Intent::Sequence(vec![value]))
    }
    
    /// Normalizes operator text to symbol
    fn normalize_operator(&self, op_text: &str) -> String {
        match op_text.to_lowercase().as_str() {
            "add" | "plus" => "+".to_string(),
            "subtract" | "minus" => "-".to_string(),
            "multiply" | "times" => "*".to_string(),
            "divide" | "divided by" => "/".to_string(),
            "equals" | "is" => "==".to_string(),
            "greater than" | "more than" => ">".to_string(),
            "less than" => "<".to_string(),
            _ => op_text.to_string(),
        }
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nil::tokenizer::Tokenizer;
    
    #[test]
    fn test_match_function_intent() {
        let tokenizer = Tokenizer::new();
        let matcher = PatternMatcher::new();
        
        let tokens = tokenizer.tokenize("create a function that adds numbers");
        let intent = matcher.match_intent(&tokens).unwrap();
        
        if let Intent::CreateFunction { .. } = intent {
            // Success
        } else {
            panic!("Expected CreateFunction intent");
        }
    }
    
    #[test]
    fn test_match_loop_intent() {
        let tokenizer = Tokenizer::new();
        let matcher = PatternMatcher::new();
        
        let tokens = tokenizer.tokenize("loop over items");
        let intent = matcher.match_intent(&tokens).unwrap();
        
        if let Intent::Loop { .. } = intent {
            // Success
        } else {
            panic!("Expected Loop intent, got: {:?}", intent);
        }
    }
    
    #[test]
    fn test_match_assignment() {
        let tokenizer = Tokenizer::new();
        let matcher = PatternMatcher::new();
        
        let tokens = tokenizer.tokenize("let x be 42");
        let intent = matcher.match_intent(&tokens).unwrap();
        
        if let Intent::Assignment { variable, .. } = intent {
            assert_eq!(variable, "x");
        } else {
            panic!("Expected Assignment intent");
        }
    }
    
    #[test]
    fn test_ambiguous_match() {
        let tokenizer = Tokenizer::new();
        let matcher = PatternMatcher::new();
        
        let tokens = tokenizer.tokenize("do something weird");
        let intent = matcher.match_intent(&tokens).unwrap();
        
        assert!(intent.is_ambiguous());
    }
    
    #[test]
    fn test_normalize_operator() {
        let matcher = PatternMatcher::new();
        
        assert_eq!(matcher.normalize_operator("add"), "+");
        assert_eq!(matcher.normalize_operator("multiply"), "*");
        assert_eq!(matcher.normalize_operator("greater than"), ">");
    }
}
