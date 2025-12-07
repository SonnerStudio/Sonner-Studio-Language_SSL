//! SSL 3.2 Platform Definitions for Freestanding Environments
//! 
//! Provides platform-specific configuration and code generation.

use crate::ast::TargetPlatform;

/// Platform-specific configuration
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    /// Target platform
    pub platform: TargetPlatform,
    /// Default kernel load address
    pub kernel_load_address: u64,
    /// Default stack size
    pub stack_size: usize,
    /// Page size
    pub page_size: usize,
    /// Whether the platform uses UEFI
    pub uses_uefi: bool,
    /// Endianness
    pub little_endian: bool,
}

impl PlatformConfig {
    /// Get configuration for a platform
    pub fn for_platform(platform: TargetPlatform) -> Self {
        match platform {
            TargetPlatform::X86_64 => Self {
                platform,
                kernel_load_address: 0x100000, // 1 MB
                stack_size: 64 * 1024,         // 64 KB
                page_size: 4096,               // 4 KB
                uses_uefi: false,
                little_endian: true,
            },
            TargetPlatform::ARM64 => Self {
                platform,
                kernel_load_address: 0x200000, // 2 MB
                stack_size: 64 * 1024,
                page_size: 4096,
                uses_uefi: true,
                little_endian: true,
            },
            TargetPlatform::AppleIntel => Self {
                platform,
                kernel_load_address: 0x100000,
                stack_size: 128 * 1024, // Larger stack for macOS-style
                page_size: 4096,
                uses_uefi: true,
                little_endian: true,
            },
            TargetPlatform::AppleSilicon => Self {
                platform,
                kernel_load_address: 0x800000000, // 32 GB offset (Apple's memory map)
                stack_size: 128 * 1024,
                page_size: 16384,                 // 16 KB pages on Apple Silicon
                uses_uefi: false,                 // Uses iBoot
                little_endian: true,
            },
            TargetPlatform::SteamDeck => Self {
                platform,
                kernel_load_address: 0x100000,
                stack_size: 256 * 1024, // Larger stack for gaming
                page_size: 4096,
                uses_uefi: true,
                little_endian: true,
            },
            TargetPlatform::All => Self {
                platform,
                kernel_load_address: 0x100000,
                stack_size: 64 * 1024,
                page_size: 4096,
                uses_uefi: false,
                little_endian: true,
            },
        }
    }
    
