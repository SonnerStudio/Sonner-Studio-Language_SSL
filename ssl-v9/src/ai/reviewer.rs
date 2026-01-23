use crate::ast::Statement;
use crate::ai::provider::LLMProvider;

/// AI-powered code reviewer
pub struct CodeReviewer {
    provider: Box<dyn LLMProvider>,
}

impl CodeReviewer {
    /// Create a new code reviewer
    pub fn new(provider: Box<dyn LLMProvider>) -> Self {
        CodeReviewer { provider }
    }
    
    /// Review code for issues
    pub fn review(&self, code: &str) -> Result<CodeReview, String> {
        let prompt = format!(
            "Review this SSL code for security vulnerabilities, performance issues, and best practice violations:\n\n```ssl\n{}\n```\n\nProvide a structured review with:\n1. Security issues (if any)\n2. Performance concerns (if any)\n3. Best practice violations (if any)\n4. Suggestions for improvement\n\nBe concise and specific.",
            code
        );
        
        let response = self.provider.complete(&prompt)?;
        
        Ok(CodeReview {
            code: code.to_string(),
            analysis: response,
            security_issues: vec![], // TODO: Parse from response
            performance_issues: vec![],
            suggestions: vec![],
        })
    }
    
    /// Quick security check
    pub fn security_check(&self, code: &str) -> Result<Vec<SecurityIssue>, String> {
        let prompt = format!(
            "Analyze this SSL code for SECURITY VULNERABILITIES ONLY:\n\n```ssl\n{}\n```\n\nList each vulnerability with:\n- Severity (Critical/High/Medium/Low)\n- Description\n- Line number (if applicable)\n- Fix suggestion\n\nFormat: JSON array",
            code
        );
        
        let response = self.provider.complete(&prompt)?;
        
        // TODO: Parse JSON response
        Ok(vec![])
    }
}

/// Code review result
#[derive(Debug, Clone)]
pub struct CodeReview {
    pub code: String,
    pub analysis: String,
    pub security_issues: Vec<SecurityIssue>,
    pub performance_issues: Vec<PerformanceIssue>,
    pub suggestions: Vec<String>,
}

/// Security issue found by AI
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub description: String,
    pub line: Option<usize>,
    pub fix: String,
}

/// Performance issue found by AI
#[derive(Debug, Clone)]
pub struct PerformanceIssue {
    pub description: String,
    pub impact: String,
    pub suggestion: String,
}

/// Issue severity
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock provider for testing
    struct MockProvider;
    impl LLMProvider for MockProvider {
        fn complete(&self, _prompt: &str) -> Result<String, String> {
            Ok("Code looks good. No major issues found.".to_string())
        }
    }
    
    #[test]
    fn test_code_review() {
        let reviewer = CodeReviewer::new(Box::new(MockProvider));
        let code = "fn test() { return 42 }";
        
        let review = reviewer.review(code).unwrap();
        assert!(!review.analysis.is_empty());
    }
}
