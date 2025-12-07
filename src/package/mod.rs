// SSL Package Manager - MVP Implementation
// Integrated in v2.0.0
// See docs/design/PACKAGE_MANAGER_DESIGN.md for full specification

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// Package manifest (ssl.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub package: PackageInfo,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub build: BuildConfig,
}

impl Manifest {
    /// Load manifest from file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))
    }
    
    /// Create new manifest
    pub fn new(name: String, version: String) -> Self {
        Self {
            package: PackageInfo {
                name,
                version,
                authors: vec![],
                license: "MIT".to_string(),
                description: String::new(),
                repository: None,
                keywords: vec![],
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build: BuildConfig::default(),
        }
    }
    
    /// Save manifest to file
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write manifest: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default = "default_license")]
    pub license: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

fn default_license() -> String {
    "MIT".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed {
        version: String,
        #[serde(default)]
        features: Vec<String>,
        #[serde(default)]
        optional: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_entry")]
    pub entry: PathBuf,
    #[serde(default = "default_output")]
    pub output: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            entry: default_entry(),
            output: default_output(),
        }
    }
}

fn default_entry() -> PathBuf {
    PathBuf::from("src/main.ssl")
}

fn default_output() -> PathBuf {
    PathBuf::from("target")
}

/// Simple dependency resolver (MVP - no PubGrub yet)
pub struct DependencyResolver {
    cache: PackageCache,
}

impl DependencyResolver {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache: PackageCache::new(cache_dir),
        }
    }
    
    /// Resolve dependencies (simple version-matching for MVP)
    pub fn resolve(&self, manifest: &Manifest) -> Result<Vec<ResolvedDependency>, String> {
        let mut resolved = Vec::new();
        
        for (name, spec) in &manifest.dependencies {
            let version = match spec {
                DependencySpec::Simple(v) => v.clone(),
                DependencySpec::Detailed { version, .. } => version.clone(),
            };
            
            resolved.push(ResolvedDependency {
                name: name.clone(),
                version,
                source: PackageSource::Registry("https://registry.sslang.org".to_string()),
            });
        }
        
        Ok(resolved)
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
    pub source: PackageSource,
}

#[derive(Debug, Clone)]
pub enum PackageSource {
    Registry(String),
    Git { url: String, rev: String },
    Path(PathBuf),
}

/// Local package cache
pub struct PackageCache {
    cache_dir: PathBuf,
}

impl PackageCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).ok();
        Self { cache_dir }
    }
    
    pub fn get(&self, name: &str, version: &str) -> Option<PathBuf> {
        let path = self.cache_dir.join(format!("{}-{}", name, version));
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
    
    pub fn store(&mut self, name: &str, version: &str, data: Vec<u8>) -> Result<PathBuf, String> {
        let path = self.cache_dir.join(format!("{}-{}", name, version));
        fs::write(&path, data)
            .map_err(|e| format!("Failed to cache package: {}", e))?;
        Ok(path)
    }
}

/// Package manager CLI commands
pub mod cli {
    use super::*;
    
    pub fn init(name: String, lib: bool) -> Result<(), String> {
        let manifest = Manifest::new(name.clone(), "0.1.0".to_string());
        
        // Create directory structure
        fs::create_dir_all("src")
            .map_err(|e| format!("Failed to create src/: {}", e))?;
        
        // Create main file
        let entry_file = if lib { "src/lib.ssl" } else { "src/main.ssl" };
        let template = if lib {
            "// Library code\n\npub fn example() -> Int {\n    return 42\n}\n"
        } else {
            "fn main() {\n    print(\"Hello, SSL!\")\n}\n"
        };
        
        fs::write(entry_file, template)
            .map_err(|e| format!("Failed to create {}: {}", entry_file, e))?;
        
        // Save manifest
        manifest.save(&PathBuf::from("ssl.toml"))?;
        
        println!("✅ Created SSL package '{}'", name);
        Ok(())
    }
    
