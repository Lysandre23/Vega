use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
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
        map.extend(Stdlib::io_functions()); // -> print read
        map.extend(Stdlib::language_functions()); // -> typeof
        map.extend(Stdlib::array_functions()); // -> len get
        map.extend(Stdlib::string_functions()); // -> parse
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
        map.insert("abs".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let Some(Value::Number(n)) = args.get(0) {
                Value::Number(n.abs())
            } else {
                panic!("abs expects a number")
            }
        })));
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
        map.insert("not".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let Some(Value::Bool(b)) = args.get(0) {
                Value::Bool(!b)
            } else {
                panic!("not expects a boolean")
            }
        })));
        map
    }
    fn io_functions() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("print".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            for val in args {
                print!("{} ", val.to_string());
            }
            println!();
            Value::Nil
        })));
        map.insert("read".to_string(), Value::NativeFunction(NativeFunction::Pure(|_| {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            Value::String(line.trim_end().to_string())
        })));
        map
    }
    fn language_functions() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("typeof".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            return match args.first() {
                Some(Value::Number(_)) => Value::String("Number".to_string()),
                Some(Value::String(_)) => Value::String("String".to_string()),
                Some(Value::Array(_)) => Value::String("Array".to_string()),
                Some(Value::Function { params: _, body: _, func_env: _, annotations: _ }) => Value::String("Function".to_string()),
                Some(Value::NativeFunction(_)) => Value::String("Function".to_string()),
                Some(Value::Bool(_)) => Value::String("Bool".to_string()),
                _ => panic!("typeof misses parameters !")
            }
        })));
        map.insert("get".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            match args.get(0) {
                Some(Value::Array(arr)) => {
                    let index = match args.get(1) {
                        Some(Value::Number(n)) => *n as usize,
                        _ => panic!("Second argument to get must be a number (index)"),
                    };
                    arr.get(index).cloned().unwrap_or(Value::Nil)
                },
                Some(Value::String(s)) => {
                    let index = match args.get(1) {
                        Some(Value::Number(n)) => *n as usize,
                        _ => panic!("Second argument to get must be a number (index)"),
                    };
                    s.chars().nth(index)
                        .map(|c| Value::String(c.to_string()))
                        .unwrap_or(Value::Nil)
                },
                Some(Value::Object{class, attrs}) => {
                    let key = args.get(1).unwrap_or(&Value::Nil);
                    if let Value::String(s) = key {
                        if attrs.contains_key(s) {
                            attrs.get(s).unwrap().clone()
                        } else {
                            Value::Nil
                        }
                    } else {
                        Value::Nil
                    }
                },
                _ => panic!("get only supports arrays and strings"),
            }
        })));
        /*map.insert("set".to_string(), Value::NativeFunction(NativeFunction::WithEnv(|args, env| {
            match args.get(0) {
                Some(Value::Array(arr)) => {}
            }
        })));*/
        map
    }
    fn array_functions() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("len".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            return match args.get(0) {
                Some(Value::Array(arr)) => Value::Number(arr.len() as f32),
                Some(Value::String(s)) => Value::Number(s.chars().count() as f32),
                _ => panic!("Type has no length !")
            }
        })));
        map.insert("concat".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if args.len() == 0 {
                return Value::Nil;
            }
            return match args[0] {
                Value::String(_) => {
                    let mut result = String::new();
                    for arg in args.iter() {
                        result.push_str(&arg.to_string());
                    }
                    Value::String(result)
                }
                _ => Value::Nil,
            }
        })));
        map.insert("range".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            if let (Some(Value::Number(start)), Some(Value::Number(end))) = (args.get(0), args.get(1)) {
                let mut vec: Vec<Value> = Vec::new();
                for i in (*start as i32)..(*end as i32) {
                    vec.push(Value::Number(i as f32));
                }
                Value::Array(vec)
            } else {
                panic!("range expects a number argument");
            }
        })));
        map
    }
    fn string_functions() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("parse".to_string(), Value::NativeFunction(NativeFunction::Pure(|args| {
            return match args.get(0) {
                Some(Value::String(s)) => {
                    match s.parse::<f32>() {
                        Ok(f) => Value::Number(f),
                        Err(_) => Value::Nil,
                    }
                },
                _ => Value::Nil,
            }
        })));
        map
    }
}