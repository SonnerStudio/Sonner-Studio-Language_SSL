use crate::ai::provider::LLMProvider;
use crate::ast::{Statement, FunctionDecl};

/// AI-powered test generator
pub struct TestGenerator {
    provider: Box<dyn LLMProvider>,
}

impl TestGenerator {
    /// Create a new test generator
    pub fn new(provider: Box<dyn LLMProvider>) -> Self {
        TestGenerator { provider }
    }
    
    /// Generate tests for a function
    pub fn generate_tests(&self, function: &str) -> Result<String, String> {
        let prompt = format!(
            "Generate comprehensive unit tests for this SSL function:\n\n```ssl\n{}\n```\n\nCreate tests that cover:\n1. Happy path (normal cases)\n2. Edge cases\n3. Error cases\n4. Boundary values\n\nReturn ONLY the SSL test code, no explanations.",
            function
        );
        
        let response = self.provider.complete(&prompt)?;
        
        // Clean up response (remove markdown fences if present)
        let cleaned = response
            .trim()
            .trim_start_matches("```ssl")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();
        
        Ok(cleaned)
    }
    
    /// Generate tests with specific coverage target
    pub fn generate_with_coverage(
        &self,
        function: &str,
        target_coverage: f32,
    ) -> Result<String, String> {
        let prompt = format!(
            "Generate unit tests for this SSL function with {:.0}% code coverage:\n\n```ssl\n{}\n```\n\nEnsure all code paths are tested. Return ONLY SSL test code.",
            target_coverage * 100.0,
            function
        );
        
        let response = self.provider.complete(&prompt)?;
        
        let cleaned = response
            .trim()
            .trim_start_matches("```ssl")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();
        
        Ok(cleaned)
    }
    
    /// Auto-generate tests for entire file
    pub fn auto_test_file(&self, source: &str) -> Result<Vec<GeneratedTest>, String> {
        let prompt = format!(
            "Analyze this SSL code and generate tests for all functions:\n\n```ssl\n{}\n```\n\nFor each function, provide:\n1. Function name\n2. Test code\n3. Coverage estimate\n\nFormat as JSON array.",
            source
        );
        
        let _response = self.provider.complete(&prompt)?;
        
        // TODO: Parse JSON response
        Ok(vec![])
    }
}

/// Generated test result
#[derive(Debug, Clone)]
pub struct GeneratedTest {
    pub function_name: String,
    pub test_code: String,
    pub coverage_estimate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockProvider;
    impl LLMProvider for MockProvider {
        fn complete(&self, _prompt: &str) -> Result<String, String> {
            Ok(r#"
fn test_add() {
    assert_eq!(add(2, 3), 5)
    assert_eq!(add(0, 0), 0)
    assert_eq!(add(-1, 1), 0)
}
"#.to_string())
        }
    }
    
    #[test]
    fn test_generate_tests() {
        let generator = TestGenerator::new(Box::new(MockProvider));
        let function = "fn add(a: Int, b: Int) -> Int { return a + b }";
        
        let tests = generator.generate_tests(function).unwrap();
        assert!(tests.contains("test_add"));
    }
}
