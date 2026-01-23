// Semantic Normalizer - Converts SSIT Intents → SSL AST

use crate::nil::intent::*;
use crate::nil::type_inference::TypeInferenceEngine;
use crate::nil::name_gen::NameGenerator;
use crate::ast::*;

/// Semantic normalizer for Intent → AST conversion
pub struct SemanticNormalizer {
    pub type_inference: TypeInferenceEngine,
    pub name_generator: NameGenerator,
}

impl SemanticNormalizer {
    pub fn new() -> Self {
        SemanticNormalizer {
            type_inference: TypeInferenceEngine::new(),
            name_generator: NameGenerator::new(),
        }
    }
    
    /// Normalizes an intent into SSL AST statements
    pub fn normalize(&self, intent: Intent) -> Result<Vec<Statement>, String> {
        match intent {
            Intent::CreateFunction { name, parameters, intent, return_hint } => {
                self.normalize_function(name, parameters, *intent, return_hint)
            }
            
            Intent::Assignment { variable, value } => {
                self.normalize_assignment(variable, *value)
            }
            
            Intent::Loop { iterator, collection, body } => {
                self.normalize_loop(iterator, *collection, *body)
            }
            
            Intent::Conditional { condition, then_branch, else_branch } => {
                self.normalize_conditional(*condition, *then_branch, else_branch.map(|b| *b))
            }
            
            Intent::Sequence(intents) => {
                let mut statements = Vec::new();
                for intent in intents {
                    statements.extend(self.normalize(intent)?);
                }
                Ok(statements)
            }
            
            Intent::DataProcessing { operation, source, .. } => {
                self.normalize_data_processing(operation, source)
            }
            
            _ => {
                // For other intents, try to convert to expression
                let expr = self.intent_to_expression(intent)?;
                Ok(vec![Statement::ExpressionStmt(expr)])
            }
        }
    }
    
    /// Normalizes a function creation intent
    fn normalize_function(
        &self,
        name: Option<String>,
        parameters: Vec<IntentParameter>,
        body_intent: Intent,
        return_hint: Option<String>,
    ) -> Result<Vec<Statement>, String> {
        let func_name = name.unwrap_or_else(|| self.name_generator.generate_function_name());
        
        // Convert parameters
        let params: Vec<Parameter> = parameters.iter().map(|p| {
            let param_type = p.type_hint.as_ref()
                .and_then(|h| self.parse_type_hint(h))
                .or_else(|| Some(self.type_inference.infer_parameter_type(&p.name, &body_intent)))
                .unwrap_or(Type::Int); // Default to Int if can't infer
            
            Parameter {
                name: p.name.clone(),
                param_type,
            }
        }).collect();
        
        // Convert body
        let body_statements = self.normalize(body_intent.clone())?;
        
        // Infer return type
        let return_type = return_hint
            .and_then(|h| self.parse_type_hint(&h))
            .or_else(|| self.type_inference.infer_return(&body_intent));
        
        Ok(vec![Statement::FunctionDecl(FunctionDecl {
            name: func_name,
            type_params: vec![],
            params,
            return_type,
            body: body_statements,
            is_async: false,
            attributes: vec![], // SSL 3.2: No attributes for NIL-generated functions
        })])
    }
    
    /// Normalizes an assignment intent
    fn normalize_assignment(&self, variable: String, value: Intent) -> Result<Vec<Statement>, String> {
        let value_expr = self.intent_to_expression(value)?;
        
        Ok(vec![Statement::VariableDecl(VariableDecl {
            name: variable,
            mutable: false,
            var_type: None,
            value: Some(value_expr),
        })])
    }
    
    /// Normalizes a loop intent
    fn normalize_loop(&self, iterator: Option<String>, collection: Intent, body: Intent) -> Result<Vec<Statement>, String> {
        let iter_var = iterator.unwrap_or_else(|| self.name_generator.generate_variable_name());
        let collection_expr = self.intent_to_expression(collection)?;
        let body_statements = self.normalize(body)?;
        
        Ok(vec![Statement::For(ForLoop {
            var: iter_var,
            iterable: collection_expr,
            body: body_statements,
        })])
    }
    
    /// Normalizes a conditional intent
    fn normalize_conditional(
        &self,
        condition: Intent,
        then_branch: Intent,
        else_branch: Option<Intent>,
    ) -> Result<Vec<Statement>, String> {
        let condition_expr = self.intent_to_expression(condition)?;
        let then_statements = self.normalize(then_branch)?;
        let else_statements = else_branch
            .map(|b| self.normalize(b))
            .transpose()?;
        
        Ok(vec![Statement::If(IfStatement {
            condition: condition_expr,
            then_block: then_statements,
            else_block: else_statements,
        })])
    }
    
    /// Normalizes data processing intent
    fn normalize_data_processing(&self, operation: DataOp, source: Option<String>) -> Result<Vec<Statement>, String> {
        // Simplified: Create a placeholder variable with the data
        let var_name = source.unwrap_or_else(|| "data".to_string());
        
        Ok(vec![Statement::VariableDecl(VariableDecl {
            name: var_name,
            mutable: false,
            var_type: None,
            value: Some(Expression::ListLiteral(vec![])),
        })])
    }
    
