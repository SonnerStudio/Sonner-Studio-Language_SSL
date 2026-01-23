// Security Manager
// Enforces the SandboxPolicy

use crate::security::policy::{SandboxPolicy, Permission};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SecurityManager {
    policy: Arc<Mutex<SandboxPolicy>>,
}

impl SecurityManager {
    pub fn new(policy: SandboxPolicy) -> Self {
        SecurityManager {
            policy: Arc::new(Mutex::new(policy)),
        }
    }

    pub fn check_permission(&self, perm: Permission) -> Result<(), String> {
        let policy = self.policy.lock().unwrap();
        if policy.check(&perm) {
            Ok(())
        } else {
            Err(format!("Security Violation: Permission denied for {:?}", perm))
        }
    }
    
    pub fn update_policy(&self, new_policy: SandboxPolicy) {
        let mut policy = self.policy.lock().unwrap();
        *policy = new_policy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::policy::Permission;

    #[test]
    fn test_security_check() {
        let policy = SandboxPolicy::new()
            .allow(Permission::FileRead("/tmp/*".to_string()))
            .deny(Permission::FileRead("/tmp/secret.txt".to_string()));
            
        let manager = SecurityManager::new(policy);
        
        // Allowed
        assert!(manager.check_permission(Permission::FileRead("/tmp/data.txt".to_string())).is_ok());
        
        // Denied explicitly
        assert!(manager.check_permission(Permission::FileRead("/tmp/secret.txt".to_string())).is_err());
        
        // Denied by default (not in allow list)
        assert!(manager.check_permission(Permission::FileRead("/etc/passwd".to_string())).is_err());
    }
}
