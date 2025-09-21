use crate::lang::ast::binop::AstBinopOp;
use crate::lang::module::Module;
use crate::lang::opcode::Opcode;
use crate::lang::scope::Scope;
use crate::lang::strong_u64::*;
use crate::lang::value::*;
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
    pub ret: Value,
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
            ret: Value::Null(),
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

    fn compute_values(&mut self, left: Value, right: Value, op: AstBinopOp) -> Value {
        match op {
            AstBinopOp::PLUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num + right_num);
                    }
                }
            }
            AstBinopOp::MINUS => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num - right_num);
                    }
                }
            }
            AstBinopOp::MUL => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num * right_num);
                    }
                }
            }
            AstBinopOp::DIV => {
                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {
                        return Value::Number(left_num / right_num);
                    }
                }
            }
        }
        self.error = "unmatched left and right value types".into();
        return Value::Null();
    }

    fn compute_stack_values(&mut self, op: AstBinopOp) -> Value {
        let mut left = Value::Null();
        let mut right = Value::Null();

        if let Some(value) = self.stack.pop() {
            right = value;
        }
        if let Some(value) = self.stack.pop() {
            left = value;
        }

        return self.compute_values(left, right, op);
    }

    pub fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Loadcn(value) => {
                let value = Value::Number(*value);
                self.stack.push(value);
            }
            Opcode::Loadv(name) => {
                // do something with the clone here
                if let Some(scope) = self.scopes.last() {
                    if let Some(value) = scope.get(name) {
                        self.stack.push(value.clone());
                        return;
                    }
                }
                self.error = format!("unknown variable: {}", name);
            }
            Opcode::Storev(name) => {
                // do something with the clone here
                if let Some(store_value) = self.stack.pop() {
                    if let Some(scope) = self.scopes.last_mut() {
                        scope.create_if_doesnt_exist(name);
                        if let Some(value) = scope.get_mut(name) {
                            *value = store_value;
                            return;
                        }
                    } else {
                        self.error = format!("storev failed: scopes is empty");
                    }
                } else {
                    self.error = format!("storev failed: no value in stack");
                }
            }
            Opcode::Add() => {
                let value = self.compute_stack_values(AstBinopOp::PLUS);
                self.stack.push(value);
            }
            Opcode::Sub() => {
                let value = self.compute_stack_values(AstBinopOp::MINUS);
                self.stack.push(value);
            }
            Opcode::Mul() => {
                let value = self.compute_stack_values(AstBinopOp::MUL);
                self.stack.push(value);
            }
            Opcode::Div() => {
                let value = self.compute_stack_values(AstBinopOp::DIV);
                self.stack.push(value);
            }
            Opcode::Ret() => {
                if !self.stack.is_empty() {
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
