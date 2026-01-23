// Abstract Syntax Tree definitions for SSL

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    FunctionDecl(FunctionDecl),
    VariableDecl(VariableDecl),
    Assignment(Assignment),
    ExpressionStmt(Expression),
    If(IfStatement),
    For(ForLoop),
    While(WhileLoop),
    Return(Option<Expression>),
    Spawn(Vec<Statement>),
    SpawnOn {
        address: String,
        block: Vec<Statement>,
    },
    TryRecover {
        try_block: Vec<Statement>,
        error_var: String,
        recover_block: Vec<Statement>,
    },
    Import {
        path: Vec<String>,
        alias: Option<String>,
    },
    UnsafeBlock(Vec<Statement>),
    NaturalBlock(String),
    VisualBlock(String),
    TraitDecl(TraitDecl),
    ImplTrait(ImplTrait),
    TypeAlias(TypeAlias),
    EnumDecl(EnumDecl),
    // SSL 3.0: Index-based assignment (map[key] = value, list[i] = value)
    IndexAssignment {
        target: Expression,
        index: Expression,
        value: Expression,
    },
    // SSL 3.2: Module-level attribute
    ModuleDecl {
        attributes: Vec<ModuleAttribute>,
        name: Option<String>,
    },
    // SSL 3.2: SSLA Hardware Block (Revolutionary Semantic Assembly)
    HardwareBlock(HardwareBlock),
    // SSL 3.2: Platform-specific block
    PlatformBlock {
        target: TargetPlatform,
        body: Vec<Statement>,
    },
    // SSL v9: Native HTML Block
    HtmlBlock(String),
    // SSL v9: Native Window Declaration
    WindowDecl(WindowDecl),
    // SSL v9: 3D Material Declaration
    MaterialDecl(MaterialDecl),
    // Dummy statement for empty semicolons
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowDecl {
    pub title: String,
    pub width: i64,
    pub height: i64,
    pub transparent: bool,
    pub decorations: bool,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialDecl {
    pub name: String,
    pub diffuse: String,
    pub normal: Option<String>,
    pub roughness: Option<String>,
}

// ============================================================================
// SSL 3.2: Freestanding Environment Types
// ============================================================================

/// Module-level attributes for freestanding environments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleAttribute {
    /// @freestanding - No OS dependencies, bare-metal mode
    Freestanding,
    /// @no_std - No standard library, only core functionality
    NoStd,
    /// @target(...) - Target platform specification
    Target(TargetPlatform),
}

/// Function-level attributes for kernel code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionAttribute {
    /// @panic_handler - Custom panic handler
    PanicHandler,
    /// @no_mangle - Exact symbol name (no name mangling)
    NoMangle,
    /// @link_section(".text.boot") - Place in specific linker section
    LinkSection(String),
    /// @naked - No function prologue/epilogue
    Naked,
    /// @interrupt - Interrupt handler ABI
    Interrupt,
    /// @entry - Kernel entry point
    Entry,
}

/// Target platforms for multi-architecture support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetPlatform {
    /// Standard x86-64 PC
    X86_64,
    /// Generic ARM64 / AArch64
    ARM64,
    /// Apple Intel Macs
    AppleIntel,
    /// Apple Silicon M1/M2/M3/M4
    AppleSilicon,
    /// Steam Deck AMD APU
    SteamDeck,
    /// All platforms (architecture-independent)
    All,
}

impl TargetPlatform {
    /// Get the LLVM target triple for this platform
    pub fn triple(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64-unknown-none",
            Self::ARM64 => "aarch64-unknown-none",
            Self::AppleIntel => "x86_64-apple-none-macho",
            Self::AppleSilicon => "aarch64-apple-none-macho",
            Self::SteamDeck => "x86_64-unknown-none",
            Self::All => "generic",
        }
    }
    
    /// Get the path to the linker script for this platform
    pub fn linker_script(&self) -> &'static str {
        match self {
            Self::X86_64 => "linker/x86_64.ld",
            Self::ARM64 => "linker/arm64.ld",
            Self::AppleIntel => "linker/apple_intel.ld",
            Self::AppleSilicon => "linker/apple_silicon.ld",
            Self::SteamDeck => "linker/steam_deck.ld",
            Self::All => "linker/generic.ld",
        }
    }
}

