// Pattern definitions for Natural Language → Intent conversion

use crate::nil::intent::*;
use crate::nil::tokenizer::{NLToken, TokenKind};

/// Pattern for matching natural language to intents
#[derive(Debug, Clone)]
pub struct Pattern {
    /// Keywords that trigger this pattern
    pub triggers: Vec<String>,
    
    /// Required token kinds in sequence (flexible matching)
    pub required_kinds: Vec<TokenKind>,
    
    /// Intent type this pattern produces
    pub intent_type: IntentType,
    
    /// Minimum confidence threshold
    pub confidence_threshold: f32,
    
    /// Pattern priority (higher = checked first)
    pub priority: i32,
}

/// Intent type classifier
#[derive(Debug, Clone, PartialEq)]
pub enum IntentType {
    Function,
    Loop,
    Conditional,
    Assignment,
    DataProcessing,
    Return,
}

impl Pattern {
    /// Creates a new pattern
    pub fn new(triggers: Vec<&str>, intent_type: IntentType, priority: i32) -> Self {
        Pattern {
            triggers: triggers.iter().map(|s| s.to_string()).collect(),
            required_kinds: vec![],
            intent_type,
            confidence_threshold: 0.7,
            priority,
        }
    }
    
    /// Adds required token kinds
    pub fn with_kinds(mut self, kinds: Vec<TokenKind>) -> Self {
        self.required_kinds = kinds;
        self
    }
    
    /// Sets confidence threshold
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }
    
    /// Checks if tokens match this pattern
    pub fn matches(&self, tokens: &[NLToken]) -> (bool, f32) {
        // Check if any trigger words are present
        let has_trigger = tokens.iter().any(|t| {
            self.triggers.iter().any(|trigger| {
                t.text.to_lowercase() == trigger.to_lowercase()
            })
        });
        
        if !has_trigger {
            return (false, 0.0);
        }
        
        // Calculate confidence based on token kinds present
        let mut confidence = 0.7; // Base confidence for trigger match
        
        if !self.required_kinds.is_empty() {
            let kinds_present: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();
            let matching_kinds = self.required_kinds.iter()
                .filter(|k| kinds_present.contains(k))
                .count();
            
            let kind_ratio = matching_kinds as f32 / self.required_kinds.len() as f32;
            confidence = (confidence + kind_ratio) / 2.0;
        }
        
        // Boost confidence with average token confidence
        let avg_token_conf: f32 = tokens.iter().map(|t| t.confidence).sum::<f32>() / tokens.len() as f32;
        confidence = (confidence + avg_token_conf) / 2.0;
        
        (confidence >= self.confidence_threshold, confidence)
    }
}

/// Default pattern library
pub fn default_patterns() -> Vec<Pattern> {
    vec![
        // Function creation patterns
        Pattern::new(
            vec!["create", "function", "fn"],
            IntentType::Function,
            10,
        ).with_kinds(vec![TokenKind::Keyword, TokenKind::Identifier])
         .with_threshold(0.75),
        
        Pattern::new(
            vec!["define", "function"],
            IntentType::Function,
            9,
        ).with_threshold(0.75),
        
        // Loop patterns
        Pattern::new(
            vec!["loop", "for", "each"],
            IntentType::Loop,
            8,
        ).with_kinds(vec![TokenKind::Keyword, TokenKind::Connector])
         .with_threshold(0.70),
        
        Pattern::new(
            vec!["iterate", "over"],
            IntentType::Loop,
            7,
        ).with_threshold(0.70),
        
        // Conditional patterns
        Pattern::new(
            vec!["if", "when"],
            IntentType::Conditional,
            8,
        ).with_kinds(vec![TokenKind::Keyword, TokenKind::Operator])
         .with_threshold(0.75),
        
        // Assignment patterns
        Pattern::new(
            vec!["let", "set", "assign"],
            IntentType::Assignment,
            6,
        ).with_kinds(vec![TokenKind::Keyword, TokenKind::Identifier])
         .with_threshold(0.70),
        
        // Data processing patterns
        Pattern::new(
            vec!["read", "load", "from"],
            IntentType::DataProcessing,
            7,
        ).with_kinds(vec![TokenKind::Keyword, TokenKind::Preposition])
         .with_threshold(0.70),
        
        Pattern::new(
            vec!["filter", "where"],
            IntentType::DataProcessing,
            7,
        ).with_threshold(0.70),
        
        // Return patterns
        Pattern::new(
            vec!["return", "give", "output"],
            IntentType::Return,
            5,
        ).with_threshold(0.75),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nil::tokenizer::Tokenizer;
    
    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern::new(vec!["create", "function"], IntentType::Function, 10);
        assert_eq!(pattern.intent_type, IntentType::Function);
        assert_eq!(pattern.triggers.len(), 2);
    }
    
    #[test]
    fn test_pattern_matching() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("create a function");
        
        let pattern = Pattern::new(vec!["create"], IntentType::Function, 10);
        let (matches, confidence) = pattern.matches(&tokens);
        
        assert!(matches);
        assert!(confidence > 0.7);
    }
    
    #[test]
    fn test_pattern_with_kinds() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("create function add");
        
        let pattern = Pattern::new(vec!["create"], IntentType::Function, 10)
            .with_kinds(vec![TokenKind::Keyword, TokenKind::Identifier]);
        
        let (matches, _) = pattern.matches(&tokens);
        assert!(matches);
    }
    
    #[test]
    fn test_no_match() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("something completely different");
        
        let pattern = Pattern::new(vec!["create"], IntentType::Function, 10);
        let (matches, _) = pattern.matches(&tokens);
        
        assert!(!matches);
    }
}
