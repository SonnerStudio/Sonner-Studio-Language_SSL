use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ssl")]
#[command(about = "Sonner Studio Language Toolchain", long_about = None)]
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
    },
    /// Build a SSL project
    Build {
        /// The source file or project to build
        path: Option<String>,
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
}

