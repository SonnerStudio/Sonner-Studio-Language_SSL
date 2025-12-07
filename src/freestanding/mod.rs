//! SSL 3.2 Freestanding Runtime
//! 
//! Minimal runtime for bare-metal environments without OS.
//! Provides hardware abstraction and panic handling for kernel development.

pub mod panic;
pub mod memory;
pub mod io;
pub mod platform;

use crate::ast::{TargetPlatform, HardwareOp, MemorySize};

/// Freestanding mode configuration
#[derive(Debug, Clone)]
pub struct FreestandingConfig {
    /// Target platform
    pub platform: TargetPlatform,
    /// Custom panic handler function name
    pub panic_handler: Option<String>,
    /// Entry point name
    pub entry_point: String,
    /// Stack size in bytes
    pub stack_size: usize,
    /// Enable interrupts at startup
    pub enable_interrupts: bool,
}

impl Default for FreestandingConfig {
    fn default() -> Self {
        Self {
            platform: TargetPlatform::X86_64,
            panic_handler: None,
            entry_point: "_start".to_string(),
            stack_size: 64 * 1024, // 64 KB default
            enable_interrupts: false,
        }
    }
}

impl FreestandingConfig {
    /// Create a new configuration for the specified platform
    pub fn new(platform: TargetPlatform) -> Self {
        Self {
            platform,
            ..Default::default()
        }
    }
    
    /// Set custom panic handler
    pub fn with_panic_handler(mut self, name: impl Into<String>) -> Self {
        self.panic_handler = Some(name.into());
        self
    }
    
    /// Set entry point name
    pub fn with_entry_point(mut self, name: impl Into<String>) -> Self {
        self.entry_point = name.into();
        self
    }
    
    /// Set stack size
    pub fn with_stack_size(mut self, size: usize) -> Self {
        self.stack_size = size;
        self
    }
}

/// Generate platform-specific assembly for a hardware operation
pub fn generate_asm_for_op(op: &HardwareOp, platform: &TargetPlatform) -> String {
    match platform {
        TargetPlatform::X86_64 | TargetPlatform::AppleIntel | TargetPlatform::SteamDeck => {
            generate_x86_64_asm(op)
        }
        TargetPlatform::ARM64 | TargetPlatform::AppleSilicon => {
            generate_arm64_asm(op)
        }
        TargetPlatform::All => {
            // Generate portable version or error
            "// Platform-independent operation".to_string()
        }
    }
}

/// Generate x86-64 assembly for a hardware operation
fn generate_x86_64_asm(op: &HardwareOp) -> String {
    match op {
        HardwareOp::DisableInterrupts => "cli".to_string(),
        HardwareOp::EnableInterrupts => "sti".to_string(),
        HardwareOp::Halt => "hlt".to_string(),
        HardwareOp::Nop => "nop".to_string(),
        HardwareOp::MemoryFence => "mfence".to_string(),
        
        HardwareOp::PortRead { dest: _, port: _, size } => {
            match size {
                MemorySize::Byte => "in al, dx".to_string(),
                MemorySize::Word => "in ax, dx".to_string(),
                MemorySize::DWord => "in eax, dx".to_string(),
                MemorySize::QWord => "in rax, dx".to_string(),
            }
        }
        
        HardwareOp::PortWrite { port: _, value: _, size } => {
            match size {
                MemorySize::Byte => "out dx, al".to_string(),
                MemorySize::Word => "out dx, ax".to_string(),
                MemorySize::DWord => "out dx, eax".to_string(),
                MemorySize::QWord => "out dx, rax".to_string(),
            }
        }
        
        HardwareOp::MemoryRead { dest: _, address: _, size } => {
            match size {
                MemorySize::Byte => "mov al, byte ptr [{addr}]".to_string(),
                MemorySize::Word => "mov ax, word ptr [{addr}]".to_string(),
                MemorySize::DWord => "mov eax, dword ptr [{addr}]".to_string(),
                MemorySize::QWord => "mov rax, qword ptr [{addr}]".to_string(),
            }
        }
        
        HardwareOp::MemoryWrite { address: _, value: _, size } => {
            match size {
                MemorySize::Byte => "mov byte ptr [{addr}], al".to_string(),
                MemorySize::Word => "mov word ptr [{addr}], ax".to_string(),
                MemorySize::DWord => "mov dword ptr [{addr}], eax".to_string(),
                MemorySize::QWord => "mov qword ptr [{addr}], rax".to_string(),
            }
        }
        
        HardwareOp::RegisterRead { dest: _, source: _ } => {
            "mov {dest}, {source}".to_string()
        }
        
        HardwareOp::RegisterWrite { register, value: _ } => {
            format!("mov {}, {{value}}", register)
        }
        
        HardwareOp::RawAsm { template, .. } => {
            template.clone()
        }
    }
}

/// Generate ARM64 assembly for a hardware operation
fn generate_arm64_asm(op: &HardwareOp) -> String {
    match op {
        HardwareOp::DisableInterrupts => "msr daifset, #2".to_string(),
        HardwareOp::EnableInterrupts => "msr daifclr, #2".to_string(),
        HardwareOp::Halt => "wfi".to_string(),
        HardwareOp::Nop => "nop".to_string(),
        HardwareOp::MemoryFence => "dsb sy".to_string(),
        
        HardwareOp::MemoryRead { dest: _, address: _, size } => {
            match size {
                MemorySize::Byte => "ldrb w0, [{addr}]".to_string(),
                MemorySize::Word => "ldrh w0, [{addr}]".to_string(),
                MemorySize::DWord => "ldr w0, [{addr}]".to_string(),
                MemorySize::QWord => "ldr x0, [{addr}]".to_string(),
            }
        }
        
        HardwareOp::MemoryWrite { address: _, value: _, size } => {
            match size {
                MemorySize::Byte => "strb w0, [{addr}]".to_string(),
                MemorySize::Word => "strh w0, [{addr}]".to_string(),
                MemorySize::DWord => "str w0, [{addr}]".to_string(),
                MemorySize::QWord => "str x0, [{addr}]".to_string(),
            }
        }
        
        HardwareOp::RegisterRead { dest: _, source: _ } => {
            "mov {dest}, {source}".to_string()
        }
        
        HardwareOp::RegisterWrite { register, value: _ } => {
            format!("mov {}, {{value}}", register)
        }
        
        // ARM64 doesn't have I/O ports - use MMIO instead
        HardwareOp::PortRead { .. } | HardwareOp::PortWrite { .. } => {
            "// ARM64: Use memory-mapped I/O".to_string()
        }
        
        HardwareOp::RawAsm { template, .. } => {
            template.clone()
        }
    }
}
