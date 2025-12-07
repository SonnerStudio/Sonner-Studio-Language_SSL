use crate::interpreter::{Value, Interpreter};
use std::env;

pub fn register(interpreter: &mut Interpreter) {
    // env_get(key)
    interpreter.register_native_function("env_get", |args| {
        if args.len() != 1 {
            return Err("env_get expects 1 argument (key)".to_string());
        }
        match &args[0] {
            Value::String(key) => {
                match env::var(key) {
                    Ok(val) => Ok(Value::String(val)),
                    Err(_) => Ok(Value::Nil),
                }
            },
            _ => Err("env_get expects a string key".to_string()),
        }
    });

    // sys_os()
    interpreter.register_native_function("sys_os", |_| {
        Ok(Value::String(std::env::consts::OS.to_string()))
    });
}
