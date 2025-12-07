// Aurora Compiler Module
// Handles JIT/AOT compilation via Multi-Stage IR

pub mod ir;
pub mod builder;
pub mod compiler;
pub mod llvm;
pub mod jit;
pub mod optimizer;  // SSL 3.0: Optimization passes
pub mod native;     // SSL 3.0: Native code execution

#[cfg(feature = "llvm")]
pub mod llvm_backend;  // SSL 3.0: Real LLVM JIT backend (inkwell)

pub use ir::*;
pub use builder::*;
pub use compiler::*;
pub use llvm::*;
pub use jit::*;
pub use optimizer::*;  // SSL 3.0
pub use native::*;     // SSL 3.0

#[cfg(feature = "llvm")]
pub use llvm_backend::LLVMBackend;  // SSL 3.0

