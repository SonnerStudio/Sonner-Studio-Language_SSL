use crate::interpreter::{Interpreter, Value};
use uuid::Uuid;

pub fn register(interpreter: &mut Interpreter) {
    interpreter.register_native_function("uuid_v4", |_args| {
        Ok(Value::String(Uuid::new_v4().to_string()))
    });
}