/// SSLA (SSL Assembly) Hardware Block
/// Revolutionary semantic assembly syntax for bare-metal programming
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardwareBlock {
    /// Is this block volatile (prevents optimization)?
    pub is_volatile: bool,
    /// Hardware operations in this block
    pub operations: Vec<HardwareOp>,
}

/// SSLA Hardware Operations - Semantic alternatives to cryptic assembly
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HardwareOp {
    // ===== Register Operations =====
    /// Read from CPU register: register result = cpu.read(value)
    RegisterRead {
        dest: String,
        source: Expression,
    },
    /// Write to CPU register: cpu.write(register, value)
    RegisterWrite {
        register: String,
        value: Expression,
    },
    
    // ===== Memory Operations =====
    /// Read from memory address: memory[addr]
    MemoryRead {
        dest: String,
        address: Expression,
        size: MemorySize,
    },
    /// Write to memory address: memory[addr] = value
    MemoryWrite {
        address: Expression,
        value: Expression,
        size: MemorySize,
    },
    
    // ===== I/O Port Operations (x86) =====
    /// Read from I/O port: port.read(port_num)
    PortRead {
        dest: String,
        port: Expression,
        size: MemorySize,
    },
    /// Write to I/O port: port.write(port_num, value)
    PortWrite {
        port: Expression,
        value: Expression,
        size: MemorySize,
    },
    
    // ===== CPU Control =====
    /// Disable interrupts: cpu.interrupt.disable()
    DisableInterrupts,
    /// Enable interrupts: cpu.interrupt.enable()
    EnableInterrupts,
    /// Halt CPU until interrupt: cpu.halt()
    Halt,
    /// No operation: cpu.nop()
    Nop,
    /// Memory fence: cpu.fence()
    MemoryFence,
    
    // ===== Raw Assembly (fallback) =====
    /// Raw inline assembly for platform-specific code
    RawAsm {
        template: String,
        outputs: Vec<AsmOperand>,
        inputs: Vec<AsmOperand>,
        clobbers: Vec<String>,
        is_intel_syntax: bool,
    },
}

/// Memory access size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemorySize {
    Byte,       // 8-bit
    Word,       // 16-bit
    DWord,      // 32-bit
    QWord,      // 64-bit (default)
}

/// Assembly operand for raw asm blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsmOperand {
    /// Constraint: "r", "=r", "m", etc.
    pub constraint: String,
    /// SSL expression
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitDecl {
    pub name: String,
    pub methods: Vec<TraitMethod>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitMethod {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplTrait {
    pub trait_name: String,
    pub for_type: String,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub target: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
    pub is_async: bool,
    // SSL 3.2: Function attributes for freestanding/kernel code
    #[serde(default)]
    pub attributes: Vec<FunctionAttribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDecl {
    pub name: String,
    pub mutable: bool,
    pub var_type: Option<Type>,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoop {
    pub var: String,
    pub iterable: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    FunctionCall(FunctionCall),
    BinaryOp(BinaryOp),
    ListLiteral(Vec<Expression>),
    MapLiteral(Vec<(String, Expression)>),
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
    Match {
        value: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    Await(Box<Expression>),
    EnumConstruction {
        enum_name: String,
        variant_name: String,
        args: Vec<Expression>,
    },
    // SSL 3.0: Pipe operator (value |> function)
    Pipe {
        value: Box<Expression>,
        function: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Variable(String),
    Constructor {
        name: String,
        args: Vec<Pattern>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub type_args: Vec<Type>,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    Lt,
    Le,
    Gt,
    Ge,
    Range,
    // SSL 3.0: Functional composition
    ComposeRight, // >>
    ComposeLeft,  // <<
    And,          // &&
    Or,           // ||
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Any,
    List(Box<Type>),
    Custom(String),
    Generic {
        base: String,
        args: Vec<Type>,
    },
}
