use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::core::env::Env;
use crate::core::expr::Expr;
use crate::core::value::Value;

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Pure(fn(Vec<Value>) -> Value),
    WithEnv(fn(Vec<Value>, Rc<RefCell<Env>>) -> Value),
}

pub struct Stdlib;

impl Stdlib {
    pub fn builtins() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.extend(Stdlib::math_symbol()); // -> + - * / ^ > >= < <=
        map.extend(Stdlib::logical_symbol()); // -> && || != ==
        map.extend(Stdlib::io_functions()); // -> print, read
        map
    }

    fn math_symbol() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("+".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a + b)
                } else {
                    panic!("< + > only apply on two numbers !")
                }
            }
        )));
        map.insert("-".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a - b)
                } else {
                    panic!("< - > only apply on two numbers !")
                }
            }
        )));
        map.insert("*".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a * b)
                } else {
                    panic!("< * > only apply on two numbers !")
                }
            }
        )));
        map.insert("/".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a / b)
                } else {
                    panic!("< / > only apply on two numbers !")
                }
            }
        )));
        map.insert("^".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                    Value::Number(a**b)
                } else {
                    panic!("< ^ > only apply on two numbers !")
                }
            }
        )));
        map.insert(">".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                Value::Bool(a > b)
            } else {
                panic!("> only apply on two numbers !")
            }
        }
        )));
        map.insert("<".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                Value::Bool(a < b)
            } else {
                panic!("< only apply on two numbers !")
            }
        }
        )));
        map.insert(">=".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                Value::Bool(a >= b)
            } else {
                panic!(">= only apply on two numbers !")
            }
        }
        )));
        map.insert("<=".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let (Some(Value::Number(a)), Some(Value::Number(b))) = (args.get(0), args.get(1)) {
                Value::Bool(a <= b)
            } else {
                panic!("<= only apply on two numbers !")
            }
        }
        )));
        map
    }
    fn logical_symbol() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("&&".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Bool(a)), Some(Value::Bool(b))) = (args.get(0), args.get(1)) {
                    Value::Bool(*a && *b)
                } else {
                    panic!("< && > only apply on two booleans !")
                }
            }
        )));
        map.insert("||".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                if let (Some(Value::Bool(a)), Some(Value::Bool(b))) = (args.get(0), args.get(1)) {
                    Value::Bool(*a || *b)
                } else {
                    panic!("< || > only apply on two booleans !")
                }
            }
        )));
        map.insert("==".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                match (args.get(0), args.get(1)) {
                    (Some(Value::Bool(a)), Some(Value::Bool(b))) => Value::Bool(*a == *b),
                    (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Bool(a == b),
                    (Some(Value::String(a)), Some(Value::String(b))) => Value::Bool(*a == *b),
                    _ => panic!("< == > misses parameters !")
                }
            }
        )));
        map.insert("!=".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
                match (args.get(0), args.get(1)) {
                    (Some(Value::Bool(a)), Some(Value::Bool(b))) => Value::Bool(*a != *b),
                    (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Bool(a != b),
                    (Some(Value::String(a)), Some(Value::String(b))) => Value::Bool(*a != *b),
                    _ => panic!("< == > misses parameters !")
                }
            }
        )));
        map
    }

    fn io_functions() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("print".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            println!("{}", args.get(0).unwrap().to_string());
            Value::Nil
        })));
        map.insert("read".to_string(), Value::NativeFunction(NativeFunction::Pure(|_| {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            Value::String(line)
        })));
        map
    }
}