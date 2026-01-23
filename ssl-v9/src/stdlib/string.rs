use crate::interpreter::{Interpreter, Value};
use std::sync::Arc;

pub fn register(interpreter: &mut Interpreter) {
    interpreter.register_native_function("string_contains", |args| {
        if args.len() != 2 {
            return Err("string_contains expects 2 arguments: (string, substring)".to_string());
        }
        
        let s = match &args[0] {
            Value::String(s) => s,
            _ => return Err("string_contains first argument must be a string".to_string()),
        };
        
        let sub = match &args[1] {
            Value::String(s) => s,
            _ => return Err("string_contains second argument must be a string".to_string()),
        };
        
        Ok(Value::Bool(s.contains(sub)))
    });
}
