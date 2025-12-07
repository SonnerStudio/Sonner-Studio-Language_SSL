use crate::interpreter::{Value, Interpreter};
use reqwest::blocking::Client;

pub fn register(interpreter: &mut Interpreter) {
    let client = Client::new();

    // http_get(url)
    interpreter.register_native_function("http_get", move |args| {
        if args.len() != 1 {
            return Err("http_get expects 1 argument (url)".to_string());
        }
        match &args[0] {
            Value::String(url) => {
                // Create a new client for each request to avoid lifetime issues with closure
                // In production we would wrap Client in Arc/Mutex or similar
                let res = reqwest::blocking::get(url)
                    .map_err(|e| format!("HTTP request failed: {}", e))?;
                
                let text = res.text()
                    .map_err(|e| format!("Failed to read response text: {}", e))?;
                
                Ok(Value::String(text))
            },
            _ => Err("http_get expects a string URL".to_string()),
        }
    });
}
