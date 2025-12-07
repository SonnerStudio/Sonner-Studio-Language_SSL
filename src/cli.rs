use clap::{Parser, Subcommand, ValueEnum};

/// SSL 5.0 Build Target
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum BuildTarget {
    /// Native binary (default)
    Native,
    /// WebAssembly (.wasm) for browsers
    Wasm,
    /// WebAssembly with JavaScript bindings
    WasmJs,
    /// iOS (requires Xcode)
    Ios,
    /// Android (requires NDK)
    Android,
    /// Edge/Serverless deployment
    Edge,
}

impl Default for BuildTarget {
    fn default() -> Self {
        Self::Native
    }
}

/// Edge deployment provider
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum EdgeProvider {
    /// Cloudflare Workers
    Cloudflare,
    /// Vercel Edge Functions
    Vercel,
    /// AWS Lambda@Edge
    Aws,
    /// Deno Deploy
    Deno,
    /// Fastly Compute
    Fastly,
}

#[derive(Parser)]
#[command(name = "ssl")]
#[command(about = "Sonner Studio Language Toolchain v5.0", long_about = None)]
#[command(version = "5.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a SSL source file
    Run {
        /// The source file to run
        file: Option<String>,
        /// Enable Time-Travel Debugging
        #[arg(long, short)]
        debug: bool,
        /// Enable Hot Reload (Live Programming)
        #[arg(long, short)]
        watch: bool,
        /// Run AI Code Review before execution
        #[arg(long)]
        ai_review: bool,
    },
    /// Build a SSL project
    Build {
        /// The source file or project to build
        path: Option<String>,
        /// Build target (native, wasm, wasm-js, ios, android, edge)
        #[arg(long, short, default_value = "native")]
        target: BuildTarget,
        /// Enable release optimizations
        #[arg(long, short)]
        release: bool,
        /// Output directory
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Check syntax and types without running
    Check {
        file: String,
    },
    /// Check system environment
    Doctor,
    /// Ask the AI Assistant
    Ai {
        /// The question or instruction
        query: Vec<String>,
    },
    /// Start the Language Server
    Lsp,
    /// Package manager commands
    Pkg {
        #[command(subcommand)]
        action: PkgCommands,
    },
    /// Property-based testing
    Test {
        #[command(subcommand)]
        action: TestCommands,
    },
    /// Deploy to edge/serverless
    Deploy {
        /// Source file or project
        path: Option<String>,
        /// Edge provider
        #[arg(long, short, default_value = "cloudflare")]
        provider: EdgeProvider,
        /// Dry run (don't actually deploy)
        #[arg(long)]
        dry_run: bool,
    },
    /// CRDT distributed data operations
    Crdt {
        #[command(subcommand)]
        action: CrdtCommands,
    },
    /// Formal verification
    Verify {
        /// Source file to verify
        file: String,
        /// Check contracts only
        #[arg(long)]
        contracts: bool,
    },
    /// GPU/SIMD compute operations
    Compute {
        /// Source file with compute kernels
        file: String,
        /// List available GPU devices
        #[arg(long)]
        list_devices: bool,
    },
}

/// Package manager subcommands
#[derive(Subcommand)]
pub enum PkgCommands {
    /// Initialize a new SSL project
    Init {
        /// Project name
        name: String,
        /// Create a library instead of an application
        #[arg(long)]
        lib: bool,
    },
    /// Add a dependency
    Add {
        /// Package name
        package: String,
        /// Version (default: latest)
        #[arg(long)]
        version: Option<String>,
    },
    /// Build the project
    Build {
        /// Enable release optimizations
        #[arg(long)]
        release: bool,
    },
    /// Update dependencies
    Update,
    /// Publish package to registry
    Publish,
    /// Search for packages
    Search {
        /// Search query
        query: String,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },
    /// List installed packages
    List,
    /// Security audit
    Audit,
}

/// Test subcommands
#[derive(Subcommand)]
pub enum TestCommands {
    /// Run property-based tests
    Property {
        /// Source file with property tests
        file: String,
        /// Number of iterations
        #[arg(long, default_value = "100")]
        iterations: usize,
        /// Random seed for reproducibility
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Run unit tests
    Unit {
        /// Source file or directory
        path: Option<String>,
    },
    /// Run all tests
    All {
        /// Source directory
        path: Option<String>,
    },
}

/// CRDT subcommands
#[derive(Subcommand)]
pub enum CrdtCommands {
    /// Start a CRDT sync server
    Serve {
        /// Port to listen on
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Connect to a CRDT peer
    Connect {
        /// Peer address
        address: String,
    },
    /// Show CRDT state
    Status,
}
