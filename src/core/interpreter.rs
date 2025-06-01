use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::Rc;
use crate::core::parser::Expr;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    String(String),
    Number(f32),
    Bool(bool),
    Array(Vec<Value>),
    Function {
        name: String,
        params: Vec<String>,
    }
}

impl Value {
    fn to_string(&self) -> String {
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
            Value::Function { name, params } => todo!(),
            _ => "".to_string()
        }
    }
}

pub struct Env {
    pub variables: HashMap<String, Value>
}

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Rc::new(RefCell::new(Env { variables: HashMap::new() }))}
    }
    pub fn compute<'a>(&mut self, exprs: &mut Peekable<impl Iterator<Item = &'a Expr>>, env: Rc<RefCell<Env>>) -> Value  {
        let mut result: Value = Value::Nil;
        while let Some(expr) = exprs.next() {
            match expr {
                Expr::Number(n) => result = Value::Number(*n),
                Expr::String(s) => result = Value::String(String::from(s)),
                Expr::Array(arr) => {
                    let mut values: Vec<Value> = Vec::new();
                    for a in arr {
                        let mut i = std::iter::once(a).peekable();
                        let v = self.compute(&mut i, env.clone());
                        values.push(v);
                    }
                    result = Value::Array(values);
                }
                Expr::Symbol(s) => {
                    let env_rc = self.env.clone();
                    let env = env_rc.borrow();
                    if env.variables.contains_key(s) {
                        let value = env.variables.get(s).unwrap();
                        match value {
                            Value::Number(n) => result = Value::Number(*n),
                            Value::String(s) => result = Value::String(String::from(s)),
                            Value::Array(arr) => result = Value::Array(arr.iter().cloned().collect()),
                            _ => continue
                        }
                    }
                },
                Expr::List(e) => {
                    let mut args = e[1..e.len()].into_iter().peekable();
                    match &e[0] {
                        Expr::Symbol(s) if s == "+" => 
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a + b, 0.0),
                        Expr::Symbol(s) if s == "-" =>
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a - b, 0.0),
                        Expr::Symbol(s) if s == "*" =>
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a * b, 1.0),
                        Expr::Symbol(s) if s == "/" => {
                            let mut i = std::iter::once(&e[1]).peekable();
                            match self.compute(&mut i, env.clone()) {
                                Value::Number(n ) => result = self.numeric_fold(&e[2..], env.clone(), |a, b| a / b, n),
                                _ => panic!("Invalid operands for division !")
                            }
                        },
                        Expr::Symbol(s) if s == ">" => result = self.binary_operator(env.clone(),|a, b| a > b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "<" => result = self.binary_operator(env.clone(),|a, b| a < b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "==" => result = self.binary_operator(env.clone(),|a, b| a == b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "!=" => result = self.binary_operator(env.clone(),|a, b| a != b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == ">=" => result = self.binary_operator(env.clone(),|a, b| a >= b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "<=" => result = self.binary_operator(env.clone(),|a, b| a <= b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "if" => {
                            let condition = self.compute(&mut std::iter::once(&e[1]).peekable(), self.env.clone());
                            if let Value::Bool(b) = condition {
                                if b {
                                    let e1 = self.compute(&mut std::iter::once(&e[2]).peekable(), self.env.clone());
                                    result = e1;
                                } else {
                                    let e2 = self.compute(&mut std::iter::once(&e[3]).peekable(), self.env.clone());
                                    result = e2;
                                }
                            } else {
                                result = Value::Nil;
                            }
                        },
                        Expr::Symbol(s) if s == "print" => {
                            let arg1: Value = self.compute(&mut args, env.clone());
                            println!("{}", arg1.to_string());
                        },
                        Expr::Symbol(s) if s == "let" => {
                            let arg1 = args.next();
                            if let Some(Expr::Symbol(name)) = arg1 {
                                let arg2 = args.next();
                                if let Some(v) = arg2 {
                                    let value = self.compute(&mut std::iter::once(v).peekable(), env.clone());
                                    self.env.clone().borrow_mut().variables.insert(name.to_string(), value);
                                }
                            }
                        },
                        Expr::Symbol(s) if s == "len" => {
                            let arg1 = self.compute(&mut args, env.clone());
                            if let Value::Array(arr) = arg1 {
                                result = Value::Number(arr.len() as f32);
                            }
                        }
                        _ => continue
                    }
                },
            }
        }
        result
    }

    fn numeric_fold<F>(&mut self, exprs: &[Expr], env: Rc<RefCell<Env>>, operator: F, init: f32) -> Value
    where F: Fn(f32, f32) -> f32
    {
        let mut result: f32 = init;
        for expr in exprs {
            let mut i = std::iter::once(expr).peekable();
            if let Value::Number(n) = self.compute(&mut i, env.clone()) {
                result = operator(result, n);
            } else {
                panic!("Expected number");
            }
        }
        Value::Number(result)
    }

    fn binary_operator<F>(&mut self, env: Rc<RefCell<Env>>, operator: F, o1: &Expr, o2: &Expr) -> Value
        where F: Fn(f32, f32) -> bool
    {
        let mut result: bool = false;
        let i1 = self.compute(&mut std::iter::once(o1).peekable(), env.clone());
        let i2 = self.compute(&mut std::iter::once(o2).peekable(), env.clone());
        if let (Value::Number(n1), Value::Number(n2)) = (i1, i2) {
            result = operator(n1, n2);
        }
        Value::Bool(result)
    }
}