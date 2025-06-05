use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Deref;
use std::rc::Rc;
use crate::core::annotation::Annotation;
use crate::core::env::Env;
use crate::core::parser::Expr;
use crate::core::stdlib::{NativeFunction, Stdlib};
use crate::core::value::Value;

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut base = Env {
            variables: HashMap::new(),
            parent: None,
        };
        base.variables.extend(Stdlib::builtins());
        Interpreter {
            env: Rc::new(RefCell::new(base)),
        }
    }
    pub fn compute<'a>(&mut self, exprs: &mut Peekable<impl Iterator<Item = &'a Expr>>, env: Rc<RefCell<Env>>) -> Value  {
        let mut result: Value = Value::Nil;
        while let Some(expr) = exprs.next() {
            match expr {
                Expr::Number(n) => result = self.handle_number(*n),
                Expr::String(s) => result = self.handle_string(s.clone()),
                Expr::Array(arr) => result = self.handle_array(arr.clone(), env.clone()),
                Expr::Symbol(s) => result = self.handle_symbol(s.to_string(), env.clone()),
                Expr::List(e) => {
                    if e.len() == 1 {
                        result = self.compute(&mut std::iter::once(&e[0]).peekable(), env.clone());
                        continue;
                    }
                    let mut args = e[1..e.len()].into_iter().peekable();
                    match &e[0] {
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
                        Expr::Symbol(s) if s == "for" => {
                            let local_env = Rc::new(RefCell::new(
                               Env {
                                   variables: Default::default(),
                                   parent: Some(env.clone())
                               }
                            ));
                            let mut p: Vec<String> = Vec::new();
                            let mut r: Vec<Vec<f32>> = Vec::new();
                            if let Some(Expr::List(params)) = args.next() {
                                for param in params {
                                    if let Expr::Symbol(param_name) = &*param {
                                        p.push(param_name.to_string());
                                    }
                                }
                            }
                            if let Some(Expr::List(ranges)) = args.next() {
                                for range in ranges {
                                    let value_expr = self.compute(&mut std::iter::once(range).peekable(), local_env.clone());
                                    if let Value::Array(arr) = value_expr {
                                        let mut values: Vec<f32> = Vec::new();
                                        for value in arr {
                                            if let Value::Number(n) = value {
                                                values.push(n);
                                            }
                                        }
                                        r.push(values);
                                    }
                                }
                            }
                            let ast = args.next().unwrap();
                            let max = r.clone().into_iter().map(|n| n.iter().count()).min().unwrap();
                            for i in 0..max {
                                for (n,j) in p.iter().zip(0..p.iter().count()) {
                                    local_env.borrow_mut().variables.insert(
                                        n.to_string(), 
                                        Value::Number(r[j][i])
                                    );
                                }
                                self.compute(&mut std::iter::once(ast).peekable(), local_env.clone());
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
                            if let (Expr::Symbol(fn_name), Expr::List(fn_args), body_expr) = (&e[1], &e[2], &e[3..]) {
                                let function_name = fn_name.to_string();

                                let function_arguments: Vec<String> = fn_args.iter().filter_map(|arg| {
                                    if let Expr::Symbol(name) = arg {
                                        Some(name.clone())
                                    } else {
                                        None
                                    }
                                }).collect();

                                let mut annotations: Vec<Annotation> = Vec::new();
                                for expr in body_expr {
                                    if let Expr::List(dir) = expr {
                                        if let Expr::Symbol(name) = &dir[0] {
                                            if name.chars().next() == Some(':') {
                                                match name.as_str() {
                                                    ":require" =>
                                                        annotations.push(Annotation::Require(dir[1].clone())),
                                                    ":test" =>
                                                        {
                                                            let args = dir[1].clone();
                                                            if let Expr::List(args) = args {
                                                                annotations.push(
                                                                    Annotation::Test {
                                                                        args,
                                                                        expected: dir[2].clone(),
                                                                    }
                                                                )
                                                            }
                                                        }
                                                    _ => panic!("Unknown directive {}", name)
                                                }
                                            }
                                        }
                                    }
                                }

                                let function_body = Box::new(body_expr);

                                let captured_env = env.clone();
                                let function = Value::Function {
                                    params: function_arguments.clone(),
                                    body: Box::new(function_body.last().clone().unwrap().clone()),
                                    func_env: captured_env,
                                    annotations: annotations.clone(),
                                };

                                env.borrow_mut().variables.insert(function_name.clone(), function);

                                // :test
                                for annotation in annotations {
                                    if let Annotation::Test { args, expected } = annotation {
                                        let test_env = Rc::new(RefCell::new(Env {
                                            variables: Default::default(),
                                            parent: Some(env.clone()),
                                        }));
                                        for (arg, value) in function_arguments.iter().zip(args) {
                                            test_env.borrow_mut().variables.insert(arg.clone(), self.compute(&mut std::iter::once(&value).peekable(), env.clone()));
                                        }
                                        let test_result = self.compute(&mut std::iter::once(function_body.last().unwrap()).peekable(), test_env.clone());
                                        let expected_result = self.compute(&mut std::iter::once(&expected).peekable(), test_env.clone());
                                        match (test_result, expected_result) {
                                            (Value::Number(a), Value::Number(b)) if (a - b).abs() < 0.1 => { /* OK */ }
                                            (Value::Bool(a), Value::Bool(b)) if a == b => { /* OK */ }
                                            (Value::String(a), Value::String(b)) if a == b => { /* OK */ }
                                            _ => panic!("Function {} did not pass the test !", function_name)
                                        }
                                    }
                                }

                                result = Value::Nil;
                            } else {
                                panic!("Invalid function definition syntax");
                            }
                        },
                        Expr::Symbol(s) => {
                            if let Some(Value::NativeFunction(f)) = env.borrow().get(s) {
                                // STDLIB FUNCTION
                                let arg_values = args
                                    .into_iter()
                                    .map(|arg| self.compute(&mut std::iter::once(arg).peekable(), env.clone()))
                                    .collect::<Vec<Value>>();
                                match f {
                                    NativeFunction::Pure(fp) => result = fp(arg_values),
                                    NativeFunction::WithEnv(fwe) => result = fwe(arg_values, env.clone())
                                }
                            }else if let Some(Value::Function { params, body, func_env, annotations: _annotation }) = env.borrow().get(s) {
                                // USER DEFINED FUNCTION
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
                        _ => panic!("Invalid function definition syntax"),
                    }
                }
            }
        }
        result
    }

      ////////////////
     //  HANDLERS  //  -> When matching an Expr in self.compute
    ////////////////

    fn handle_number(&mut self, n: f32) -> Value {
        Value::Number(n)
    }

    fn handle_string(&mut self, s: String) -> Value {
        Value::String(String::from(s))
    }

    fn handle_array(&mut self, arr: Vec<Expr>, env: Rc<RefCell<Env>>) -> Value {
        let mut values: Vec<Value> = Vec::new();
        for a in arr {
            let mut i = std::iter::once(&a).peekable();
            let v = self.compute(&mut i, env.clone());
            values.push(v);
        }
        Value::Array(values)
    }

    fn handle_symbol(&mut self, s: String, env: Rc<RefCell<Env>>) -> Value {
        let env_rc = env.clone();
        let e = env_rc.borrow();
        if let Some(Value::NativeFunction(f)) = e.get(&*s) {
            return match f {
                NativeFunction::Pure(fp) => fp(vec![]),
                NativeFunction::WithEnv(fwe) => fwe(vec![], env.clone())
            }
        } else if let Some(_) = e.get(&*s) {
            let value = e.variables.get(&s).unwrap();
            return value.clone();
        }
        Value::Nil
    }
}