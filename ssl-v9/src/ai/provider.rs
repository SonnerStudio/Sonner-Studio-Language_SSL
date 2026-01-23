use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;

/// LLM provider abstraction
pub trait LLMProvider {
    fn complete(&self, prompt: &str) -> Result<String, String>;
}

/// Configuration for LLM provider
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

/// OpenAI provider implementation
pub struct OpenAIProvider {
    config: LLMConfig,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(config: LLMConfig) -> Result<Self, String> {
        Ok(OpenAIProvider {
            config,
            client: Client::new(),
        })
    }
}

impl LLMProvider for OpenAIProvider {
    fn complete(&self, prompt: &str) -> Result<String, String> {
        #[derive(Serialize)]
        struct Request {
            model: String,
            messages: Vec<Message>,
            max_tokens: usize,
            temperature: f32,
        }
        
        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }
        
        #[derive(Deserialize)]
        struct Response {
            choices: Vec<Choice>,
        }
        
        #[derive(Deserialize)]
        struct Choice {
            message: ResponseMessage,
        }
        
        #[derive(Deserialize)]
        struct ResponseMessage {
            content: String,
        }
        
        let request = Request {
            model: self.config.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("Request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }
        
        let data: Response = response.json()
            .map_err(|e| format!("Parse error: {}", e))?;
        
        data.choices.first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No response from API".to_string())
    }
}

/// Anthropic (Claude) provider implementation
pub struct AnthropicProvider {
    config: LLMConfig,
    client: Client,
}

impl AnthropicProvider {
    pub fn new(config: LLMConfig) -> Result<Self, String> {
        Ok(AnthropicProvider {
            config,
            client: Client::new(),
        })
    }
}

impl LLMProvider for AnthropicProvider {
    fn complete(&self, prompt: &str) -> Result<String, String> {
        // Similar to OpenAI but for Anthropic API
        // Placeholder for now
        Err("Anthropic provider not yet implemented".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_creation() {
        let config = LLMConfig {
            api_key: "test_key".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
        };
        
        assert_eq!(config.model, "gpt-4");
    }
}
