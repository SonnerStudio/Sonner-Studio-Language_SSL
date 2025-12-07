// Aurora IR Builder
// Helper struct to construct the Control Flow Graph

use crate::aurora::ir::*;

pub struct Builder {
    pub module: Module,
    pub current_function: Option<usize>, // Index of current function
    pub current_block: Option<BlockId>,  // ID of current block
    pub next_reg: Reg,
}

impl Builder {
    pub fn new(module_name: &str) -> Self {
        Builder {
            module: Module::new(module_name),
            current_function: None,
            current_block: None,
            next_reg: 1, // Start from 1, 0 can be reserved/invalid
        }
    }

    /// Starts a new function definition
    pub fn create_function(&mut self, name: &str, params: Vec<String>, return_type: &str) -> usize {
        let func = Function::new(name, params, return_type);
        self.module.add_function(func);
        let func_idx = self.module.functions.len() - 1;
        self.current_function = Some(func_idx);
        self.next_reg = 1; // Reset registers for new function
        
        // Create entry block
        let entry_block = self.create_block("entry");
        self.set_current_block(entry_block);
        
        func_idx
    }

    /// Creates a new basic block in the current function
    pub fn create_block(&mut self, label: &str) -> BlockId {
        let func_idx = self.current_function.expect("No active function");
        let func = &mut self.module.functions[func_idx];
        let id = func.blocks.len();
        let block = BasicBlock::new(id, label);
        func.add_block(block)
    }

    /// Sets the current insertion block
    pub fn set_current_block(&mut self, block_id: BlockId) {
        self.current_block = Some(block_id);
    }

    /// Gets a new virtual register
    pub fn new_reg(&mut self) -> Reg {
        let reg = self.next_reg;
        self.next_reg += 1;
        reg
    }

    /// Adds an instruction to the current block
    pub fn emit(&mut self, instr: Instruction) {
        let func_idx = self.current_function.expect("No active function");
        let block_id = self.current_block.expect("No active block");
        
        self.module.functions[func_idx].blocks[block_id].push(instr);
    }

    /// Sets the terminator for the current block
    pub fn terminate(&mut self, term: Terminator) {
        let func_idx = self.current_function.expect("No active function");
        let block_id = self.current_block.expect("No active block");
        
        self.module.functions[func_idx].blocks[block_id].set_terminator(term);
    }

    // --- High-level instruction builders ---

    pub fn build_add(&mut self, lhs: Operand, rhs: Operand) -> Operand {
        let dest = self.new_reg();
        self.emit(Instruction::BinaryOp {
            op: BinaryOpCode::Add,
            dest,
            lhs,
            rhs,
        });
        Operand::Register(dest)
    }
    
    pub fn build_sub(&mut self, lhs: Operand, rhs: Operand) -> Operand {
        let dest = self.new_reg();
        self.emit(Instruction::BinaryOp {
            op: BinaryOpCode::Sub,
            dest,
            lhs,
            rhs,
        });
        Operand::Register(dest)
    }

    pub fn build_mul(&mut self, lhs: Operand, rhs: Operand) -> Operand {
        let dest = self.new_reg();
        self.emit(Instruction::BinaryOp {
            op: BinaryOpCode::Mul,
            dest,
            lhs,
            rhs,
        });
        Operand::Register(dest)
    }
    
    pub fn build_return(&mut self, val: Option<Operand>) {
        self.terminate(Terminator::Return(val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_simple() {
        let mut builder = Builder::new("test");
        builder.create_function("main", vec![], "Int");
        
        let v1 = Operand::IntImmediate(10);
        let v2 = Operand::IntImmediate(20);
        let result = builder.build_add(v1, v2);
        
        builder.build_return(Some(result));
        
        let func = &builder.module.functions[0];
        assert_eq!(func.blocks.len(), 1);
        assert_eq!(func.blocks[0].instructions.len(), 1);
        
        if let Terminator::Return(Some(Operand::Register(r))) = &func.blocks[0].terminator {
            assert_eq!(*r, 1);
        } else {
            panic!("Expected Return with Register 1");
        }
    }
}
