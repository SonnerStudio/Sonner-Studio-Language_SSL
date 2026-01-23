use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r\f]+")] // Ignore whitespace
#[logos(skip r"//[^\n]*")] // Ignore comments
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("use")]
    Use,
    #[token("natural")]
    Natural,
    #[token("spawn")]
    Spawn,
    #[token("try")]
    Try,
    #[token("recover")]
    Recover,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("unsafe")]
    Unsafe,
    #[token("match")]
    Match,
    #[token("trait")]
    Trait,
    #[token("impl")]
    Impl,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("visual")]
    Visual,
    
    #[token("html")]
    Html,

    // SSL v9: Native Window
    #[token("window")]
    Window,

    // SSL v9: 3D Material
    #[token("material")]
    Material,

    // SSL 3.2: Freestanding / Bare-Metal Keywords
    #[token("hardware")]
    Hardware,           // Revolutionary SSLA block (replaces cryptic 'asm')
    
    #[token("volatile")]
    Volatile,           // Volatile operations
    
    #[token("register")]
    Register,           // CPU register access
    
    #[token("memory")]
    Memory,             // Direct memory access
    
    #[token("port")]
    Port,               // I/O port access (x86)
    
    #[token("cpu")]
    Cpu,                // CPU control (interrupts, halt, etc.)
    
    #[token("platform")]
    Platform,           // Platform-specific block
    
    // SSL 3.2: Module & Function Attributes
    #[token("@freestanding")]
    AtFreestanding,     // No OS dependencies
    
    #[token("@no_std")]
    AtNoStd,            // No standard library
    
    #[token("@panic_handler")]
    AtPanicHandler,     // Custom panic handler
    
    #[token("@no_mangle")]
    AtNoMangle,         // Exact symbol name
    
    #[token("@link_section")]
    AtLinkSection,      // Custom linker section
    
    #[token("@naked")]
    AtNaked,            // No function prolog/epilog
    
    #[token("@interrupt")]
    AtInterrupt,        // Interrupt handler ABI
    
    #[token("@entry")]
    AtEntry,            // Kernel entry point
    
    #[token("@target")]
    AtTarget,           // Target platform specification

    // Brackets & Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(",")]
    Comma,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("_")]
    Underscore,
    #[token("@")]
    At,
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[token("#")]
    Hash,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,

    // Operators
    #[token("=")]
    Assign,
    #[token("==")]
    Equals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("..")]
    Range,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("!=")]
    NotEquals,
    
    // SSL 3.0: Functional programming operators
    #[token("|>")]
    PipeRight,
    
    #[token("<|")]
    PipeLeft,
    
    #[token(">>")]
    ComposeRight,
    
    #[token("<<")]
    ComposeLeft,
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,


    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    FloatLiteral(f64),

    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    
    // SSL 3.2: Hexadecimal literals for hardware addresses
    #[regex("0x[0-9a-fA-F]+", |lex| {
        i64::from_str_radix(&lex.slice()[2..], 16).ok()
    })]
    HexInteger(i64),
    
    // SSL 3.2: Binary literals for bit manipulation
    #[regex("0b[01]+", |lex| {
        i64::from_str_radix(&lex.slice()[2..], 2).ok()
    })]
    BinaryInteger(i64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    StringLiteral(String),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLiteral(bool),

    // Natural Language Block Content (Simplified for now)
    // In a real implementation, the parser would switch modes.
}
