use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use ssl_v9::interpreter::Interpreter;
use ssl_v9::ast::Statement;
use ssl_v9::stdlib;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to port 8080");
    println!("SSL Daemon (ssld) listening on 0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Accepted connection from: {}", peer_addr);

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    // Read JSON AST (terminated by newline)
    match reader.read_line(&mut line) {
        Ok(0) => return, // Connection closed
        Ok(_) => {
            match serde_json::from_str::<Vec<Statement>>(&line) {
                Ok(ast) => {
                    println!("Received AST from {}. Executing...", peer_addr);
                    let mut interpreter = Interpreter::new();
                    stdlib::register_all(&mut interpreter);
                    match interpreter.interpret(ast) {
                        Ok(val) => {
                            println!("Execution finished for {}. Result: {:?}", peer_addr, val);
                            // Optionally send result back?
                            // For now, just log it.
                        }
                        Err(e) => {
                            eprintln!("Execution error for {}: {}", peer_addr, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize AST from {}: {}", peer_addr, e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading from {}: {}", peer_addr, e);
        }
    }
}
