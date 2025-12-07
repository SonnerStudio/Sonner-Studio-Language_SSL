// Aurora Optimizer - LLVM IR Optimization Passes
// Implements functional pattern optimizations for SSL 3.0

use crate::aurora::ir::*;
use std::collections::{HashMap, HashSet};

pub struct Optimizer {
    /// Track which functions are tail-recursive
    tail_recursive_functions: HashSet<String>,
    
    /// Inline candidates (small functions)
    inline_candidates: HashMap<String, Function>,
    
    /// Constants for folding
    constants: HashMap<Reg, Operand>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            tail_recursive_functions: HashSet::new(),
            inline_candidates: HashMap::new(),
            constants: HashMap::new(),
        }
    }
    
    /// Run all optimization passes on a module
    pub fn optimize(&mut self, module: &mut Module) {
        // Pass 1: Detect tail-recursive functions
        self.detect_tail_recursion(module);
        
        // Pass 2: Constant folding
        self.constant_folding(module);
        
        // Pass 3: Dead code elimination
        self.dead_code_elimination(module);
        
        // Pass 4: Inline small functions
        self.inline_expansion(module);
        
        // Pass 5: Transform tail recursion to loops
        self.tail_call_to_loop(module);
        
        // Pass 6: Loop unrolling
        self.loop_unrolling(module);
    }
    
    /// Detect tail-recursive functions
    fn detect_tail_recursion(&mut self, module: &Module) {
        for func in &module.functions {
            if self.is_tail_recursive(func) {
                self.tail_recursive_functions.insert(func.name.clone());
            }
        }
    }
    
    /// Check if a function is tail-recursive
    fn is_tail_recursive(&self, func: &Function) -> bool {
        // Check each block's terminator
        for block in &func.blocks {
            if let Terminator::Return(Some(Operand::Register(_))) = &block.terminator {
                // Check if the last instruction before return is a recursive call
                if let Some(Instruction::Call { func: function, .. }) = block.instructions.last() {
                    if function == &func.name {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Constant folding - evaluate constant expressions at compile time
    fn constant_folding(&mut self, module: &mut Module) {
        for func in &mut module.functions {
            for block in &mut func.blocks {
                let mut folded_instructions = Vec::new();
                
                for instr in &block.instructions {
                    match instr {
                        Instruction::BinaryOp { op, dest, lhs, rhs } => {
                            // Try to fold if both operands are constants
                            if let (Some(lhs_val), Some(rhs_val)) = 
                                (self.get_constant_value(lhs), self.get_constant_value(rhs)) {
                                
                                let result_val = self.fold_binary_op(op.clone(), lhs_val, rhs_val);
                                
                                // Record the constant result
                                self.constants.insert(*dest, result_val.clone());
                                
                                // Replace instruction with constant assignment
                                // (In real LLVM, this would be handled differently)
                                folded_instructions.push(Instruction::BinaryOp {
                                    op: op.clone(),
                                    dest: *dest,
                                    lhs: result_val,
                                    rhs: Operand::IntImmediate(0), // Dummy
                                });
                            } else {
                                folded_instructions.push(instr.clone());
                            }
                        }
                        _ => folded_instructions.push(instr.clone()),
                    }
                }
                
                block.instructions = folded_instructions;
            }
        }
    }
    
    /// Get constant value if operand is a known constant
    fn get_constant_value(&self, op: &Operand) -> Option<i64> {
        match op {
            Operand::IntImmediate(val) => Some(*val),
            Operand::Register(reg) => {
                if let Some(Operand::IntImmediate(val)) = self.constants.get(reg) {
                    Some(*val)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// Fold binary operation with constant operands
    fn fold_binary_op(&self, op: BinaryOpCode, lhs: i64, rhs: i64) -> Operand {
        let result = match op {
            BinaryOpCode::Add => lhs + rhs,
            BinaryOpCode::Sub => lhs - rhs,
            BinaryOpCode::Mul => lhs * rhs,
            BinaryOpCode::Div => if rhs != 0 { lhs / rhs } else { 0 },
            _ => 0,
        };
        Operand::IntImmediate(result)
    }
    
    /// Dead code elimination - remove unused instructions
    fn dead_code_elimination(&self, module: &mut Module) {
        for func in &mut module.functions {
            for block in &mut func.blocks {
                // Track which registers are used
                let mut used_registers = HashSet::new();
                
                // Scan terminator for used registers
                match &block.terminator {
                    Terminator::Return(Some(Operand::Register(reg))) => {
                        used_registers.insert(*reg);
                    }
                    Terminator::CondBranch { cond: Operand::Register(reg), .. } => {
                        used_registers.insert(*reg);
                    }
                    _ => {}
                }
                
                // Scan instructions backwards to find dependencies
                for instr in block.instructions.iter().rev() {
                    if let Instruction::BinaryOp { dest, lhs, rhs, .. } = instr {
                        if used_registers.contains(dest) {
                            if let Operand::Register(reg) = lhs {
                                used_registers.insert(*reg);
                            }
                            if let Operand::Register(reg) = rhs {
                                used_registers.insert(*reg);
                            }
                        }
                    }
                }
                
                // Keep only instructions that produce used registers
                block.instructions.retain(|instr| {
                    if let Instruction::BinaryOp { dest, .. } = instr {
                        used_registers.contains(dest)
                    } else {
                        true // Keep non-binary-op instructions
                    }
                });
            }
        }
    }
    
    /// Inline small functions
    fn inline_expansion(&mut self, module: &mut Module) {
        // Step 1: Identify inlineable functions (small, non-recursive)
        for func in &module.functions {
            if self.is_inlineable(func) {
                self.inline_candidates.insert(func.name.clone(), func.clone());
            }
        }
        
        // Step 2: For each function, find and inline call sites
        for func in &mut module.functions {
            for block in &mut func.blocks {
                let mut new_instructions = Vec::new();
                
                for instr in &block.instructions {
                    match instr {
                        Instruction::Call { dest, func: callee, args } => {
                            // Check if this call should be inlined
                            if let Some(target) = self.inline_candidates.get(callee) {
                                // Inline the function body
                                let inlined = self.inline_function_body(target, args, dest);
                                new_instructions.extend(inlined);
                            } else {
                                new_instructions.push(instr.clone());
                            }
                        }
                        _ => new_instructions.push(instr.clone()),
                    }
                }
                
                block.instructions = new_instructions;
            }
        }
    }
    
    /// Check if a function should be inlined
    fn is_inlineable(&self, func: &Function) -> bool {
        // Count total instructions
        let instr_count: usize = func.blocks.iter()
            .map(|b| b.instructions.len())
            .sum();
        
        // Inline if:
        // - Small (< 5 instructions for now)
        // - Has simple control flow (single block preferred)
        // - Not recursive (not in tail_recursive_functions)
        
        let is_small = instr_count <= 5;
        let is_simple = func.blocks.len() <= 2; // Entry + maybe return
        let not_recursive = !self.tail_recursive_functions.contains(&func.name);
        
        is_small && is_simple && not_recursive
    }
    
    /// Inline a function body at call site
    fn inline_function_body(
        &self,
        target: &Function,
        args: &[Operand],
        dest: &Option<Reg>,
    ) -> Vec<Instruction> {
        let mut inlined = Vec::new();
        
        // For simple functions with single block, just copy instructions
        // and remap registers
        if let Some(body_block) = target.blocks.first() {
            // Parameter mapping: target.params[i] -> args[i]
            // For now, simplified: assume direct register remapping
            
            for instr in &body_block.instructions {
                // Clone instruction and potentially remap registers
                // In a real implementation, we'd build a register mapping table
                inlined.push(instr.clone());
            }
        }
        
        inlined
    }
    
    /// Transform tail recursion to loops
    /// 
    /// This is a simplified implementation that demonstrates the concept.
    /// A full implementation would:
    /// 1. Create proper loop header with phi nodes
    /// 2. Remap all registers correctly
    /// 3. Handle multiple recursive call sites
    /// 4. Preserve function semantics exactly
    fn tail_call_to_loop(&mut self, module: &mut Module) {
        for func in &mut module.functions {
            if !self.tail_recursive_functions.contains(&func.name) {
                continue;
            }
            
            // Simplified tail-call transformation:
            // For functions detected as tail-recursive, we mark them
            // In a real implementation, we would:
            // 
            // 1. Create loop_header block
            // 2. Add phi nodes for each parameter
            // 3. Find all tail-recursive call sites
            // 4. Replace "call + return" with parameter updates + branch to loop_header
            // 5. Restructure CFG to eliminate recursion
            
            // For now, we've at least identified these functions
            // The detection alone helps with understanding code patterns
            
            // Future enhancement: Actual CFG restructuring
            // This would require:
            // - BasicBlock manipulation
            // - Phi node creation (not yet in IR)
            // - Register remapping tables
            // - Proper terminator updates
        }
    }
    
    /// Detect and unroll small loops
    /// 
    /// Simplified implementation - detects potential loops in CFG
    /// Real implementation would:
    /// 1. Identify back-edges (loop detection)
    /// 2. Analyze iteration count (if statically known)
    /// 3. Unroll loop body N times
    /// 4. Remove loop control flow
    pub fn loop_unrolling(&mut self, module: &mut Module) {
        for func in &mut module.functions {
            // Detect loops by finding blocks that branch back to earlier blocks
            for (i, block) in func.blocks.iter().enumerate() {
                match &block.terminator {
                    Terminator::Branch(target) if *target <= i => {
                        // Potential loop: branches to earlier block
                        // In real implementation: analyze and potentially unroll
                    }
                    Terminator::CondBranch { true_target, false_target, .. } => {
                        // Check for back-edges
                        if *true_target <= i || *false_target <= i {
                            // Potential loop detected
                            // Real implementation would:
                            // 1. Analyze loop bounds
                            // 2. Check if iteration count is small and known
                            // 3. Duplicate loop body
                            // 4. Remove loop control
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aurora::builder::Builder;
    
    #[test]
    fn test_constant_folding() {
        let mut builder = Builder::new("test");
        builder.create_function("add_constants", vec![], "Int");
        
        // Create: 10 + 20
        let v1 = Operand::IntImmediate(10);
        let v2 = Operand::IntImmediate(20);
        let result = builder.build_add(v1, v2);
        builder.build_return(Some(result));
        
        let mut module = builder.module;
        let mut optimizer = Optimizer::new();
        
        optimizer.constant_folding(&mut module);
        
        // After folding, result should be recorded
        assert!(optimizer.constants.len() > 0);
    }
    
    #[test]
    fn test_tail_recursion_detection() {
        let mut builder = Builder::new("test");
        builder.create_function("factorial", vec!["n".to_string()], "Int");
        
        // Simple tail-recursive pattern detection
        let module = builder.module;
        let optimizer = Optimizer::new();
        
        // This would detect if factorial is tail-recursive
        if let Some(func) = module.functions.first() {
            let is_tail = optimizer.is_tail_recursive(func);
            // For empty function, should be false
            assert!(!is_tail);
        }
    }
}
