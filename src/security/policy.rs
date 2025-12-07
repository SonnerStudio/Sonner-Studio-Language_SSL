// Security Policy Definitions

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    /// Allow reading from specific path (glob pattern)
    FileRead(String),
    /// Allow writing to specific path (glob pattern)
    FileWrite(String),
    /// Allow network connections to specific host
    NetworkConnect(String),
    /// Allow spawning new processes/threads
    SpawnProcess,
    /// Allow usage of unsafe blocks
    UnsafeOperations,
    /// Allow loading native modules (FFI)
    LoadNativeModule,
    /// Allow all operations (Superuser)
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPolicy {
    allowed_permissions: Vec<Permission>,
    denied_permissions: Vec<Permission>, // Explicit deny overrides allow
}

impl SandboxPolicy {
    pub fn new() -> Self {
        SandboxPolicy {
            allowed_permissions: Vec::new(),
            denied_permissions: Vec::new(),
        }
    }

    pub fn allow(mut self, perm: Permission) -> Self {
        self.allowed_permissions.push(perm);
        self
    }

    pub fn deny(mut self, perm: Permission) -> Self {
        self.denied_permissions.push(perm);
        self
    }

    pub fn check(&self, request: &Permission) -> bool {
        // 1. Check if explicitly denied
        for denied in &self.denied_permissions {
            if self.matches(denied, request) {
                return false;
            }
        }

        // 2. Check if allowed
        for allowed in &self.allowed_permissions {
            if self.matches(allowed, request) {
                return true;
            }
        }

        // Default deny
        false
    }

    fn matches(&self, rule: &Permission, request: &Permission) -> bool {
        match (rule, request) {
            (Permission::All, _) => true,
            (Permission::FileRead(rule_path), Permission::FileRead(req_path)) => {
                self.glob_match(rule_path, req_path)
            },
            (Permission::FileWrite(rule_path), Permission::FileWrite(req_path)) => {
                self.glob_match(rule_path, req_path)
            },
            (Permission::NetworkConnect(rule_host), Permission::NetworkConnect(req_host)) => {
                self.glob_match(rule_host, req_host)
            },
            (Permission::SpawnProcess, Permission::SpawnProcess) => true,
            (Permission::UnsafeOperations, Permission::UnsafeOperations) => true,
            (Permission::LoadNativeModule, Permission::LoadNativeModule) => true,
            _ => false,
        }
    }

    fn glob_match(&self, pattern: &str, target: &str) -> bool {
        // Simple glob matching: * matches anything
        if pattern == "*" {
            return true;
        }
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            return target.starts_with(prefix);
        }
        pattern == target
    }
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        // Default policy: restrictive
        SandboxPolicy::new()
    }
}