    /// Get the LLVM target triple
    pub fn target_triple(&self) -> &'static str {
        self.platform.triple()
    }
    
    /// Get the linker script path
    pub fn linker_script(&self) -> &'static str {
        self.platform.linker_script()
    }
    
    /// Generate boot code preamble
    pub fn boot_preamble(&self) -> String {
        match self.platform {
            TargetPlatform::X86_64 | TargetPlatform::SteamDeck => self.x86_64_boot_preamble(),
            TargetPlatform::ARM64 => self.arm64_boot_preamble(),
            TargetPlatform::AppleIntel => self.apple_intel_boot_preamble(),
            TargetPlatform::AppleSilicon => self.apple_silicon_boot_preamble(),
            TargetPlatform::All => "// Platform-independent boot".to_string(),
        }
    }
    
    fn x86_64_boot_preamble(&self) -> String {
        format!(r#"
// x86-64 Boot Preamble
// SSL 3.2 Generated Code for ZetaCore

@freestanding
@target(x86_64)

// Multiboot2 header (for GRUB)
@link_section(".multiboot2")
let MULTIBOOT2_MAGIC: Int = 0xE85250D6
let MULTIBOOT2_ARCH: Int = 0  // i386/x86_64

@entry
@no_mangle
@link_section(".text.boot")
fn _start() {{
    // Disable interrupts immediately
    hardware volatile {{
        cpu.interrupt.disable()
    }}
    
    // Set up stack
    hardware volatile {{
        cpu.register.rsp = __stack_top
    }}
    
    // Zero BSS
    let bss_start = __bss_start
    let bss_end = __bss_end
    for addr in bss_start..bss_end {{
        hardware volatile {{
            memory[addr] = 0
        }}
    }}
    
    // Call kernel main
    kernel_main()
    
    // Should never return
    loop {{
        hardware volatile {{
            cpu.halt()
        }}
    }}
}}
"#)
    }
    
    fn arm64_boot_preamble(&self) -> String {
        format!(r#"
// ARM64 Boot Preamble
// SSL 3.2 Generated Code for ZetaCore

@freestanding
@target(arm64)

@entry
@no_mangle
@link_section(".text.boot")
fn _start() {{
    // Disable interrupts (mask all)
    hardware volatile {{
        cpu.interrupt.disable()
    }}
    
    // Get CPU ID (only CPU 0 continues)
    let cpu_id = 0  // TODO: Read from MPIDR_EL1
    if cpu_id != 0 {{
        loop {{
            hardware volatile {{
                cpu.halt()  // WFI on ARM
            }}
        }}
    }}
    
    // Set up stack
    hardware volatile {{
        cpu.register.sp = __stack_top
    }}
    
    // Zero BSS
    let bss_start = __bss_start
    let bss_end = __bss_end
    for addr in bss_start..bss_end {{
        hardware volatile {{
            memory[addr] = 0
        }}
    }}
    
    // Call kernel main
    kernel_main()
    
    // Should never return
    loop {{
        hardware volatile {{
            cpu.halt()
        }}
    }}
}}
"#)
    }
    
    fn apple_intel_boot_preamble(&self) -> String {
        format!(r#"
// Apple Intel Boot Preamble
// SSL 3.2 Generated Code for ZetaCore

@freestanding
@target(apple_intel)

// Note: Requires custom bootloader for Apple hardware
// Standard EFI boot path

@entry
@no_mangle
fn _start() {{
    hardware volatile {{
        cpu.interrupt.disable()
        cpu.register.rsp = __stack_top
    }}
    
    kernel_main()
    
    loop {{
        hardware volatile {{
            cpu.halt()
        }}
    }}
}}
"#)
    }
    
    fn apple_silicon_boot_preamble(&self) -> String {
        format!(r#"
// Apple Silicon Boot Preamble
// SSL 3.2 Generated Code for ZetaCore

@freestanding
@target(apple_silicon)

// Note: Apple Silicon requires special bootchain (m1n1, etc.)
// Memory starts at 0x800000000

@entry
@no_mangle
fn _start() {{
    // Disable interrupts
    hardware volatile {{
        cpu.interrupt.disable()
    }}
    
    // Apple Silicon has 16KB pages
    hardware volatile {{
        cpu.register.sp = __stack_top
    }}
    
    // Initialize MMU for 16KB pages
    // TODO: Apple-specific MMU setup
    
    kernel_main()
    
    loop {{
        hardware volatile {{
            cpu.halt()
        }}
    }}
}}
"#)
    }
}

/// CPU register names per platform
pub mod registers {
    /// x86-64 general purpose registers
    pub const X86_64_GP: &[&str] = &[
        "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
        "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
    ];
    
    /// x86-64 segment registers
    pub const X86_64_SEG: &[&str] = &["cs", "ds", "es", "fs", "gs", "ss"];
    
    /// x86-64 control registers
    pub const X86_64_CR: &[&str] = &["cr0", "cr2", "cr3", "cr4"];
    
    /// ARM64 general purpose registers
    pub const ARM64_GP: &[&str] = &[
        "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7",
        "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15",
        "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23",
        "x24", "x25", "x26", "x27", "x28", "x29", "x30", "sp",
    ];
    
    /// ARM64 system registers commonly used
    pub const ARM64_SYS: &[&str] = &[
        "sctlr_el1", "tcr_el1", "mair_el1", "ttbr0_el1", "ttbr1_el1",
        "spsr_el1", "elr_el1", "vbar_el1", "daif",
    ];
}

/// CPU feature flags
#[derive(Debug, Clone, Default)]
pub struct CpuFeatures {
    /// SSE support (x86)
    pub sse: bool,
    /// SSE2 support (x86)
    pub sse2: bool,
    /// AVX support (x86)
    pub avx: bool,
    /// AVX-512 support (x86)
    pub avx512: bool,
    /// NEON support (ARM)
    pub neon: bool,
    /// SVE support (ARM)
    pub sve: bool,
    /// x2APIC support (x86)
    pub x2apic: bool,
    /// 5-level paging (x86)
    pub la57: bool,
}
