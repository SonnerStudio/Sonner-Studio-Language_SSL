// Aurora LLVM Backend - Real JIT Compilation via inkwell
// Phase 1: LLVM Setup & Integration

#![cfg(feature = "llvm")]

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::types::{BasicTypeEnum, BasicType};
use inkwell::basic_block::BasicBlock as LLVMBasicBlock;
use inkwell::AddressSpace;
use std::collections::HashMap;

use crate::aurora::ir::*;

/// LLVM Backend for Aurora JIT Compiler
/// 
/// Compiles Aurora IR-B (CFG) to native machine code via LLVM
pub struct LLVMBackend<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    
    // Register tracking for value resolution
    register_values: HashMap<Reg, BasicValueEnum<'ctx>>,
    
    // Basic block mapping
    basic_blocks: HashMap<usize, LLVMBasicBlock<'ctx>>,
}

impl<'ctx> LLVMBackend<'ctx> {
    /// Create a new LLVM backend instance
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, String> {
        // Create LLVM module
        let module = context.create_module(module_name);
        
        // Create instruction builder
        let builder = context.create_builder();
        
        // Create JIT execution engine with aggressive optimization
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .map_err(|e| format!("Failed to create JIT execution engine: {}", e))?;
        
        Ok(Self {
            context,
            module,
            builder,
            execution_engine,
            register_values: HashMap::new(),
            basic_blocks: HashMap::new(),
        })
    }
    