    /// Converts an intent to an expression
    fn intent_to_expression(&self, intent: Intent) -> Result<Expression, String> {
        match intent {
            Intent::Literal(lit) => Ok(self.literal_to_expression(lit)),
            
            Intent::Variable(name) => Ok(Expression::Identifier(name)),
            
            Intent::BinaryOp { left, op, right } => {
                let left_expr = self.intent_to_expression(*left)?;
                let right_expr = self.intent_to_expression(*right)?;
                let operator = self.string_to_operator(&op);
                
                Ok(Expression::BinaryOp(BinaryOp {
                    left: Box::new(left_expr),
                    op: operator,
                    right: Box::new(right_expr),
                }))
            }
            
            Intent::FunctionCall { name, arguments } => {
                let args: Result<Vec<_>, _> = arguments.into_iter()
                    .map(|a| self.intent_to_expression(a))
                    .collect();
                
                Ok(Expression::FunctionCall(FunctionCall {
                    name,
                    type_args: vec![],
                    args: args?,
                }))
            }
            
            _ => Err(format!("Cannot convert intent to expression: {:?}", intent)),
        }
    }
    
    /// Converts a literal value to an expression
    fn literal_to_expression(&self, lit: LiteralValue) -> Expression {
        match lit {
            LiteralValue::Integer(i) => Expression::IntLiteral(i),
            LiteralValue::Float(f) => Expression::FloatLiteral(f),
            LiteralValue::String(s) => Expression::StringLiteral(s),
            LiteralValue::Boolean(b) => Expression::BoolLiteral(b),
            LiteralValue::List(items) => {
                let exprs: Vec<_> = items.into_iter()
                    .map(|item| self.literal_to_expression(item))
                    .collect();
                Expression::ListLiteral(exprs)
            }
        }
    }
    
    /// Converts operator string to Operator enum
    fn string_to_operator(&self, op: &str) -> Operator {
        match op {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "==" => Operator::Equals,
            "!=" => Operator::NotEquals,
            ">" => Operator::Gt,
            "<" => Operator::Lt,
            ">=" => Operator::Ge,
            "<=" => Operator::Le,
            _ => Operator::Add, // Default fallback
        }
    }
    
    /// Parses a type hint string into a Type
    fn parse_type_hint(&self, hint: &str) -> Option<Type> {
        match hint.to_lowercase().as_str() {
            "int" | "integer" => Some(Type::Int),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "bool" | "boolean" => Some(Type::Bool),
            _ => Some(Type::Custom(hint.to_string())),
        }
    }
}

impl Default for SemanticNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_literal() {
        let normalizer = SemanticNormalizer::new();
        let intent = Intent::literal_int(42);
        
        let statements = normalizer.normalize(intent).unwrap();
        assert_eq!(statements.len(), 1);
    }
    
    #[test]
    fn test_normalize_assignment() {
        let normalizer = SemanticNormalizer::new();
        let intent = Intent::Assignment {
            variable: "x".to_string(),
            value: Box::new(Intent::literal_int(10)),
        };
        
        let statements = normalizer.normalize(intent).unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::VariableDecl(var) = &statements[0] {
            assert_eq!(var.name, "x");
        } else {
            panic!("Expected VariableDecl");
        }
    }
    
    #[test]
    fn test_normalize_function() {
        let normalizer = SemanticNormalizer::new();
        let intent = Intent::CreateFunction {
            name: Some("double".to_string()),
            parameters: vec![IntentParameter {
                name: "x".to_string(),
                type_hint: Some("Int".to_string()),
            }],
            intent: Box::new(Intent::BinaryOp {
                left: Box::new(Intent::var("x".to_string())),
                op: "*".to_string(),
                right: Box::new(Intent::literal_int(2)),
            }),
            return_hint: Some("Int".to_string()),
        };
        
        let statements = normalizer.normalize(intent).unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::FunctionDecl(func) = &statements[0] {
            assert_eq!(func.name, "double");
            assert_eq!(func.params.len(), 1);
        } else {
            panic!("Expected FunctionDecl");
        }
    }
    
    #[test]
    fn test_normalize_conditional() {
        let normalizer = SemanticNormalizer::new();
        let intent = Intent::Conditional {
            condition: Box::new(Intent::BinaryOp {
                left: Box::new(Intent::var("x".to_string())),
                op: ">".to_string(),
                right: Box::new(Intent::literal_int(0)),
            }),
            then_branch: Box::new(Intent::literal_int(1)),
            else_branch: Some(Box::new(Intent::literal_int(0))),
        };
        
        let statements = normalizer.normalize(intent).unwrap();
        assert_eq!(statements.len(), 1);
        
        if let Statement::If(_) = &statements[0] {
            // Success
        } else {
            panic!("Expected If statement");
        }
    }
}
