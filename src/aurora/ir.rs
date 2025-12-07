// Aurora IR-B: Control Flow Graph (CFG) Representation
// This intermediate representation is designed for optimization and easy mapping to LLVM IR.

use serde::{Serialize, Deserialize};

/// Represents a compiled module in IR-B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
}

/// Represents a function in the CFG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>, // Parameter names
    pub blocks: Vec<BasicBlock>,
    pub entry_block: BlockId,
    pub return_type: String, // Simplified type representation for now
}

/// Unique identifier for a Basic Block within a function
pub type BlockId = usize;

/// Unique identifier for a Virtual Register
pub type Reg = usize;

/// A Basic Block: A sequence of instructions with a single entry and exit point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub id: BlockId,
    pub label: String, // For debugging/LLVM labels
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

/// Instructions in Three-Address Code (TAC) format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    /// dest = lhs op rhs
    BinaryOp {
        op: BinaryOpCode,
        dest: Reg,
        lhs: Operand,
        rhs: Operand,
    },
    /// dest = op src
    UnaryOp {
        op: UnaryOpCode,
        dest: Reg,
        src: Operand,
    },
    /// dest = call func(args...)
    Call {
        dest: Option<Reg>,
        func: String,
        args: Vec<Operand>,
    },
    /// dest = load type from addr
    Load {
        dest: Reg,
        addr: Operand,
    },
    /// store src to addr
    Store {
        src: Operand,
        addr: Operand,
    },
    /// dest = alloca type
    Alloca {
        dest: Reg,
        ty: String,
    },
    /// dest = phi [(val1, block1), (val2, block2)]
    Phi {
        dest: Reg,
        incoming: Vec<(Operand, BlockId)>,
    },
}

/// Terminator instructions that end a Basic Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminator {
    /// return value?
    Return(Option<Operand>),
    /// br target
    Branch(BlockId),
    /// br cond, true_target, false_target
    CondBranch {
        cond: Operand,
        true_target: BlockId,
        false_target: BlockId,
    },
    /// Unreachable (e.g., after infinite loop)
    Unreachable,
}

/// Operands for instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    /// Virtual Register (e.g., %1)
    Register(Reg),
    /// Integer Constant
    IntImmediate(i64),
    /// Float Constant
    FloatImmediate(f64),
    /// Boolean Constant
    BoolImmediate(bool),
    /// String Constant (Global ID or raw string)
    StringImmediate(String),
    /// Undefined value
    Undef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOpCode {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, Xor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOpCode {
    Neg, Not,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }
}

impl Function {
    pub fn new(name: &str, params: Vec<String>, return_type: &str) -> Self {
        Function {
            name: name.to_string(),
            params,
            blocks: Vec::new(),
            entry_block: 0,
            return_type: return_type.to_string(),
        }
    }
    
    pub fn add_block(&mut self, block: BasicBlock) -> BlockId {
        let id = self.blocks.len();
        // Ensure the block ID matches its index
        let mut block = block;
        block.id = id;
        self.blocks.push(block);
        id
    }
}

impl BasicBlock {
    pub fn new(id: BlockId, label: &str) -> Self {
        BasicBlock {
            id,
            label: label.to_string(),
            instructions: Vec::new(),
            terminator: Terminator::Unreachable, // Default, must be set later
        }
    }
    
    pub fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
    
    pub fn set_terminator(&mut self, term: Terminator) {
        self.terminator = term;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_structure() {
        let mut module = Module::new("test_module");
        let mut func = Function::new("main", vec![], "Int");
        
        let mut block = BasicBlock::new(0, "entry");
        
        // %1 = add 10, 20
        block.push(Instruction::BinaryOp {
            op: BinaryOpCode::Add,
            dest: 1,
            lhs: Operand::IntImmediate(10),
            rhs: Operand::IntImmediate(20),
        });
        
        // ret %1
        block.set_terminator(Terminator::Return(Some(Operand::Register(1))));
        
        func.add_block(block);
        module.add_function(func);
        
        assert_eq!(module.functions.len(), 1);
        assert_eq!(module.functions[0].blocks.len(), 1);
        
        if let Instruction::BinaryOp { op, .. } = &module.functions[0].blocks[0].instructions[0] {
            assert_eq!(*op, BinaryOpCode::Add);
        } else {
            panic!("Expected BinaryOp");
        }
    }
}
