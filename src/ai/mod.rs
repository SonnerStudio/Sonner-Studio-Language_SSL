// AI-First Programming Module
// Integrates LLMs for code review, test generation, and refactoring

pub mod provider;
pub mod reviewer;
pub mod testgen;
pub mod refactor;

pub use provider::{LLMProvider, LLMConfig, OpenAIProvider};
pub use reviewer::{CodeReviewer, CodeReview};
pub use testgen::TestGenerator;
pub use refactor::RefactoringAgent;

/// AI features configuration
#[derive(Debug, Clone)]
pub struct AIConfig {
    /// LLM provider (OpenAI, Anthropic, etc.)
    pub provider: String,
    
    /// API key
    pub api_key: Option<String>,
    
    /// Model name
    pub model: String,
    
    /// Max tokens per request
    pub max_tokens: usize,
    
    /// Temperature (0.0-1.0)
    pub temperature: f32,
}

impl Default for AIConfig {
    fn default() -> Self {
        AIConfig {
            provider: "openai".to_string(),
            api_key: None,
            model: "gpt-4".to_string(),
            max_tokens: 2000,
            temperature: 0.7,
        }
    }
}
