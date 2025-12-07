//! Edge provider implementations

pub mod cloudflare;
pub mod vercel;

use super::{EdgeProvider, EdgeFunction, DeploymentResult, DeploymentStatus};
use std::path::Path;

/// Provider trait for deployment
pub trait Provider {
    /// Get provider type
    fn provider_type(&self) -> EdgeProvider;
    
    /// Deploy function
    fn deploy(&self, function: &EdgeFunction, output_dir: &Path) -> Result<DeploymentResult, String>;
    
    /// Get deployment status
    fn status(&self, deployment_id: &str) -> Result<DeploymentStatus, String>;
    
    /// Delete deployment
    fn delete(&self, deployment_id: &str) -> Result<(), String>;
    
    /// List deployments
    fn list(&self) -> Result<Vec<String>, String>;
}

/// Provider factory
pub fn create_provider(provider: EdgeProvider, api_token: Option<&str>) -> Box<dyn Provider> {
    match provider {
        EdgeProvider::Cloudflare => Box::new(cloudflare::CloudflareProvider::new(api_token)),
        EdgeProvider::Vercel => Box::new(vercel::VercelProvider::new(api_token)),
        _ => Box::new(GenericProvider::new(provider)),
    }
}

/// Generic provider for unsupported platforms
struct GenericProvider {
    provider: EdgeProvider,
}

impl GenericProvider {
    fn new(provider: EdgeProvider) -> Self {
        Self { provider }
    }
}

impl Provider for GenericProvider {
    fn provider_type(&self) -> EdgeProvider {
        self.provider
    }
    
    fn deploy(&self, function: &EdgeFunction, _output_dir: &Path) -> Result<DeploymentResult, String> {
        Err(format!("{} provider not yet implemented", self.provider.name()))
    }
    
    fn status(&self, _deployment_id: &str) -> Result<DeploymentStatus, String> {
        Err("Not implemented".to_string())
    }
    
    fn delete(&self, _deployment_id: &str) -> Result<(), String> {
        Err("Not implemented".to_string())
    }
    
    fn list(&self) -> Result<Vec<String>, String> {
        Err("Not implemented".to_string())
    }
}
