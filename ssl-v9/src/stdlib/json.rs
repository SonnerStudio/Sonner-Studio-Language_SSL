use crate::interpreter::{Value, Interpreter};
use serde_json::{self, json};
use std::collections::HashMap;

pub fn register(interpreter: &mut Interpreter) {
    // json_parse(string) -> Value
    interpreter.register_native_function("json_parse", |args| {
        if args.len() != 1 {
            return Err("json_parse expects 1 argument (string)".to_string());
        }
        match &args[0] {
            Value::String(s) => {
                match serde_json::from_str::<serde_json::Value>(s) {
                    Ok(json_val) => Ok(json_to_value(&json_val)),
                    Err(e) => Err(format!("Invalid JSON: {}", e)),
                }
            },
            _ => Err("json_parse expects a string".to_string()),
        }
    });

    // json_stringify(value) -> String
    interpreter.register_native_function("json_stringify", |args| {
        if args.len() != 1 {
            return Err("json_stringify expects 1 argument".to_string());
        }
        let json_val = value_to_json(&args[0]);
        Ok(Value::String(json_val.to_string()))
    });
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Int(i) => json!(i),
        Value::Float(f) => json!(f),
        Value::String(s) => json!(s),
        Value::Bool(b) => json!(b),
        Value::List(l) => {
            let vec: Vec<serde_json::Value> = l.iter().map(value_to_json).collect();
            json!(vec)
        },
        Value::Map(m) => {
            let mut map = serde_json::Map::new();
            for (k, v) in m {
                map.insert(k.clone(), value_to_json(v));
            }
            json!(map)
        },
        Value::Nil => json!(null),
        _ => json!(format!("{:?}", v)), // Fallback
    }
}

fn json_to_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Value::Int(n.as_i64().unwrap())
            } else {
                Value::Float(n.as_f64().unwrap_or(0.0))
            }
        },
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(a) => {
            Value::List(a.iter().map(json_to_value).collect())
        },
        serde_json::Value::Object(o) => {
            let mut map = HashMap::new();
            for (k, v) in o {
                map.insert(k.clone(), json_to_value(v));
            }
            Value::Map(map)
        }
    }
}
