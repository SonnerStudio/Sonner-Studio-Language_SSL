use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ssl")]
#[command(version = "9.3.0")]
#[command(about = "Sonner Studio Language v9.3 - Aurora Engine")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile an SSL source file to native executable
    Compile {
        /// Input source file (.ssl)
        #[arg(value_parser)]
        source: String,
    },
    
    /// Run an SSL script (Interpreted Mode)
    Run {
        /// Input file
        #[arg(value_parser)]
        file: Option<String>,
        
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
        
        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
        
        /// AI Code Review
        #[arg(long)]
        ai_review: bool,

        /// Script arguments
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Check system status
    Doctor,
}
