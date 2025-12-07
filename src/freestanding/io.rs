//! SSL 3.2 I/O Operations for Freestanding Environments
//! 
//! Provides abstractions for port I/O (x86) and memory-mapped I/O (ARM).

use crate::ast::MemorySize;

/// I/O Port (x86 specific)
#[derive(Debug, Clone)]
pub struct IoPort {
    /// Port number
    pub port: u16,
    /// Access size
    pub size: MemorySize,
}

impl IoPort {
    /// Create a new I/O port
    pub fn new(port: u16) -> Self {
        Self {
            port,
            size: MemorySize::Byte,
        }
    }
    
    /// Create with specific size
    pub fn with_size(port: u16, size: MemorySize) -> Self {
        Self { port, size }
    }
    
    /// Generate SSLA code for reading from this port
    pub fn read_code(&self, dest: &str) -> String {
        format!("let {} = port.read(0x{:X})", dest, self.port)
    }
    
    /// Generate SSLA code for writing to this port
    pub fn write_code(&self, value: &str) -> String {
        format!("port.write(0x{:X}, {})", self.port, value)
    }
}

/// Common x86 I/O ports
pub mod ports {
    use super::IoPort;
    
    /// PIC1 (Master) command port
    pub const PIC1_COMMAND: IoPort = IoPort { port: 0x20, size: super::MemorySize::Byte };
    /// PIC1 (Master) data port
    pub const PIC1_DATA: IoPort = IoPort { port: 0x21, size: super::MemorySize::Byte };
    /// PIC2 (Slave) command port
    pub const PIC2_COMMAND: IoPort = IoPort { port: 0xA0, size: super::MemorySize::Byte };
    /// PIC2 (Slave) data port
    pub const PIC2_DATA: IoPort = IoPort { port: 0xA1, size: super::MemorySize::Byte };
    
    /// PS/2 keyboard/mouse data port
    pub const PS2_DATA: IoPort = IoPort { port: 0x60, size: super::MemorySize::Byte };
    /// PS/2 keyboard/mouse status/command port
    pub const PS2_STATUS: IoPort = IoPort { port: 0x64, size: super::MemorySize::Byte };
    
    /// COM1 serial port base
    pub const COM1: IoPort = IoPort { port: 0x3F8, size: super::MemorySize::Byte };
    /// COM2 serial port base
    pub const COM2: IoPort = IoPort { port: 0x2F8, size: super::MemorySize::Byte };
    
    /// VGA CRTC address register
    pub const VGA_CRTC_ADDR: IoPort = IoPort { port: 0x3D4, size: super::MemorySize::Byte };
    /// VGA CRTC data register
    pub const VGA_CRTC_DATA: IoPort = IoPort { port: 0x3D5, size: super::MemorySize::Byte };
    
    /// PIT (Programmable Interval Timer) channel 0
    pub const PIT_CH0: IoPort = IoPort { port: 0x40, size: super::MemorySize::Byte };
    /// PIT command register
    pub const PIT_COMMAND: IoPort = IoPort { port: 0x43, size: super::MemorySize::Byte };
    
    /// RTC address port
    pub const RTC_ADDRESS: IoPort = IoPort { port: 0x70, size: super::MemorySize::Byte };
    /// RTC data port
    pub const RTC_DATA: IoPort = IoPort { port: 0x71, size: super::MemorySize::Byte };
}

/// Serial port configuration
#[derive(Debug, Clone)]
pub struct SerialConfig {
    /// Base port address
    pub base_port: u16,
    /// Baud rate divisor
    pub baud_divisor: u16,
    /// Data bits (5-8)
    pub data_bits: u8,
    /// Stop bits (1-2)
    pub stop_bits: u8,
    /// Parity (None, Odd, Even)
    pub parity: Parity,
}

/// Serial parity modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            base_port: 0x3F8, // COM1
            baud_divisor: 3,  // 38400 baud
            data_bits: 8,
            stop_bits: 1,
            parity: Parity::None,
        }
    }
}

impl SerialConfig {
    /// Generate SSLA code for serial port initialization
    pub fn init_code(&self) -> String {
        format!(r#"
// Initialize serial port at 0x{:X}
hardware volatile {{
    // Disable interrupts
    port.write(0x{:X}, 0x00)
    
    // Enable DLAB
    port.write(0x{:X}, 0x80)
    
    // Set baud rate divisor
    port.write(0x{:X}, 0x{:02X})  // Low byte
    port.write(0x{:X}, 0x{:02X})  // High byte
    
    // Set data format: {} data bits, {} stop bit, {:?} parity
    port.write(0x{:X}, 0x{:02X})
    
    // Enable FIFO
    port.write(0x{:X}, 0xC7)
    
    // Enable IRQs, RTS/DSR set
    port.write(0x{:X}, 0x0B)
}}
"#,
            self.base_port,
            self.base_port + 1,  // IER
            self.base_port + 3,  // LCR
            self.base_port,      // DLAB low
            self.baud_divisor & 0xFF,
            self.base_port + 1,  // DLAB high
            (self.baud_divisor >> 8) & 0xFF,
            self.data_bits,
            self.stop_bits,
            self.parity,
            self.base_port + 3,  // LCR
            self.line_control_byte(),
            self.base_port + 2,  // FCR
            self.base_port + 4,  // MCR
        )
    }
    
    /// Calculate line control register value
    fn line_control_byte(&self) -> u8 {
        let mut lcr = 0u8;
        
        // Data bits (bits 0-1)
        lcr |= (self.data_bits - 5) & 0x03;
        
        // Stop bits (bit 2)
        if self.stop_bits == 2 {
            lcr |= 0x04;
        }
        
        // Parity (bits 3-5)
        match self.parity {
            Parity::None => {}
            Parity::Odd => lcr |= 0x08,
            Parity::Even => lcr |= 0x18,
        }
        
        lcr
    }
}

/// Generate serial write function
pub fn generate_serial_write_code(port: u16) -> String {
    format!(r#"
// Write a byte to serial port
fn serial_write(byte: Int) {{
    // Wait for transmit buffer to be empty
    while (port.read(0x{:X}) & 0x20) == 0 {{
        cpu.nop()
    }}
    
    // Write byte
    port.write(0x{:X}, byte)
}}
"#, port + 5, port)
}

/// Generate serial read function
pub fn generate_serial_read_code(port: u16) -> String {
    format!(r#"
// Read a byte from serial port
fn serial_read() -> Int {{
    // Wait for data to be available
    while (port.read(0x{:X}) & 0x01) == 0 {{
        cpu.nop()
    }}
    
    // Read byte
    return port.read(0x{:X})
}}
"#, port + 5, port)
}
