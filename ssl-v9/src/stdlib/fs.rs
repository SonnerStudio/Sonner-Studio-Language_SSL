use crate::interpreter::{Value, Interpreter};
use crate::security::Permission;
use std::fs;
use std::io::Write;

pub fn register(interpreter: &mut Interpreter) {
    // fs_read(path)
    let sec_man = interpreter.security_manager.clone();
    interpreter.register_native_function("fs_read", move |args| {
        if args.len() != 1 {
            return Err("fs_read expects 1 argument (path)".to_string());
        }
        match &args[0] {
            Value::String(path) => {
                if let Err(e) = sec_man.check_permission(Permission::FileRead(path.clone())) {
                    return Err(e);
                }
                match fs::read_to_string(path) {
                    Ok(content) => Ok(Value::String(content)),
                    Err(e) => Err(format!("fs_read failed: {}", e)),
                }
            },
            _ => Err("fs_read expects a string path".to_string()),
        }
    });

    // fs_write(path, content)
    let sec_man = interpreter.security_manager.clone();
    interpreter.register_native_function("fs_write", move |args| {
        if args.len() != 2 {
            return Err("fs_write expects 2 arguments (path, content)".to_string());
        }
        match (&args[0], &args[1]) {
            (Value::String(path), Value::String(content)) => {
                if let Err(e) = sec_man.check_permission(Permission::FileWrite(path.clone())) {
                    return Err(e);
                }
                match fs::write(path, content) {
                    Ok(_) => Ok(Value::Nil),
                    Err(e) => Err(format!("fs_write failed: {}", e)),
                }
            },
            _ => Err("fs_write expects (string, string)".to_string()),
        }
    });

    // fs_exists(path)
    interpreter.register_native_function("fs_exists", |args| {
        if args.len() != 1 {
            return Err("fs_exists expects 1 argument (path)".to_string());
        }
        match &args[0] {
            Value::String(path) => {
                Ok(Value::Bool(std::path::Path::new(path).exists()))
            },
            _ => Err("fs_exists expects a string path".to_string()),
        }
    });
}
