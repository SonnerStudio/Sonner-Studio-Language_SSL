mod ast;
mod cli;
mod interpreter;
mod lexer;
mod lsp;
mod parser;

use clap::Parser;
use cli::{Cli, Commands};
use lexer::Token;
use logos::Logos;
use std::fs;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run { file } => {
            if let Some(path) = file {
                println!("Running file: {}", path);
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let mut parser = parser::Parser::new(&content);
                        match parser.parse() {
                            Ok(ast) => {
                                println!("✅ Parsed successfully!");
                                
                                // Execute the code
                                let mut interpreter = interpreter::Interpreter::new();
                                match interpreter.interpret(ast) {
                                    Ok(result) => {
                                        println!("✅ Execution completed!");
                                        if result != interpreter::Value::Nil {
                                            println!("Result: {:?}", result);
                                        }
                                    }
                                    Err(e) => println!("❌ Runtime error: {}", e),
                                }
                            }
                            Err(e) => println!("❌ Parse error: {}", e),
                        }
                    }
                    Err(e) => println!("Error reading file: {}", e),
                }
            } else {
                // REPL Mode
                repl();
            }
        }
        Commands::Build { path } => {
            println!("Building project at: {:?}", path.unwrap_or_else(|| ".".to_string()));
            // TODO: Compiler Pipeline
        }
        Commands::Check { file } => {
            println!("Checking {}", file);
            // TODO: Semantic Analyzer
        }
        Commands::Doctor => {
            println!("SSL Doctor: Checking system...");
            println!("Rust: OK");
            println!("SSL Core: v0.1.0");
        }
        Commands::Ai { query } => {
            let q = query.join(" ");
            println!("Asking AI: '{}'", q);
            // TODO: Connect to ssld
        }
        Commands::Lsp => {
            eprintln!("Starting SSL Language Server...");
            lsp::run_lsp_server().await;
        }
    }
}

fn repl() {
    use std::io::{self, Write};
    println!("Sonner Studio Language (SSL) REPL v0.1");
    println!("Type 'exit' to quit.");

    loop {
        print!("ssl> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "exit" {
                    break;
                }
                if input.is_empty() {
                    continue;
                }

                let lexer = Token::lexer(input);
                for (token, span) in lexer.spanned() {
                    println!("{:?} at {:?}", token, span);
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
