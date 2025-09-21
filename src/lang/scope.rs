use crate::lang::value::Value;
use alloc::string::*;
use alloc::vec::*;

pub struct Scope {
    vars: Vec<(String, Value)>,
}

impl Scope {
    pub fn new() -> Scope {
        return Scope { vars: Vec::new() };
    }

    pub fn get(& self, name: String) -> Option<&Value> {
        for var in self.vars.iter() {
            if var.0 == name {
                return Some(&var.1);
            }
        }
        return None;
    }

    pub fn get_mut(&mut self, name: String) -> Option<&mut Value> {
        for var in self.vars.iter_mut() {
            if var.0 == name {
                return Some(&mut var.1);
            }
        }
        return None;
    }
}

impl Default for Scope {
    fn default() -> Self {
        return Scope::new();
    }
}
