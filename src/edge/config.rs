//! Edge deployment configuration

use super::EdgeProvider;
use std::collections::HashMap;

/// Project-level edge configuration (from ssl.toml)
#[derive(Debug, Clone)]
pub struct EdgeProjectConfig {
    /// Default provider
    pub provider: EdgeProvider,
    /// Default regions
    pub regions: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Build settings
    pub build: EdgeBuildConfig,
}

impl Default for EdgeProjectConfig {
    fn default() -> Self {
        Self {
            provider: EdgeProvider::Cloudflare,
            regions: vec!["auto".to_string()],
            env: HashMap::new(),
            build: EdgeBuildConfig::default(),
        }
    }
}

/// Build configuration
#[derive(Debug, Clone)]
pub struct EdgeBuildConfig {
    /// Output directory
    pub output_dir: String,
    /// Minify output
    pub minify: bool,
    /// Source maps
    pub source_maps: bool,
    /// Bundle external dependencies
    pub bundle: bool,
}

impl Default for EdgeBuildConfig {
    fn default() -> Self {
        Self {
            output_dir: "dist".to_string(),
            minify: true,
            source_maps: false,
            bundle: true,
        }
    }
}

/// Provider-specific configuration
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    Cloudflare(CloudflareConfig),
    Vercel(VercelConfig),
    AwsLambda(AwsLambdaConfig),
    Deno(DenoConfig),
    Fastly(FastlyConfig),
}

/// Cloudflare Workers configuration
#[derive(Debug, Clone, Default)]
pub struct CloudflareConfig {
    /// Account ID
    pub account_id: Option<String>,
    /// Zone ID (for routes)
    pub zone_id: Option<String>,
    /// Workers.dev subdomain
    pub workers_dev: bool,
    /// Custom routes
    pub routes: Vec<String>,
    /// KV namespaces
    pub kv_namespaces: Vec<String>,
    /// Durable Objects
    pub durable_objects: Vec<String>,
}

/// Vercel Edge configuration
#[derive(Debug, Clone, Default)]
pub struct VercelConfig {
    /// Project ID
    pub project_id: Option<String>,
    /// Team ID
    pub team_id: Option<String>,
    /// Framework (none for edge functions)
    pub framework: Option<String>,
}

/// AWS Lambda configuration
#[derive(Debug, Clone)]
pub struct AwsLambdaConfig {
    /// AWS Region
    pub region: String,
    /// IAM Role ARN
    pub role_arn: Option<String>,
    /// Function URL enabled
    pub function_url: bool,
    /// VPC configuration
    pub vpc: Option<VpcConfig>,
}

impl Default for AwsLambdaConfig {
    fn default() -> Self {
        Self {
            region: "us-east-1".to_string(),
            role_arn: None,
            function_url: true,
            vpc: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VpcConfig {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}

/// Deno Deploy configuration
#[derive(Debug, Clone, Default)]
pub struct DenoConfig {
    /// Project name
    pub project: Option<String>,
    /// GitHub integration
    pub github_integration: bool,
}

/// Fastly Compute configuration
#[derive(Debug, Clone, Default)]
pub struct FastlyConfig {
    /// Service ID
    pub service_id: Option<String>,
    /// Domain
    pub domain: Option<String>,
}

/// Load edge config from ssl.toml
pub fn load_edge_config(toml_content: &str) -> Result<EdgeProjectConfig, String> {
    let mut config = EdgeProjectConfig::default();
    
    // Simple parsing - in production use toml crate
    for line in toml_content.lines() {
        let line = line.trim();
        
        if line.starts_with("provider") {
            if line.contains("cloudflare") {
                config.provider = EdgeProvider::Cloudflare;
            } else if line.contains("vercel") {
                config.provider = EdgeProvider::Vercel;
            } else if line.contains("aws") || line.contains("lambda") {
                config.provider = EdgeProvider::AwsLambda;
            } else if line.contains("deno") {
                config.provider = EdgeProvider::Deno;
            } else if line.contains("fastly") {
                config.provider = EdgeProvider::Fastly;
            }
        }
        
        if line.starts_with("minify") {
            config.build.minify = line.contains("true");
        }
    }
    
    Ok(config)
}

/// Generate wrangler.toml for Cloudflare
pub fn generate_wrangler_toml(config: &CloudflareConfig, name: &str) -> String {
    format!(r#"name = "{name}"
main = "src/index.js"
compatibility_date = "2024-01-01"

[vars]
# Environment variables go here

# Account and zone IDs (optional if using workers.dev)
{account_id}
{zone_id}

[build]
command = "ssl build --target wasm"
"#,
        name = name,
        account_id = config.account_id.as_ref()
            .map(|id| format!("account_id = \"{}\"", id))
            .unwrap_or_else(|| "# account_id = \"your-account-id\"".to_string()),
        zone_id = config.zone_id.as_ref()
            .map(|id| format!("zone_id = \"{}\"", id))
            .unwrap_or_else(|| "# zone_id = \"your-zone-id\"".to_string()),
    )
}

/// Generate vercel.json
pub fn generate_vercel_json(config: &VercelConfig, name: &str) -> String {
    format!(r#"{{
  "name": "{}",
  "version": 2,
  "builds": [
    {{
      "src": "api/**/*.ssl",
      "use": "@vercel/ssl"
    }}
  ],
  "routes": [
    {{
      "src": "/api/(.*)",
      "dest": "/api/$1"
    }}
  ]
}}"#, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_edge_config() {
        let toml = r#"
[edge]
provider = "cloudflare"
minify = true
"#;
        
        let config = load_edge_config(toml).unwrap();
        assert_eq!(config.provider, EdgeProvider::Cloudflare);
        assert!(config.build.minify);
    }
    
    #[test]
    fn test_generate_wrangler() {
        let config = CloudflareConfig::default();
        let wrangler = generate_wrangler_toml(&config, "my-worker");
        assert!(wrangler.contains("my-worker"));
    }
}