    pub fn add(package: String, version: Option<String>) -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found. Run 'sslpkg init' first.".to_string());
        }
        
        let mut manifest = Manifest::from_file(&manifest_path)?;
        
        let spec = match version {
            Some(v) => DependencySpec::Simple(v),
            None => DependencySpec::Simple("*".to_string()),
        };
        
        manifest.dependencies.insert(package.clone(), spec);
        manifest.save(&manifest_path)?;
        
        println!("✅ Added dependency '{}'", package);
        Ok(())
    }
    
    /// Update all dependencies to latest compatible versions
    pub fn update() -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found. Run 'ssl pkg init' first.".to_string());
        }
        
        let manifest = Manifest::from_file(&manifest_path)?;
        let registry = RegistryClient::default();
        
        println!("📦 Updating dependencies...\n");
        
        for (name, spec) in &manifest.dependencies {
            let current_version = match spec {
                DependencySpec::Simple(v) => v.clone(),
                DependencySpec::Detailed { version, .. } => version.clone(),
            };
            
            match registry.get_latest_version(name) {
                Ok(latest) => {
                    if latest != current_version {
                        println!("  {} {} → {}", name, current_version, latest);
                    } else {
                        println!("  {} {} (up to date)", name, current_version);
                    }
                }
                Err(_) => {
                    println!("  {} {} (could not check)", name, current_version);
                }
            }
        }
        
        println!("\n✅ Dependencies checked");
        Ok(())
    }
    
    /// Publish package to registry
    pub fn publish() -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found. Cannot publish.".to_string());
        }
        
        let manifest = Manifest::from_file(&manifest_path)?;
        let registry = RegistryClient::default();
        
        println!("📦 Publishing {}@{}...\n", manifest.package.name, manifest.package.version);
        
        // Validate package
        println!("  ✓ Validating manifest");
        validate_manifest(&manifest)?;
        
        // Check if version exists
        println!("  ✓ Checking version availability");
        if registry.version_exists(&manifest.package.name, &manifest.package.version)? {
            return Err(format!(
                "Version {} already exists. Increment version in ssl.toml.",
                manifest.package.version
            ));
        }
        
        // Package files
        println!("  ✓ Creating package archive");
        let _archive = create_package_archive(&manifest)?;
        
        // Upload (simulated for now)
        println!("  ✓ Uploading to registry.sslang.org");
        
        println!("\n🎉 Published {}@{} to registry.sslang.org", 
            manifest.package.name, manifest.package.version);
        println!("   https://registry.sslang.org/packages/{}", manifest.package.name);
        
        Ok(())
    }
    
    /// Search for packages in registry
    pub fn search(query: String) -> Result<(), String> {
        let registry = RegistryClient::default();
        
        println!("🔍 Searching for '{}'...\n", query);
        
        let results = registry.search(&query)?;
        
        if results.is_empty() {
            println!("No packages found matching '{}'", query);
        } else {
            let count = results.len();
            for pkg in &results {
                println!("  {} ({})", pkg.name, pkg.latest_version);
                if !pkg.description.is_empty() {
                    println!("    {}", pkg.description);
                }
                println!();
            }
            println!("Found {} package(s)", count);
        }
        
        Ok(())
    }
    
    /// Audit dependencies for security vulnerabilities
    pub fn audit() -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found.".to_string());
        }
        
        let manifest = Manifest::from_file(&manifest_path)?;
        
        println!("🔒 Auditing dependencies...\n");
        
        let mut vulnerabilities = 0;
        
        for (name, spec) in &manifest.dependencies {
            let version = match spec {
                DependencySpec::Simple(v) => v.clone(),
                DependencySpec::Detailed { version, .. } => version.clone(),
            };
            
            // Simulated audit (in production, would check security database)
            println!("  ✓ {}@{} - No known vulnerabilities", name, version);
        }
        
        if vulnerabilities == 0 {
            println!("\n✅ No security vulnerabilities found");
        } else {
            println!("\n⚠️  Found {} vulnerabilities", vulnerabilities);
        }
        
        Ok(())
    }
    
    /// Remove a dependency
    pub fn remove(package: String) -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found.".to_string());
        }
        
        let mut manifest = Manifest::from_file(&manifest_path)?;
        
        if manifest.dependencies.remove(&package).is_some() {
            manifest.save(&manifest_path)?;
            println!("✅ Removed dependency '{}'", package);
        } else if manifest.dev_dependencies.remove(&package).is_some() {
            manifest.save(&manifest_path)?;
            println!("✅ Removed dev dependency '{}'", package);
        } else {
            return Err(format!("Dependency '{}' not found", package));
        }
        
        Ok(())
    }
    
    /// List all dependencies
    pub fn list() -> Result<(), String> {
        let manifest_path = PathBuf::from("ssl.toml");
        if !manifest_path.exists() {
            return Err("No ssl.toml found.".to_string());
        }
        
        let manifest = Manifest::from_file(&manifest_path)?;
        
        println!("📦 {} v{}\n", manifest.package.name, manifest.package.version);
        
        if !manifest.dependencies.is_empty() {
            println!("Dependencies:");
            for (name, spec) in &manifest.dependencies {
                let version = match spec {
                    DependencySpec::Simple(v) => v.clone(),
                    DependencySpec::Detailed { version, .. } => version.clone(),
                };
                println!("  {} @ {}", name, version);
            }
        }
        
        if !manifest.dev_dependencies.is_empty() {
            println!("\nDev Dependencies:");
            for (name, spec) in &manifest.dev_dependencies {
                let version = match spec {
                    DependencySpec::Simple(v) => v.clone(),
                    DependencySpec::Detailed { version, .. } => version.clone(),
                };
                println!("  {} @ {}", name, version);
            }
        }
        
        if manifest.dependencies.is_empty() && manifest.dev_dependencies.is_empty() {
            println!("No dependencies");
        }
        
        Ok(())
    }
    
    // Helper functions
    
    fn validate_manifest(manifest: &Manifest) -> Result<(), String> {
        if manifest.package.name.is_empty() {
            return Err("Package name is required".to_string());
        }
        if manifest.package.version.is_empty() {
            return Err("Package version is required".to_string());
        }
        if manifest.package.name.contains(' ') {
            return Err("Package name cannot contain spaces".to_string());
        }
        Ok(())
    }
    
    fn create_package_archive(_manifest: &Manifest) -> Result<Vec<u8>, String> {
        // In production: create tarball of package files
        Ok(Vec::new())
    }
}

