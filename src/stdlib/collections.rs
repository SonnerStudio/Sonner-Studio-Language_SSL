// SSL 3.0 - Functional Collections Module
// Provides functional update methods for Maps and Lists

use crate::interpreter::{Interpreter, Value};
use std::collections::HashMap;

pub fn register(interpreter: &mut Interpreter) {
    // Map.with(key, value) -> neue Map mit hinzugefügtem/aktualisiertem Key
    interpreter.register_native_function("map_with", |args| {
        if args.len() != 3 {
            return Err("map_with expects 3 arguments: (map, key, value)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::Map(map), Value::String(key)) => {
                let mut new_map = map.clone();
                new_map.insert(key.clone(), args[2].clone());
                Ok(Value::Map(new_map))
            }
            _ => Err("map_with requires (Map, String, value)".to_string()),
        }
    });
    
    // Map.without(key) -> neue Map ohne diesen Key
    interpreter.register_native_function("map_without", |args| {
        if args.len() != 2 {
            return Err("map_without expects 2 arguments: (map, key)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::Map(map), Value::String(key)) => {
                let mut new_map = map.clone();
                new_map.remove(key);
                Ok(Value::Map(new_map))
            }
            _ => Err("map_without requires (Map, String)".to_string()),
        }
    });
    
    // Map.get(key, default) -> Wert oder Default
    interpreter.register_native_function("map_get", |args| {
        if args.len() != 3 {
            return Err("map_get expects 3 arguments: (map, key, default)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::Map(map), Value::String(key)) => {
                Ok(map.get(key).cloned().unwrap_or_else(|| args[2].clone()))
            }
            _ => Err("map_get requires (Map, String, default)".to_string()),
        }
    });
    
    // Map.keys() -> Liste aller Keys
    interpreter.register_native_function("map_keys", |args| {
        if args.len() != 1 {
            return Err("map_keys expects 1 argument: (map)".to_string());
        }
        
        match &args[0] {
            Value::Map(map) => {
                let keys: Vec<Value> = map.keys()
                    .map(|k| Value::String(k.clone()))
                    .collect();
                Ok(Value::List(keys))
            }
            _ => Err("map_keys requires a Map".to_string()),
        }
    });
    
    // Map.values() -> Liste aller Values
    interpreter.register_native_function("map_values", |args| {
        if args.len() != 1 {
            return Err("map_values expects 1 argument: (map)".to_string());
        }
        
        match &args[0] {
            Value::Map(map) => {
                let values: Vec<Value> = map.values().cloned().collect();
                Ok(Value::List(values))
            }
            _ => Err("map_values requires a Map".to_string()),
        }
    });
    
    // List.append(element) -> neue Liste mit angefügtem Element
    interpreter.register_native_function("list_append", |args| {
        if args.len() != 2 {
            return Err("list_append expects 2 arguments: (list, element)".to_string());
        }
        
        match &args[0] {
            Value::List(list) => {
                let mut new_list = list.clone();
                new_list.push(args[1].clone());
                Ok(Value::List(new_list))
            }
            _ => Err("list_append requires a List".to_string()),
        }
    });
    
    // List.prepend(element) -> neue Liste mit vorangestelltem Element
    interpreter.register_native_function("list_prepend", |args| {
        if args.len() != 2 {
            return Err("list_prepend expects 2 arguments: (list, element)".to_string());
        }
        
        match &args[0] {
            Value::List(list) => {
                let mut new_list = vec![args[1].clone()];
                new_list.extend(list.clone());
                Ok(Value::List(new_list))
            }
            _ => Err("list_prepend requires a List".to_string()),
        }
    });
    
    // List.concat(other_list) -> neue Liste mit beiden Listen kombiniert
    interpreter.register_native_function("list_concat", |args| {
        if args.len() != 2 {
            return Err("list_concat expects 2 arguments: (list1, list2)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::List(list1), Value::List(list2)) => {
                let mut new_list = list1.clone();
                new_list.extend(list2.clone());
                Ok(Value::List(new_list))
            }
            _ => Err("list_concat requires two Lists".to_string()),
        }
    });
    
    // List.with_index(index, value) -> neue Liste mit geändertem Element
    interpreter.register_native_function("list_with_index", |args| {
        if args.len() != 3 {
            return Err("list_with_index expects 3 arguments: (list, index, value)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::List(list), Value::Int(i)) => {
                if *i >= 0 && (*i as usize) < list.len() {
                    let mut new_list = list.clone();
                    new_list[*i as usize] = args[2].clone();
                    Ok(Value::List(new_list))
                } else {
                    Err(format!("Index out of bounds: {}", i))
                }
            }
            _ => Err("list_with_index requires (List, Int, value)".to_string()),
        }
    });
    
    // List.filter(predicate) -> neue gefilterte Liste
    interpreter.register_native_function("list_filter", |args| {
        if args.len() != 2 {
            return Err("list_filter expects 2 arguments: (list, predicate_fn)".to_string());
        }
        
        match &args[0] {
            Value::List(list) => {
                // Für jetzt verwenden wir einen simplen Ansatz
                // In einer vollständigen Implementation würde predicate eine Funktion sein
                // die wir für jedes Element aufrufen
                // Placeholder: Filtert alle Nil-Werte raus
                let filtered: Vec<Value> = list.iter()
                    .filter(|v| !matches!(v, Value::Nil))
                    .cloned()
                    .collect();
                Ok(Value::List(filtered))
            }
            _ => Err("list_filter requires a List".to_string()),
        }
    });
    
    // List.take(n) -> erste n Elemente
    interpreter.register_native_function("list_take", |args| {
        if args.len() != 2 {
            return Err("list_take expects 2 arguments: (list, n)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::List(list), Value::Int(n)) => {
                let count = (*n).max(0) as usize;
                let taken: Vec<Value> = list.iter()
                    .take(count)
                    .cloned()
                    .collect();
                Ok(Value::List(taken))
            }
            _ => Err("list_take requires (List, Int)".to_string()),
        }
    });
    
    // List.drop(n) -> alle außer die ersten n Elemente
    interpreter.register_native_function("list_drop", |args| {
        if args.len() != 2 {
            return Err("list_drop expects 2 arguments: (list, n)".to_string());
        }
        
        match (&args[0], &args[1]) {
            (Value::List(list), Value::Int(n)) => {
                let count = (*n).max(0) as usize;
                let dropped: Vec<Value> = list.iter()
                    .skip(count)
                    .cloned()
                    .collect();
                Ok(Value::List(dropped))
            }
            _ => Err("list_drop requires (List, Int)".to_string()),
        }
    });
    
    // List.reverse() -> umgekehrte Liste
    interpreter.register_native_function("list_reverse", |args| {
        if args.len() != 1 {
            return Err("list_reverse expects 1 argument: (list)".to_string());
        }
        
        match &args[0] {
            Value::List(list) => {
                let mut reversed = list.clone();
                reversed.reverse();
                Ok(Value::List(reversed))
            }
            _ => Err("list_reverse requires a List".to_string()),
        }
    });
    
    // List.length() -> Anzahl der Elemente
    interpreter.register_native_function("list_length", |args| {
        if args.len() != 1 {
            return Err("list_length expects 1 argument: (list)".to_string());
        }
        
        match &args[0] {
            Value::List(list) => Ok(Value::Int(list.len() as i64)),
            _ => Err("list_length requires a List".to_string()),
        }
    });

    // SSL 3.0: partial(fn, arg1, arg2, ...) -> Partially applied function
    // Creates a new function with some arguments pre-filled
    interpreter.register_native_function("partial", |args| {
        if args.is_empty() {
            return Err("partial expects at least 1 argument: (function, args...)".to_string());
        }
        
        let function = args[0].clone();
        
        // Check that first argument is a function-like value
        match &function {
            Value::Function { .. } | 
            Value::ComposedFunction { .. } | 
            Value::PartiallyApplied { .. } => {
                let applied_args: Vec<Value> = args[1..].to_vec();
                
                if applied_args.is_empty() {
                    // No args to apply, just return the function as-is
                    Ok(function)
                } else {
                    Ok(Value::PartiallyApplied {
                        function: Box::new(function),
                        applied_args,
                    })
                }
            }
            _ => Err("partial requires a function as first argument".to_string()),
        }
    });

    // is_function(val) -> true if val is any callable type
    interpreter.register_native_function("is_function", |args| {
        if args.len() != 1 {
            return Err("is_function expects 1 argument".to_string());
        }
        
        let is_fn = matches!(&args[0], 
            Value::Function { .. } | 
            Value::ComposedFunction { .. } | 
            Value::PartiallyApplied { .. }
        );
        Ok(Value::Bool(is_fn))
    });

    // is_list(val) -> true if val is a list
    interpreter.register_native_function("is_list", |args| {
        if args.len() != 1 {
            return Err("is_list expects 1 argument".to_string());
        }
        Ok(Value::Bool(matches!(&args[0], Value::List(_))))
    });

    // is_map(val) -> true if val is a map
    interpreter.register_native_function("is_map", |args| {
        if args.len() != 1 {
            return Err("is_map expects 1 argument".to_string());
        }
        Ok(Value::Bool(matches!(&args[0], Value::Map(_))))
    });

    // print(value) -> prints value and returns nil
    interpreter.register_native_function("print", |args| {
        for arg in &args {
            match arg {
                Value::String(s) => print!("{}", s),
                Value::Int(n) => print!("{}", n),
                Value::Float(f) => print!("{}", f),
                Value::Bool(b) => print!("{}", b),
                Value::Nil => print!("nil"),
                Value::List(items) => {
                    print!("[");
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 { print!(", "); }
                        print!("{:?}", item);
                    }
                    print!("]");
                }
                Value::Map(map) => {
                    print!("{{");
                    for (i, (k, v)) in map.iter().enumerate() {
                        if i > 0 { print!(", "); }
                        print!("{}: {:?}", k, v);
                    }
                    print!("}}");
                }
                _ => print!("{:?}", arg),
            }
        }
        println!();
        Ok(Value::Nil)
    });
}
