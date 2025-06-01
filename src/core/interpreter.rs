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
        params: Vec<String>,
        body: Box<Expr>,
        func_env: Rc<RefCell<Env>>,
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
            _ => "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Env {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>
}

impl Env {
    pub fn get(&self, key: &str) -> Option<Value> {
        if let Some(val) = self.variables.get(key) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent.clone() {
            parent.borrow().get(key)
        } else {
            return None;
        }
    }

    pub fn set(&mut self, key: &str, value: Value) {
        if let Some(val) = self.variables.get_mut(key) {
            *val = value;
        } else if let Some(parent) = &self.parent.clone() {
            parent.borrow_mut().set(key, value);
        } else {
            panic!("Variable {} not found", key);
        }
    }
}

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Rc::new(RefCell::new(Env { variables: HashMap::new(), parent: None })) }
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
                    let env_rc = env.clone();
                    let env = env_rc.borrow();
                    if let Some(_) = env.get(s) {
                        let value = env.variables.get(s).unwrap();
                        match value {
                            Value::Number(n) => result = Value::Number(*n),
                            Value::String(s) => result = Value::String(String::from(s)),
                            Value::Array(arr) => result = Value::Array(arr.iter().cloned().collect()),
                            Value::Bool(b) => result = Value::Bool(*b),
                            _ => result = Value::Nil
                        }
                    }
                },
                Expr::List(e) => {
                    if e.len() == 1 {
                        result = self.compute(&mut std::iter::once(&e[0]).peekable(), env.clone());
                        continue;
                    }
                    let mut args = e[1..e.len()].into_iter().peekable();
                    match &e[0] {
                        Expr::Symbol(s) if s == "+" => 
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a + b, Some(0.0)),
                        Expr::Symbol(s) if s == "-" =>
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a - b, None),
                        Expr::Symbol(s) if s == "*" =>
                            result = self.numeric_fold(&e[1..], env.clone(), |a, b| a * b, Some(1.0)),
                        Expr::Symbol(s) if s == "/" => {
                            let mut i = std::iter::once(&e[1]).peekable();
                            match self.compute(&mut i, env.clone()) {
                                Value::Number(n ) => result = self.numeric_fold(&e[1..], env.clone(), |a, b| a / b, None),
                                _ => panic!("Invalid operands for division !")
                            }
                        },
                        Expr::Symbol(s) if s == ">" => result = self.binary_operator(env.clone(),|a, b| a > b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "<" => result = self.binary_operator(env.clone(),|a, b| a < b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "==" => result = self.binary_operator(env.clone(),|a, b| a == b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "!=" => result = self.binary_operator(env.clone(),|a, b| a != b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == ">=" => result = self.binary_operator(env.clone(),|a, b| a >= b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "<=" => result = self.binary_operator(env.clone(),|a, b| a <= b, &e[1], &e[2]),
                        Expr::Symbol(s) if s == "do" => {
                            for arg in args {
                                let mut i = std::iter::once(arg).peekable();
                                result = self.compute(&mut i, env.clone());
                            }
                        },
                        Expr::Symbol(s) if s == "if" => {
                            let condition = self.compute(&mut std::iter::once(&e[1]).peekable(), env.clone());
                            if let Value::Bool(b) = condition {
                                if b {
                                    let e1 = self.compute(&mut std::iter::once(&e[2]).peekable(), env.clone());
                                    result = e1;
                                } else {
                                    let e2 = self.compute(&mut std::iter::once(&e[3]).peekable(), env.clone());
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
                        Expr::Symbol(s) if s == "var" => {
                            let arg1 = args.next();
                            if let Some(Expr::Symbol(name)) = arg1 {
                                let arg2 = args.next();
                                if let Some(v) = arg2 {
                                    let value = self.compute(&mut std::iter::once(v).peekable(), env.clone());
                                    env.clone().borrow_mut().variables.insert(name.to_string(), value);
                                }
                            }
                        },
                        Expr::Symbol(s) if s == "let" => {
                            let local_env = Rc::new(RefCell::new(
                                Env {
                                    variables: Default::default(),
                                    parent: Some(env.clone()),
                                }
                            ));

                            let bindings_expr = &e[1];
                            let body_exprs = &e[2..];
                            if let Expr::List(bindings) = bindings_expr {
                                for binding in bindings {
                                    if let Expr::List(pair) = binding {
                                        if pair.len() != 2 {
                                            panic!("Local declaration should contain variable name and value !");
                                        }
                                        if let Expr::Symbol(name) = &pair[0] {
                                            let value = self.compute(&mut std::iter::once(&pair[1]).peekable(), env.clone());
                                            local_env.clone().borrow_mut().variables.insert(name.to_string(), value);
                                        }
                                    }
                                }
                            }
                            for expr in body_exprs {
                                let value = self.compute(&mut std::iter::once(expr).peekable(), local_env.clone());
                                result = value;
                            }
                        },
                        Expr::Symbol(s) if s == "fn" => {
                            if let (Expr::Symbol(fn_name), Expr::List(fn_args), body_expr) = (&e[1], &e[2], &e[3]) {
                                let function_name = fn_name.to_string();

                                let function_arguments: Vec<String> = fn_args.iter().filter_map(|arg| {
                                    if let Expr::Symbol(name) = arg {
                                        Some(name.clone())
                                    } else {
                                        None
                                    }
                                }).collect();

                                let function_body = Box::new(body_expr.clone());

                                let captured_env = env.clone();

                                let function = Value::Function {
                                    params: function_arguments,
                                    body: function_body,
                                    func_env: captured_env,
                                };
                                env.borrow_mut().variables.insert(function_name.clone(), function);
                                result = Value::Nil;
                            } else {
                                panic!("Invalid function definition syntax");
                            }
                        },
                        Expr::Symbol(s) if s == "len" => {
                            let arg1 = self.compute(&mut args, env.clone());
                            if let Value::Array(arr) = arg1 {
                                result = Value::Number(arr.len() as f32);
                            }
                        },
                        Expr::Symbol(s) => {
                            if let Some(Value::Function { params, body, func_env }) = env.borrow().get(s) {
                                let params_value: Vec<_> = e[1..]
                                    .iter()
                                    .map(|p| self.compute(&mut std::iter::once(p).peekable(), env.clone()))
                                    .collect();

                                let local_env = Rc::new(RefCell::new(Env {
                                    variables: HashMap::new(),
                                    parent: Some(func_env.clone()), // closure
                                }));

                                for (param, val) in params.iter().zip(params_value) {
                                    local_env.borrow_mut().variables.insert(param.clone(), val);
                                }

                                let mut body_iter = std::iter::once(body.as_ref()).peekable();
                                result = self.compute(&mut body_iter, local_env);
                            } else {
                                panic!("Undefined function: {}", s);
                            }
                        }
                        _ => continue
                    }
                },
            }
        }
        result
    }

    fn numeric_fold<F>(&mut self, exprs: &[Expr], env: Rc<RefCell<Env>>, operator: F, init: Option<f32>) -> Value
    where
        F: Fn(f32, f32) -> f32,
    {
        let mut exprs_iter = exprs.iter();
        let first_value = if let Some(i) = init {
            i
        } else if let Some(first_expr) = exprs_iter.next() {
            if let Value::Number(n) = self.compute(&mut std::iter::once(first_expr).peekable(), env.clone()) {
                n
            } else {
                panic!("Expected number as first operand")
            }
        } else {
            panic!("Expected at least one argument")
        };

        let result = exprs_iter.fold(first_value, |acc, expr| {
            if let Value::Number(n) = self.compute(&mut std::iter::once(expr).peekable(), env.clone()) {
                operator(acc, n)
            } else {
                panic!("Expected number")
            }
        });

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