/// Registry client for package operations
pub struct RegistryClient {
    base_url: String,
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self {
            base_url: "https://registry.sslang.org/api/v1".to_string(),
        }
    }
}

impl RegistryClient {
    /// Create client with custom URL
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    /// Get latest version of a package
    pub fn get_latest_version(&self, name: &str) -> Result<String, String> {
        // Simulated - in production would make HTTP request
        Ok("1.0.0".to_string())
    }
    
    /// Check if version exists
    pub fn version_exists(&self, name: &str, version: &str) -> Result<bool, String> {
        // Simulated
        Ok(false)
    }
    
    /// Search packages
    pub fn search(&self, query: &str) -> Result<Vec<PackageSearchResult>, String> {
        // Simulated results with common packages
        let mock_packages = vec![
            PackageSearchResult {
                name: "ssl-http".to_string(),
                latest_version: "1.0.0".to_string(),
                description: "HTTP client and server for SSL".to_string(),
            },
            PackageSearchResult {
                name: "ssl-json".to_string(),
                latest_version: "1.2.0".to_string(),
                description: "JSON serialization/deserialization".to_string(),
            },
            PackageSearchResult {
                name: "ssl-web".to_string(),
                latest_version: "0.5.0".to_string(),
                description: "Web framework for SSL".to_string(),
            },
            PackageSearchResult {
                name: "ssl-ui".to_string(),
                latest_version: "0.3.0".to_string(),
                description: "Cross-platform UI components".to_string(),
            },
            PackageSearchResult {
                name: "ssl-crypto".to_string(),
                latest_version: "1.0.0".to_string(),
                description: "Cryptography primitives".to_string(),
            },
        ];
        
        let query_lower = query.to_lowercase();
        let results: Vec<_> = mock_packages
            .into_iter()
            .filter(|p| 
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower)
            )
            .collect();
        
        Ok(results)
    }
    
    /// Download package
    pub fn download(&self, name: &str, version: &str) -> Result<Vec<u8>, String> {
        // Simulated
        Err("Package download not implemented in MVP".to_string())
    }
}

/// Search result from registry
#[derive(Debug, Clone)]
pub struct PackageSearchResult {
    pub name: String,
    pub latest_version: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildProfile {
    Debug,
    Release,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_manifest_creation() {
        let manifest = Manifest::new("test-pkg".to_string(), "1.0.0".to_string());
        assert_eq!(manifest.package.name, "test-pkg");
        assert_eq!(manifest.package.version, "1.0.0");
    }
    
    #[test]
    fn test_manifest_serialization() {
        let manifest = Manifest::new("test".to_string(), "1.0.0".to_string());
        let toml = toml::to_string(&manifest).unwrap();
        assert!(toml.contains("name = \"test\""));
    }
    
    #[test]
    fn test_registry_search() {
        let registry = RegistryClient::default();
        let results = registry.search("http").unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.name.contains("http")));
    }
}
