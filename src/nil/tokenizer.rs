// Natural Language Tokenizer for SSL NIL

use serde::{Serialize, Deserialize};

/// Token from natural language input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NLToken {
    pub text: String,
    pub kind: TokenKind,
    pub confidence: f32,
    pub position: usize,
}

/// Token type classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    /// Keywords: "create", "function", "if", "loop", "return"
    Keyword,
    
    /// Identifiers: variable names, function names
    Identifier,
    
    /// Operators: "add", "multiply", "greater than", "equals"
    Operator,
    
    /// Data types: "number", "string", "list", "boolean"
    DataType,
    
    /// Literal values: numbers, strings in quotes
    Literal,
    
    /// Connectors: "then", "with", "for each", "where"
    Connector,
    
    /// Prepositions: "from", "to", "in", "on"
    Preposition,
    
    /// Unknown/unclassified
    Unknown,
}

/// Natural Language Tokenizer
pub struct Tokenizer {
    keywords: Vec<String>,
    operators: Vec<String>,
    data_types: Vec<String>,
    connectors: Vec<String>,
    prepositions: Vec<String>,
}

impl Tokenizer {
    /// Creates a new tokenizer with default patterns
    pub fn new() -> Self {
        Tokenizer {
            keywords: vec![
                "create", "make", "define", "function", "fn",
                "if", "then", "else", "otherwise",
                "loop", "for", "each", "while",
                "return", "give", "output",
                "let", "set", "assign",
                "read", "write", "load", "save",
            ].iter().map(|s| s.to_string()).collect(),
            
            operators: vec![
                "add", "plus", "+",
                "subtract", "minus", "-",
                "multiply", "times", "*",
                "divide", "divided by", "/",
                "equals", "is", "==",
                "greater than", "more than", ">",
                "less than", "<",
                "and", "&&",
                "or", "||",
                "not", "!",
            ].iter().map(|s| s.to_string()).collect(),
            
            data_types: vec![
                "number", "int", "integer",
                "float", "decimal",
                "string", "text",
                "boolean", "bool",
                "list", "array",
            ].iter().map(|s| s.to_string()).collect(),
            
            connectors: vec![
                "then", "with", "using",
                "for each", "where",
                "that", "which",
            ].iter().map(|s| s.to_string()).collect(),
            
            prepositions: vec![
                "from", "to", "in", "on", "at",
                "by", "of", "as",
            ].iter().map(|s| s.to_string()).collect(),
        }
    }
    
    /// Tokenizes natural language input
    pub fn tokenize(&self, input: &str) -> Vec<NLToken> {
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut tokens = Vec::new();
        let mut pos = 0;
        
        let mut i = 0;
        while i < words.len() {
            let word = words[i].to_lowercase();
            
            // Try multi-word patterns first
            if i + 1 < words.len() {
                let two_word = format!("{} {}", word, words[i + 1].to_lowercase());
                
                if self.operators.contains(&two_word) {
                    tokens.push(NLToken {
                        text: two_word.clone(),
                        kind: TokenKind::Operator,
                        confidence: 0.95,
                        position: pos,
                    });
                    i += 2;
                    pos += words[i - 2].len() + words[i - 1].len() + 1;
                    continue;
                } else if self.connectors.contains(&two_word) {
                    tokens.push(NLToken {
                        text: two_word.clone(),
                        kind: TokenKind::Connector,
                        confidence: 0.90,
                        position: pos,
                    });
                    i += 2;
                    pos += words[i - 2].len() + words[i - 1].len() + 1;
                    continue;
                }
            }
            
            // Single word classification
            let (kind, confidence) = self.classify_word(&word);
            tokens.push(NLToken {
                text: word.clone(),
                kind,
                confidence,
                position: pos,
            });
            
            pos += words[i].len() + 1;
            i += 1;
        }
        
        tokens
    }
    
    /// Classifies a single word
    fn classify_word(&self, word: &str) -> (TokenKind, f32) {
        // Check if it's a number
        if word.parse::<i64>().is_ok() || word.parse::<f64>().is_ok() {
            return (TokenKind::Literal, 1.0);
        }
        
        // Check if it's a string literal (quotes)
        if (word.starts_with('"') && word.ends_with('"')) ||
           (word.starts_with('\'') && word.ends_with('\'')) {
            return (TokenKind::Literal, 1.0);
        }
        
        // Check keyword
        if self.keywords.contains(&word.to_string()) {
            return (TokenKind::Keyword, 0.95);
        }
        
        // Check operator
        if self.operators.contains(&word.to_string()) {
            return (TokenKind::Operator, 0.90);
        }
        
        // Check data type
        if self.data_types.contains(&word.to_string()) {
            return (TokenKind::DataType, 0.85);
        }
        
        // Check connector
        if self.connectors.contains(&word.to_string()) {
            return (TokenKind::Connector, 0.85);
        }
        
        // Check preposition
        if self.prepositions.contains(&word.to_string()) {
            return (TokenKind::Preposition, 0.80);
        }
        
        // Likely an identifier if it starts with letter/underscore
        if word.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') {
            return (TokenKind::Identifier, 0.70);
        }
        
        (TokenKind::Unknown, 0.3)
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("create a function");
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "create");
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[2].text, "function");
        assert_eq!(tokens[2].kind, TokenKind::Keyword);
    }
    
    #[test]
    fn test_tokenize_with_operator() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("add two numbers");
        
        assert!(tokens.iter().any(|t| t.text == "add" && t.kind == TokenKind::Operator));
    }
    
    #[test]
    fn test_tokenize_multi_word_operator() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("if x greater than 5");
        
        assert!(tokens.iter().any(|t| t.text == "greater than" && t.kind == TokenKind::Operator));
    }
    
    #[test]
    fn test_tokenize_with_literal() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("return 42");
        
        let literal_token = tokens.iter().find(|t| t.text == "42");
        assert!(literal_token.is_some());
        assert_eq!(literal_token.unwrap().kind, TokenKind::Literal);
    }
    
    #[test]
    fn test_classify_identifier() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("let myVariable be 10");
        
        let var_token = tokens.iter().find(|t| t.text == "myvariable");
        assert!(var_token.is_some());
        assert_eq!(var_token.unwrap().kind, TokenKind::Identifier);
    }
}
