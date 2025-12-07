//! SSL 3.2 Panic Handler for Freestanding Environments
//! 
//! Provides panic handling infrastructure for bare-metal kernels.

use std::fmt;

/// Panic information for kernel context
#[derive(Debug, Clone)]
pub struct PanicInfo {
    /// Panic message
    pub message: String,
    /// Source file where panic occurred
    pub file: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
}

impl fmt::Display for PanicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KERNEL PANIC: {} at {}:{}:{}",
            self.message, self.file, self.line, self.column
        )
    }
}

impl PanicInfo {
    /// Create a new panic info
    pub fn new(message: impl Into<String>, file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            message: message.into(),
            file: file.into(),
            line,
            column,
        }
    }
}

/// Panic handler action
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanicAction {
    /// Halt the CPU and never return
    Halt,
    /// Reboot the system
    Reboot,
    /// Enter debugger (if available)
    Debug,
    /// Log and continue (not recommended for kernel panics)
    Continue,
}

/// Trait for custom panic handlers
pub trait PanicHandler {
    /// Handle a panic. This function should never return for kernel panics.
    fn handle_panic(&self, info: &PanicInfo) -> PanicAction;
    
    /// Called after handle_panic to perform the action
    fn execute_action(&self, action: PanicAction);
}

/// Default panic handler that halts the CPU
pub struct DefaultPanicHandler;

impl PanicHandler for DefaultPanicHandler {
    fn handle_panic(&self, info: &PanicInfo) -> PanicAction {
        // In a real kernel, this would output to VGA/serial
        eprintln!("{}", info);
        PanicAction::Halt
    }
    
    fn execute_action(&self, action: PanicAction) {
        match action {
            PanicAction::Halt => {
                // Platform-specific halt
                // Will be replaced by SSLA hardware block
            }
            PanicAction::Reboot => {
                // Platform-specific reboot
            }
            PanicAction::Debug => {
                // Breakpoint
            }
            PanicAction::Continue => {
                // Do nothing (dangerous!)
            }
        }
    }
}

/// Generate panic handler function signature for the target platform
pub fn generate_panic_handler_signature(name: &str) -> String {
    format!(
        r#"
@panic_handler
@no_mangle
fn {}(message: String, file: String, line: Int, column: Int) {{
    // Disable interrupts first
    hardware volatile {{
        cpu.interrupt.disable()
    }}
    
    // TODO: Output to VGA/serial
    
    // Halt forever
    loop {{
        hardware volatile {{
            cpu.halt()
        }}
    }}
}}
"#,
        name
    )
}
