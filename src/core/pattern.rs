use crate::core::value::Value;

#[derive(Debug)]
pub struct Pattern {
    pub value: Value,
    pub result: Value,
}

impl Pattern {
    pub fn handle_number_matching() -> Value {
        
        Value::Nil
    }
}