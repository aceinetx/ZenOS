use crate::lang::module::Module;
use crate::lang::opcode::Opcode;
use crate::lang::register::Register;
use crate::lang::scope::Scope;
use crate::lang::strong_u64::*;
use crate::lang::value::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;

pub struct VM<'a> {
    modules: Vec<&'a mut Module>,
    pub pc: u64,
    pub stack: Vec<Value>,
    pub call_stack: Vec<u64>,
    pub scopes: Vec<Scope>,
    pub error: String,
    pub zero: u64,
    pub ret: Value
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        return VM {
            modules: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            scopes: Vec::new(),
            error: String::new(),
            zero: 0,
            ret: Value::Number(0.0)
        };
    }

    pub fn load_module(&mut self, module: &'a mut Module) {
        self.modules.push(module);
    }

    pub fn set_entry_function(&mut self, entry_fn_name: &str) -> Result<(), &'static str> {
        for i in 0..self.modules.len() {
            let module = &self.modules[i];
            for function in module.functions.iter() {
                if function.0 == entry_fn_name {
                    self.pc.set_low(function.1 as u32);
                    self.pc.set_high(i as u32);
                    self.scopes.push(Scope::new());
                    return Ok(());
                }
            }
        }
        return Err("cannot find entry function");
    }

    fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Loadcn(value) => {
                let value = Value::Number(*value);
                self.stack.push(value);
            }
            Opcode::Loadv(name) => {
                // do something with the clone here
                if let Some(scope) = self.scopes.last() {
                    if let Some(value) = scope.get(name.to_string()){
                        self.stack.push(value.clone());
                    }
                }
            }
            Opcode::Add() => {
                let mut left: f64 = 0.;
                let mut right: f64 = 0.;

                if let Value::Number(num) = self.stack.pop().unwrap() {
                    right = num;
                }
                if let Value::Number(num) = self.stack.pop().unwrap() {
                    left = num;
                }

                self.stack.push(Value::Number(left + right));
            }
            Opcode::Sub() => {
                let mut left: f64 = 0.;
                let mut right: f64 = 0.;

                if let Value::Number(num) = self.stack.pop().unwrap() {
                    right = num;
                }
                if let Value::Number(num) = self.stack.pop().unwrap() {
                    left = num;
                }

                self.stack.push(Value::Number(left - right));
            }
            Opcode::Mul() => {
                let mut left: f64 = 0.;
                let mut right: f64 = 0.;

                if let Value::Number(num) = self.stack.pop().unwrap() {
                    right = num;
                }
                if let Value::Number(num) = self.stack.pop().unwrap() {
                    left = num;
                }

                self.stack.push(Value::Number(left * right));
            }
            Opcode::Div() => {
                let mut left: f64 = 0.;
                let mut right: f64 = 0.;

                if let Value::Number(num) = self.stack.pop().unwrap() {
                    right = num;
                }
                if let Value::Number(num) = self.stack.pop().unwrap() {
                    left = num;
                }

                self.stack.push(Value::Number(left / right));
            }
            Opcode::Ret() => {
                if !self.stack.is_empty(){
                    self.ret = self.stack.pop().unwrap();
                }

                if !self.call_stack.is_empty() {
                    self.pc = self.call_stack.pop().unwrap();
                } else {
                    self.pc.set_high(u32::MAX);
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
        self.zero = 0;
        if !self.error.is_empty() {
            return false;
        }

        let module_index = self.pc.get_high() as usize;
        let opcode_index = self.pc.get_low();
        if module_index >= self.modules.len() {
            return false;
        }

        let module = core::mem::take(self.modules[module_index]);
        if opcode_index >= module.opcodes.len() as u32 {
            return false;
        }

        let opcode = module.get_opcode(opcode_index);

        self.execute_opcode(opcode);

        *self.modules[module_index as usize] = module;

        self.pc.add_low(1);

        return true;
    }
}
