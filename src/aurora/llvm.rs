// Aurora LLVM IR Generator
// Converts Aurora IR-B (CFG) into textual LLVM IR (.ll)

use crate::aurora::ir::*;
use std::fmt::Write;

pub struct LLVMGenerator {
    output: String,
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, module: &Module) -> String {
        self.output.clear();
        
        // Header
        writeln!(self.output, "; ModuleID = '{}'", module.name).unwrap();
        writeln!(self.output, "source_filename = \"{}.ssl\"", module.name).unwrap();
        writeln!(self.output).unwrap();
        
        // Functions
        for func in &module.functions {
            self.generate_function(func);
        }
        
        self.output.clone()
    }

    fn generate_function(&mut self, func: &Function) {
        // Signature
        // For now, assume all params and return types are i64 (Int)
        let params_str: Vec<String> = func.params.iter().map(|_| "i64".to_string()).collect();
        let params_list = params_str.join(", ");
        
        writeln!(self.output, "define i64 @{}({}) {{", func.name, params_list).unwrap();
        
        // Basic Blocks
        for (i, block) in func.blocks.iter().enumerate() {
            self.generate_block(block, i == 0);
        }
        
        writeln!(self.output, "}}").unwrap();
        writeln!(self.output).unwrap();
    }

    fn generate_block(&mut self, block: &BasicBlock, is_entry: bool) {
        // Entry block doesn't need a label if it's the first thing, 
        // but LLVM requires predecessors for non-entry blocks.
        // For simplicity, we label everything but the very first if strictly needed,
        // but explicit labels are safer.
        if !is_entry {
            writeln!(self.output, "{}:", self.get_label(block.id)).unwrap();
        } else {
            // LLVM entry block cannot have predecessors, so no label usually needed 
            // unless we jump to it (which is illegal for entry).
            // But we might need to name it for clarity.
            // Let's skip label for entry block for now.
        }

        for instr in &block.instructions {
            self.generate_instruction(instr);
        }
        
        self.generate_terminator(&block.terminator);
    }

    fn generate_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::BinaryOp { op, dest, lhs, rhs } => {
                let llvm_op = match op {
                    BinaryOpCode::Add => "add",
                    BinaryOpCode::Sub => "sub",
                    BinaryOpCode::Mul => "mul",
                    BinaryOpCode::Div => "sdiv", // Signed division
                    _ => "add", // Placeholder
                };
                
                writeln!(self.output, "  %{} = {} i64 {}, {}", 
                    dest, llvm_op, self.format_operand(lhs), self.format_operand(rhs)).unwrap();
            }
            Instruction::UnaryOp { .. } => {
                // TODO
            }
            Instruction::Call { .. } => {
                // TODO
            }
            _ => {
                writeln!(self.output, "  ; Unimplemented instruction").unwrap();
            }
        }
    }

    fn generate_terminator(&mut self, term: &Terminator) {
        match term {
            Terminator::Return(val) => {
                if let Some(op) = val {
                    writeln!(self.output, "  ret i64 {}", self.format_operand(op)).unwrap();
                } else {
                    writeln!(self.output, "  ret void").unwrap();
                }
            }
            Terminator::Branch(target) => {
                writeln!(self.output, "  br label %{}", self.get_label(*target)).unwrap();
            }
            Terminator::CondBranch { cond, true_target, false_target } => {
                writeln!(self.output, "  br i1 {}, label %{}, label %{}", 
                    self.format_operand(cond), self.get_label(*true_target), self.get_label(*false_target)).unwrap();
            }
            Terminator::Unreachable => {
                writeln!(self.output, "  unreachable").unwrap();
            }
        }
    }

    fn format_operand(&self, op: &Operand) -> String {
        match op {
            Operand::Register(reg) => format!("%{}", reg),
            Operand::IntImmediate(val) => format!("{}", val),
            Operand::FloatImmediate(val) => format!("{}", val), // Note: Type mismatch if used in i64 context
            _ => "undef".to_string(),
        }
    }

    fn get_label(&self, id: BlockId) -> String {
        format!("bb{}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aurora::builder::Builder;

    #[test]
    fn test_generate_simple_llvm() {
        let mut builder = Builder::new("test_module");
        builder.create_function("add_nums", vec![], "Int");
        
        let v1 = Operand::IntImmediate(10);
        let v2 = Operand::IntImmediate(20);
        let result = builder.build_add(v1, v2);
        builder.build_return(Some(result));
        
        let mut generator = LLVMGenerator::new();
        let llvm_ir = generator.generate(&builder.module);
        
        println!("{}", llvm_ir);
        
        assert!(llvm_ir.contains("define i64 @add_nums()"));
        assert!(llvm_ir.contains("add i64 10, 20"));
        assert!(llvm_ir.contains("ret i64"));
    }
}
