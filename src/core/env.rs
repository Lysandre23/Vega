use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::core::value::Value;

#[derive(Debug)]
pub struct Env {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
    pub classes: HashMap<String, Vec<String>>
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
    
    pub fn class_exists(&self, name: &str) -> Option<Vec<String>> {
        if let Some(val) = self.classes.get(name) {
            Some(val.clone())
        } else if let Some(parent) = &self.parent.clone() {
            parent.borrow().class_exists(name)
        } else {
            None
        }
    }
}