    /// Get LLVM type for Aurora type
    fn get_llvm_type(&self, _aurora_type: &str) -> BasicTypeEnum<'ctx> {
        // For Phase 1: All values are i64 (Int)
        // Phase 2 will add: Float (f64), Bool (i1), String (i8*)
        self.context.i64_type().as_basic_type_enum()
    }
    
    /// Get or create function
    fn get_or_create_function(&self, name: &str, param_count: usize) -> FunctionValue<'ctx> {
        // Check if function already exists
        if let Some(func) = self.module.get_function(name) {
            return func;
        }
        
        // Create function type: (i64, i64, ...) -> i64
        let i64_type = self.context.i64_type();
        let param_types: Vec<_> = (0..param_count)
            .map(|_| i64_type.into())
            .collect();
        
        let fn_type = i64_type.fn_type(&param_types, false);
        
        // Add function to module
        self.module.add_function(name, fn_type, None)
    }
    
    /// Store value for register
    fn set_register(&mut self, reg: Reg, value: BasicValueEnum<'ctx>) {
        self.register_values.insert(reg, value);
    }
    
    /// Get value from register
    fn get_register(&self, reg: &Reg) -> Result<BasicValueEnum<'ctx>, String> {
        self.register_values.get(reg)
            .copied()
            .ok_or_else(|| format!("Register %{} not found", reg))
    }
    
    /// Get value from operand
    fn get_operand_value(&self, operand: &Operand) -> Result<IntValue<'ctx>, String> {
        match operand {
            Operand::Register(reg) => {
                let value = self.get_register(reg)?;
                Ok(value.into_int_value())
            }
            Operand::IntImmediate(n) => {
                Ok(self.context.i64_type().const_int(*n as u64, true))
            }
            _ => Err(format!("Unsupported operand type: {:?}", operand))
        }
    }
    
    /// Compile Aurora function to LLVM IR and native code
    pub fn compile_function(&mut self, func: &Function) -> Result<(), String> {
        // Create LLVM function
        // Function params in IR are Vec<String> (param names), not Vec<Reg>
        let llvm_func = self.get_or_create_function(&func.name, func.params.len());
        
        // Clear state
        self.register_values.clear();
        self.basic_blocks.clear();
        
        // NOTE: In Aurora IR, params are just names (Vec<String>)
        // We'll need to map them to registers manually based on usage
        // For now, we assume parameters are passed via registers %0, %1, etc.
        for (i, _param_name) in func.params.iter().enumerate() {
            let param_value = llvm_func.get_nth_param(i as u32)
                .ok_or_else(|| format!("Parameter {} not found", i))?;
            // Store parameter in register %i
            self.set_register(i, param_value);
        }
        
        // Create LLVM basic blocks for all Aurora blocks
        for block in &func.blocks {
            let bb_name = if block.id == 0 {
                "entry".to_string()
            } else {
                format!("bb{}", block.id)
            };
            
            let llvm_bb = self.context.append_basic_block(llvm_func, &bb_name);
            self.basic_blocks.insert(block.id, llvm_bb);
        }
        
        // Compile each basic block
        for block in &func.blocks {
            self.compile_basic_block(block)?;
        }
        
        // Verify function
        if llvm_func.verify(true) {
            Ok(())
        } else {
            Err(format!("LLVM function verification failed: {}", func.name))
        }
    }
    
    /// Compile a basic block
    fn compile_basic_block(&mut self, block: &BasicBlock) -> Result<(), String> {
        // Position builder at end of this block
        let llvm_bb = self.basic_blocks.get(&block.id)
            .ok_or_else(|| format!("Basic block {} not found", block.id))?;
        
        self.builder.position_at_end(*llvm_bb);
        
        // Compile instructions
        for instr in &block.instructions {
            self.compile_instruction(instr)?;
        }
        
        // Compile terminator
        self.compile_terminator(&block.terminator)?;
        
        Ok(())
    }
    
    /// Compile an instruction
    fn compile_instruction(&mut self, instr: &Instruction) -> Result<(), String> {
        match instr {
            Instruction::BinaryOp { op, dest, lhs, rhs } => {
                let left = self.get_operand_value(lhs)?;
                let right = self.get_operand_value(rhs)?;
                
                let result = match op {
                    // Arithmetic operations
                    BinaryOpCode::Add => self.builder.build_int_add(left, right, "add")
                        .map_err(|e| format!("Failed to build add: {}", e))?,
                    BinaryOpCode::Sub => self.builder.build_int_sub(left, right, "sub")
                        .map_err(|e| format!("Failed to build sub: {}", e))?,
                    BinaryOpCode::Mul => self.builder.build_int_mul(left, right, "mul")
                        .map_err(|e| format!("Failed to build mul: {}", e))?,
                    BinaryOpCode::Div => self.builder.build_int_signed_div(left, right, "div")
                        .map_err(|e| format!("Failed to build div: {}", e))?,
                    BinaryOpCode::Rem => self.builder.build_int_signed_rem(left, right, "rem")
                        .map_err(|e| format!("Failed to build rem: {}", e))?,
                    
                    // Comparison operations - return i64 (0 or 1)
                    BinaryOpCode::Eq => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::EQ, left, right, "eq"
                        ).map_err(|e| format!("Failed to build eq: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "eq_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    BinaryOpCode::Ne => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::NE, left, right, "ne"
                        ).map_err(|e| format!("Failed to build ne: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "ne_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    BinaryOpCode::Lt => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::SLT, left, right, "lt"
                        ).map_err(|e| format!("Failed to build lt: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "lt_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    BinaryOpCode::Le => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::SLE, left, right, "le"
                        ).map_err(|e| format!("Failed to build le: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "le_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    BinaryOpCode::Gt => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::SGT, left, right, "gt"
                        ).map_err(|e| format!("Failed to build gt: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "gt_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    BinaryOpCode::Ge => {
                        let cmp = self.builder.build_int_compare(
                            inkwell::IntPredicate::SGE, left, right, "ge"
                        ).map_err(|e| format!("Failed to build ge: {}", e))?;
                        self.builder.build_int_z_extend(cmp, self.context.i64_type(), "ge_ext")
                            .map_err(|e| format!("Failed to extend: {}", e))?
                    }
                    
                    // Logical operations
                    BinaryOpCode::And => self.builder.build_and(left, right, "and")
                        .map_err(|e| format!("Failed to build and: {}", e))?,
                    BinaryOpCode::Or => self.builder.build_or(left, right, "or")
                        .map_err(|e| format!("Failed to build or: {}", e))?,
                    BinaryOpCode::Xor => self.builder.build_xor(left, right, "xor")
                        .map_err(|e| format!("Failed to build xor: {}", e))?,
                    
                    _ => return Err(format!("Unsupported binary op: {:?}", op)),
                };
                
                self.set_register(*dest, result.into());
                Ok(())
            }
            
            Instruction::Call { dest, func, args } => {
                // Get function
                let callee = self.module.get_function(func)
                    .ok_or_else(|| format!("Function {} not found", func))?;
                
                // Get argument values
                let arg_values: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.get_operand_value(arg).map(|v| v.into()))
                    .collect();
                let arg_values = arg_values?;
                
                // Build call
                let call_site = self.builder.build_call(callee, &arg_values, "call")
                    .map_err(|e| format!("Failed to build call: {}", e))?;
                
                // Store result if destination exists
                if let Some(reg) = dest {
                    if let Some(ret_val) = call_site.try_as_basic_value().left() {
                        self.set_register(*reg, ret_val);
                    }
                }
                
                Ok(())
            }
            
            _ => {
                // Phase 1: Only BinaryOp and Call supported
                Err(format!("Unsupported instruction: {:?}", instr))
            }
        }
    }
    
    /// Compile a terminator
    fn compile_terminator(&mut self, term: &Terminator) -> Result<(), String> {
        match term {
            Terminator::Return(operand) => {
                if let Some(op) = operand {
                    let value = self.get_operand_value(op)?;
                    self.builder.build_return(Some(&value))
                        .map_err(|e| format!("Failed to build return: {}", e))?;
                } else {
                    self.builder.build_return(None)
                        .map_err(|e| format!("Failed to build return: {}", e))?;
                }
                Ok(())
            }
            
            Terminator::Branch(target) => {
                let target_bb = self.basic_blocks.get(target)
                    .ok_or_else(|| format!("Branch target {} not found", target))?;
                
                self.builder.build_unconditional_branch(*target_bb)
                    .map_err(|e| format!("Failed to build branch: {}", e))?;
                Ok(())
            }
            
            Terminator::CondBranch { cond, true_target, false_target } => {
                let condition = self.get_operand_value(cond)?;
                
                // Convert to i1 (boolean)
                let bool_cond = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    condition,
                    self.context.i64_type().const_zero(),
                    "cond"
                ).map_err(|e| format!("Failed to build compare: {}", e))?;
                
                let then_bb = self.basic_blocks.get(true_target)
                    .ok_or_else(|| format!("True branch target {} not found", true_target))?;
                let else_bb = self.basic_blocks.get(false_target)
                    .ok_or_else(|| format!("False branch target {} not found", false_target))?;
                
                self.builder.build_conditional_branch(bool_cond, *then_bb, *else_bb)
                    .map_err(|e| format!("Failed to build conditional branch: {}", e))?;
                Ok(())
            }
            
            Terminator::Unreachable => {
                self.builder.build_unreachable()
                    .map_err(|e| format!("Failed to build unreachable: {}", e))?;
                Ok(())
            }
        }
    }
    
    /// Execute compiled function
    pub unsafe fn execute_function<R>(&self, func_name: &str) -> Result<R, String> {
        let function: JitFunction<unsafe extern "C" fn() -> R> = 
            self.execution_engine.get_function(func_name)
                .map_err(|e| format!("Failed to get function {}: {}", func_name, e))?;
        
        Ok(function.call())
    }
    
    /// Get LLVM IR as string (for debugging)
    pub fn get_llvm_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_arithmetic() {
        let context = Context::create();
        let mut backend = LLVMBackend::new(&context, "test_module").unwrap();
        
        // Create simple function: fn test() -> i64 { return 10 + 20 }
        let func = Function {
            name: "test".to_string(),
            params: vec![],  // Vec<String>
            entry_block: 0,
            return_type: "i64".to_string(),
            blocks: vec![
                BasicBlock {
                    id: 0,
                    label: "entry".to_string(),
                    instructions: vec![
                        Instruction::BinaryOp {
                            op: BinaryOpCode::Add,
                            dest: 0,  // Reg is usize
                            lhs: Operand::IntImmediate(10),
                            rhs: Operand::IntImmediate(20),
                        }
                    ],
                    terminator: Terminator::Return(Some(Operand::Register(0))),
                }
            ],
        };
        
        backend.compile_function(&func).unwrap();
        
        let result: i64 = unsafe { backend.execute_function("test").unwrap() };
        assert_eq!(result, 30);
    }
}
