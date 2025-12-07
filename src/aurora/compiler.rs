// Aurora Compiler - AST to IR-B Lowering

use crate::ast::*;
use crate::aurora::ir::*;
use crate::aurora::builder::Builder;

pub struct Compiler {
    builder: Builder,
}

impl Compiler {
    pub fn new(module_name: &str) -> Self {
        Compiler {
            builder: Builder::new(module_name),
        }
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> Module {
        // First pass: Find all functions and declare them
        // For now, we assume top-level statements are either functions or global code
        // We wrap global code in a "main" function if needed, but for now let's focus on functions
        
        for stmt in &statements {
            if let Statement::FunctionDecl(func) = stmt {
                let params: Vec<String> = func.params.iter().map(|p| p.name.clone()).collect();
                let return_type = "Int"; // Simplified for now
                self.builder.create_function(&func.name, params, return_type);
                
                // Compile body
                self.compile_block(&func.body);
                
                // Ensure block is terminated
                // self.builder.terminate(Terminator::Return(None)); // Default return
            }
        }
        
        self.builder.module.clone()
    }

    fn compile_block(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.compile_statement(stmt);
        }
    }

    fn compile_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDecl(decl) => {
                // For now, treat variables as registers or stack slots
                // Simplified: Just evaluate the value
                if let Some(expr) = &decl.value {
                    let _val = self.compile_expression(expr);
                    // TODO: Store _val in a variable map
                }
            }
            Statement::Return(expr) => {
                let val = expr.as_ref().map(|e| self.compile_expression(e));
                self.builder.build_return(val);
            }
            Statement::ExpressionStmt(expr) => {
                self.compile_expression(expr);
            }
            Statement::If(if_stmt) => {
                self.compile_if(if_stmt);
            }
            _ => {
                // Other statements not yet implemented
            }
        }
    }

    fn compile_if(&mut self, if_stmt: &IfStatement) {
        let cond = self.compile_expression(&if_stmt.condition);
        
        let then_block = self.builder.create_block("then");
        let else_block = self.builder.create_block("else");
        let merge_block = self.builder.create_block("merge");
        
        // Branch based on condition
        self.builder.terminate(Terminator::CondBranch {
            cond,
            true_target: then_block,
            false_target: else_block,
        });
        
        // Compile Then block
        self.builder.set_current_block(then_block);
        self.compile_block(&if_stmt.then_block);
        if !self.is_terminated() {
            self.builder.terminate(Terminator::Branch(merge_block));
        }
        
        // Compile Else block
        self.builder.set_current_block(else_block);
        if let Some(else_body) = &if_stmt.else_block {
            self.compile_block(else_body);
        }
        if !self.is_terminated() {
            self.builder.terminate(Terminator::Branch(merge_block));
        }
        
        // Continue at Merge block
        self.builder.set_current_block(merge_block);
    }
    
    fn is_terminated(&self) -> bool {
        // Helper to check if current block has a terminator
        // Simplified: assume not terminated unless explicitly set
        // In a real implementation, we'd check the current block's terminator
        false 
    }

    fn compile_expression(&mut self, expr: &Expression) -> Operand {
        match expr {
            Expression::IntLiteral(i) => Operand::IntImmediate(*i),
            Expression::FloatLiteral(f) => Operand::FloatImmediate(*f),
            Expression::BoolLiteral(b) => Operand::BoolImmediate(*b),
            Expression::BinaryOp(op) => {
                let lhs = self.compile_expression(&op.left);
                let rhs = self.compile_expression(&op.right);
                match op.op {
                    Operator::Add => self.builder.build_add(lhs, rhs),
                    Operator::Subtract => self.builder.build_sub(lhs, rhs),
                    Operator::Multiply => self.builder.build_mul(lhs, rhs),
                    // ... other ops
                    _ => Operand::Undef,
                }
            }
            Expression::Identifier(_name) => {
                // TODO: Lookup variable
                Operand::IntImmediate(0) // Placeholder
            }
            _ => Operand::Undef,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_function() {
        let mut compiler = Compiler::new("test");
        
        // fn add(x: Int, y: Int) -> Int { return 10 + 20 }
        let func = FunctionDecl {
            name: "add".to_string(),
            type_params: vec![],
            params: vec![],
            return_type: None,
            body: vec![
                Statement::Return(Some(Expression::BinaryOp(BinaryOp {
                    left: Box::new(Expression::IntLiteral(10)),
                    op: Operator::Add,
                    right: Box::new(Expression::IntLiteral(20)),
                })))
            ],
            is_async: false,
        };
        
        let module = compiler.compile(vec![Statement::FunctionDecl(func)]);
        
        assert_eq!(module.functions.len(), 1);
        let func = &module.functions[0];
        assert_eq!(func.name, "add");
        
        // Should have at least one block with instructions
        assert!(!func.blocks.is_empty());
    }
}
