//! SSL 4.0 Edge/Serverless Deployment
//!
//! Deploy SSL functions directly to cloud edge networks.

pub mod deploy;
pub mod config;
pub mod providers;

use std::collections::HashMap;

/// Edge function configuration
#[derive(Debug, Clone)]
pub struct EdgeFunction {
    /// Function name
    pub name: String,
    /// Handler function path
    pub handler: String,
    /// Runtime configuration
    pub config: EdgeConfig,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Routes/triggers
    pub routes: Vec<EdgeRoute>,
}

/// Edge configuration
#[derive(Debug, Clone)]
pub struct EdgeConfig {
    /// Memory in MB
    pub memory: u32,
    /// Timeout in seconds
    pub timeout: u32,
    /// Deployment regions
    pub regions: Vec<String>,
    /// Minimum instances (0 = scale to zero)
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// CPU allocation (millicores)
    pub cpu: u32,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            memory: 128,
            timeout: 10,
            regions: vec!["auto".to_string()],
            min_instances: 0,
            max_instances: 100,
            cpu: 1000,
        }
    }
}

/// Edge route configuration
#[derive(Debug, Clone)]
pub struct EdgeRoute {
    /// Route path pattern
    pub path: String,
    /// HTTP methods
    pub methods: Vec<HttpMethod>,
    /// Authentication required
    pub auth: bool,
}

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

/// Supported edge providers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeProvider {
    /// Cloudflare Workers
    Cloudflare,
    /// Vercel Edge Functions
    Vercel,
    /// AWS Lambda@Edge
    AwsLambda,
    /// Deno Deploy
    Deno,
    /// Fastly Compute
    Fastly,
}

impl EdgeProvider {
    pub fn name(&self) -> &'static str {
        match self {
            EdgeProvider::Cloudflare => "Cloudflare Workers",
            EdgeProvider::Vercel => "Vercel Edge Functions",
            EdgeProvider::AwsLambda => "AWS Lambda@Edge",
            EdgeProvider::Deno => "Deno Deploy",
            EdgeProvider::Fastly => "Fastly Compute",
        }
    }
    
    /// Get default regions for provider
    pub fn default_regions(&self) -> Vec<&'static str> {
        match self {
            EdgeProvider::Cloudflare => vec!["global"],
            EdgeProvider::Vercel => vec!["iad1", "sfo1", "cdg1"],
            EdgeProvider::AwsLambda => vec!["us-east-1", "eu-west-1"],
            EdgeProvider::Deno => vec!["gcp-us-east1", "gcp-europe-west1"],
            EdgeProvider::Fastly => vec!["global"],
        }
    }
}

/// Edge deployment result
#[derive(Debug)]
pub struct DeploymentResult {
    /// Deployment ID
    pub id: String,
    /// Deployment URL
    pub url: String,
    /// Deployed regions
    pub regions: Vec<String>,
    /// Deployment status
    pub status: DeploymentStatus,
    /// Deployment logs
    pub logs: Vec<String>,
}

/// Deployment status
#[derive(Debug, Clone, PartialEq)]
pub enum DeploymentStatus {
    /// Deployment in progress
    Deploying,
    /// Deployment successful
    Ready,
    /// Deployment failed
    Failed(String),
}

/// HTTP Request for edge functions
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: HttpMethod,
    /// Request URL
    pub url: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Option<String>,
    /// Query parameters
    pub query: HashMap<String, String>,
    /// Path parameters
    pub params: HashMap<String, String>,
}

impl Request {
    /// Create a new GET request
    pub fn get(url: &str) -> Self {
        Self {
            method: HttpMethod::GET,
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            query: HashMap::new(),
            params: HashMap::new(),
        }
    }
    
    /// Create a new POST request
    pub fn post(url: &str, body: &str) -> Self {
        Self {
            method: HttpMethod::POST,
            url: url.to_string(),
            headers: HashMap::new(),
            body: Some(body.to_string()),
            query: HashMap::new(),
            params: HashMap::new(),
        }
    }
    
    /// Add a header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

/// HTTP Response from edge functions
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code
    pub status: u16,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: String,
}

impl Response {
    /// Create OK response
    pub fn ok(body: &str) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.to_string(),
        }
    }
    
    /// Create JSON response
    pub fn json(body: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body: body.to_string(),
        }
    }
    
    /// Create error response
    pub fn error(status: u16, message: &str) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: message.to_string(),
        }
    }
    
    /// Create redirect
    pub fn redirect(location: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Location".to_string(), location.to_string());
        Self {
            status: 302,
            headers,
            body: String::new(),
        }
    }
    
    /// Add header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Set status code
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
}

/// Edge function attribute parser
pub fn parse_edge_attribute(attr: &str) -> Option<EdgeConfig> {
    // Parse @edge(...) attributes
    // Example: @edge(memory: 256, timeout: 30, regions: ["us-east-1", "eu-west-1"])
    
    let mut config = EdgeConfig::default();
    
    // Simple parsing - in production would use proper parser
    if attr.contains("memory:") {
        if let Some(mem) = extract_number(attr, "memory:") {
            config.memory = mem;
        }
    }
    
    if attr.contains("timeout:") {
        if let Some(timeout) = extract_number(attr, "timeout:") {
            config.timeout = timeout;
        }
    }
    
    Some(config)
}

fn extract_number(s: &str, key: &str) -> Option<u32> {
    s.find(key)
        .and_then(|i| {
            let rest = &s[i + key.len()..];
            let num: String = rest.chars()
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| c.is_ascii_digit())
                .collect();
            num.parse().ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edge_config_default() {
        let config = EdgeConfig::default();
        assert_eq!(config.memory, 128);
        assert_eq!(config.timeout, 10);
    }
    
    #[test]
    fn test_response_json() {
        let resp = Response::json(r#"{"status":"ok"}"#);
        assert_eq!(resp.status, 200);
        assert_eq!(resp.headers.get("Content-Type").unwrap(), "application/json");
    }
    
    #[test]
    fn test_request_builder() {
        let req = Request::get("/api/users")
            .with_header("Authorization", "Bearer token");
        
        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.headers.get("Authorization").unwrap(), "Bearer token");
    }
    
    #[test]
    fn test_parse_edge_attribute() {
        let config = parse_edge_attribute("memory: 256, timeout: 30").unwrap();
        assert_eq!(config.memory, 256);
        assert_eq!(config.timeout, 30);
    }
}
