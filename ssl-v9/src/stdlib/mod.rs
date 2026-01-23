pub mod env;
pub mod fs;
pub mod http;
pub mod json;
pub mod collections;  // SSL 3.0: Functional collections
pub mod uuid;         // SSL v9: Unique ID generation
pub mod string;       // SSL v9: String utilities

use crate::interpreter::Interpreter;

pub fn register_all(interpreter: &mut Interpreter) {
    env::register(interpreter);
    fs::register(interpreter);
    http::register(interpreter);
    json::register(interpreter);
    collections::register(interpreter);  // SSL 3.0
    uuid::register(interpreter);
    string::register(interpreter);
}
