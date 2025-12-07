//! SSL 4.0 Content-Addressable Code
//!
//! Hash-based code identification for distributed systems (Unison-inspired).

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Content hash for code
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentHash(pub String);

impl ContentHash {
    /// Create hash from content
    pub fn from_content(content: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let hash = hasher.finish();
        Self(format!("sha256:{:016x}", hash))
    }
    
    /// Create from existing hash string
    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }
    
    /// Get short hash (first 8 chars)
    pub fn short(&self) -> &str {
        &self.0[7..15.min(self.0.len())]
    }
}

/// Content-addressable code store
pub struct CodeStore {
    /// Hash -> Code content
    code: HashMap<ContentHash, String>,
    /// Name -> Hash mapping
    names: HashMap<String, ContentHash>,
    /// Hash -> Dependencies
    dependencies: HashMap<ContentHash, Vec<ContentHash>>,
}

impl Default for CodeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeStore {
    /// Create new code store
    pub fn new() -> Self {
        Self {
            code: HashMap::new(),
            names: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    /// Store code and get its hash
    pub fn store(&mut self, content: &str) -> ContentHash {
        let hash = ContentHash::from_content(content);
        self.code.insert(hash.clone(), content.to_string());
        hash
    }
    
    /// Store with name binding
    pub fn store_named(&mut self, name: &str, content: &str) -> ContentHash {
        let hash = self.store(content);
        self.names.insert(name.to_string(), hash.clone());
        hash
    }
    
    /// Get code by hash
    pub fn get(&self, hash: &ContentHash) -> Option<&String> {
        self.code.get(hash)
    }
    
    /// Get hash by name
    pub fn get_hash(&self, name: &str) -> Option<&ContentHash> {
        self.names.get(name)
    }
    
    /// Get code by name
    pub fn get_by_name(&self, name: &str) -> Option<&String> {
        self.get_hash(name).and_then(|h| self.get(h))
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, from: &ContentHash, to: &ContentHash) {
        self.dependencies
            .entry(from.clone())
            .or_default()
            .push(to.clone());
    }
    
    /// Get dependencies
    pub fn get_dependencies(&self, hash: &ContentHash) -> Vec<&ContentHash> {
        self.dependencies
            .get(hash)
            .map(|deps| deps.iter().collect())
            .unwrap_or_default()
    }
    
    /// Check if hash exists
    pub fn contains(&self, hash: &ContentHash) -> bool {
        self.code.contains_key(hash)
    }
    
    /// Rename a binding (keeps same hash)
    pub fn rename(&mut self, old_name: &str, new_name: &str) -> bool {
        if let Some(hash) = self.names.remove(old_name) {
            self.names.insert(new_name.to_string(), hash);
            true
        } else {
            false
        }
    }
}

/// Content-addressable function definition
#[derive(Debug, Clone)]
pub struct HashedFunction {
    pub hash: ContentHash,
    pub name: String,
    pub params: Vec<(String, String)>, // (name, type)
    pub return_type: Option<String>,
    pub body_hash: ContentHash,
}

impl HashedFunction {
    /// Create new hashed function
    pub fn new(name: &str, params: Vec<(&str, &str)>, return_type: Option<&str>, body: &str) -> Self {
        let body_hash = ContentHash::from_content(body);
        
        // Create signature for function hash
        let sig = format!("fn {}({}) -> {:?}", 
            name,
            params.iter().map(|(n, t)| format!("{}: {}", n, t)).collect::<Vec<_>>().join(", "),
            return_type
        );
        let func_content = format!("{}\n{}", sig, body);
        let hash = ContentHash::from_content(&func_content);
        
        Self {
            hash,
            name: name.to_string(),
            params: params.into_iter().map(|(n, t)| (n.to_string(), t.to_string())).collect(),
            return_type: return_type.map(String::from),
            body_hash,
        }
    }
}

/// Generate import statement using hash
pub fn hash_import(hash: &ContentHash, alias: &str) -> String {
    format!("use \"{}\" as {}", hash.0, alias)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_content_hash() {
        let h1 = ContentHash::from_content("fn add(a, b) { a + b }");
        let h2 = ContentHash::from_content("fn add(a, b) { a + b }");
        let h3 = ContentHash::from_content("fn add(a, b) { a - b }");
        
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
    
    #[test]
    fn test_code_store() {
        let mut store = CodeStore::new();
        
        let hash = store.store_named("add", "fn add(a, b) { a + b }");
        
        assert!(store.contains(&hash));
        assert_eq!(store.get_hash("add"), Some(&hash));
        assert!(store.get_by_name("add").is_some());
    }
    
    #[test]
    fn test_rename() {
        let mut store = CodeStore::new();
        store.store_named("foo", "fn foo() {}");
        
        assert!(store.rename("foo", "bar"));
        assert!(store.get_by_name("foo").is_none());
        assert!(store.get_by_name("bar").is_some());
    }
}
