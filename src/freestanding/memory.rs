//! SSL 3.2 Memory Operations for Freestanding Environments
//! 
//! Provides safe abstractions for volatile memory access and memory management.

use crate::ast::MemorySize;

/// Volatile memory access - prevents compiler optimization
#[derive(Debug, Clone)]
pub struct VolatilePtr {
    /// Memory address
    pub address: u64,
    /// Access size
    pub size: MemorySize,
}

impl VolatilePtr {
    /// Create a new volatile pointer
    pub fn new(address: u64) -> Self {
        Self {
            address,
            size: MemorySize::QWord,
        }
    }
    
    /// Create with specific size
    pub fn with_size(address: u64, size: MemorySize) -> Self {
        Self { address, size }
    }
    
    /// Generate SSLA code for reading this address
    pub fn read_code(&self, dest: &str) -> String {
        let size_suffix = match self.size {
            MemorySize::Byte => ".byte",
            MemorySize::Word => ".word",
            MemorySize::DWord => ".dword",
            MemorySize::QWord => "",
        };
        format!("let {} = memory{}[0x{:X}]", dest, size_suffix, self.address)
    }
    
    /// Generate SSLA code for writing to this address
    pub fn write_code(&self, value: &str) -> String {
        let size_suffix = match self.size {
            MemorySize::Byte => ".byte",
            MemorySize::Word => ".word",
            MemorySize::DWord => ".dword",
            MemorySize::QWord => "",
        };
        format!("memory{}[0x{:X}] = {}", size_suffix, self.address, value)
    }
}

/// Common hardware addresses
pub mod addresses {
    /// VGA text buffer start (x86)
    pub const VGA_BUFFER: u64 = 0xB8000;
    /// VGA text buffer end
    pub const VGA_BUFFER_END: u64 = 0xB8FA0;
    /// VGA buffer size (80x25 characters, 2 bytes each)
    pub const VGA_BUFFER_SIZE: usize = 80 * 25 * 2;
    
    /// BIOS data area (x86)
    pub const BIOS_DATA_AREA: u64 = 0x400;
    
    /// ACPI RSDP search start
    pub const ACPI_SEARCH_START: u64 = 0x000E0000;
    /// ACPI RSDP search end
    pub const ACPI_SEARCH_END: u64 = 0x00100000;
    
    /// Local APIC default address
    pub const LOCAL_APIC: u64 = 0xFEE00000;
    
    /// I/O APIC default address
    pub const IO_APIC: u64 = 0xFEC00000;
}

/// Memory region descriptor
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Start address
    pub start: u64,
    /// End address (exclusive)
    pub end: u64,
    /// Region type
    pub region_type: MemoryRegionType,
}

/// Memory region types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryRegionType {
    /// Usable RAM
    Usable,
    /// Reserved by firmware
    Reserved,
    /// ACPI reclaimable
    AcpiReclaimable,
    /// ACPI NVS
    AcpiNvs,
    /// Bad memory
    BadMemory,
    /// Kernel code/data
    Kernel,
    /// Stack
    Stack,
    /// Heap
    Heap,
    /// Memory-mapped I/O
    Mmio,
}

impl MemoryRegion {
    /// Create a new memory region
    pub fn new(start: u64, end: u64, region_type: MemoryRegionType) -> Self {
        Self { start, end, region_type }
    }
    
    /// Get region size in bytes
    pub fn size(&self) -> u64 {
        self.end - self.start
    }
    
    /// Check if an address is within this region
    pub fn contains(&self, address: u64) -> bool {
        address >= self.start && address < self.end
    }
}

/// Generate BSS zero initialization code
pub fn generate_bss_init_code() -> String {
    r#"
// Zero out BSS section
hardware volatile {
    let start = __bss_start
    let end = __bss_end
    
    for addr in start..end {
        memory[addr] = 0
    }
}
"#.to_string()
}

/// Generate stack setup code for a platform
pub fn generate_stack_setup_code(stack_top_symbol: &str) -> String {
    format!(r#"
// Set up stack pointer
hardware volatile {{
    cpu.register.sp = {}
}}
"#, stack_top_symbol)
}
