use std::cell::RefCell;
use std::rc::Rc;
use crate::core::annotation::Annotation;
use crate::core::env::Env;
use crate::core::expr::Expr;
use crate::core::stdlib;
use crate::core::stdlib::NativeFunction;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    String(String),
    Number(f32),
    Bool(bool),
    Array(Vec<Value>),
    Function {
        params: Vec<String>,
        body: Box<Expr>,
        func_env: Rc<RefCell<Env>>,
        annotations: Vec<Annotation>,
    },
    NativeFunction(NativeFunction)
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            //Value::Nil => "nil".to_string(),
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => {
                let mut res = String::from("[");
                for v in arr {
                    res.push_str(&v.to_string());
                    res.push(' ');
                }
                res.pop();
                res.push_str("]");
                res
            },
            Value::Nil => String::from("nil"),
            _ => "".to_string()
        }
    }

    pub fn as_number(&self) -> f32 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Expected a number, got {:?}", self),
        }
    }
}