use crate::io::get_string;
use crate::lang::ast::binop::AstBinopOp;
use crate::lang::module::Module;
use crate::lang::opcode::Opcode;
use crate::lang::scope::Scope;
use crate::lang::strong_u64::*;
use crate::lang::value::*;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;
use uefi::{print, println};

static MAX_STACK_SIZE: usize = 1000;

pub struct VM<'a> {
    pub modules: Vec<&'a mut Module>,
    pub pc: u64,
    pub stack: Vec<Value>,
    pub call_stack: Vec<u64>,
    pub scopes: Vec<Scope>,
    pub error: String,
    pub zero: u64,
    pub ret: Value,
    bfas_stack_start: i64,
    bfas_stack_end: i64,
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
            bfas_stack_start: 0,
            bfas_stack_end: 0,
        };
    }

    pub fn load_module(&mut self, module: &'a mut Module) {
        self.modules.push(module);
    }

    pub fn set_entry_function(&mut self, entry_fn_name: &str) -> Result<(), &'static str> {
        for i in 0..self.modules.len() {
            let module = &self.modules[i];
            for function in module.functions.iter() {
                if function.name == entry_fn_name {
                    self.pc.set_low(function.addr as u32);
                    self.pc.set_high(i as u32);
                    self.add_scope();
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

    pub fn check_stack_overflow(&mut self) {
        if self.call_stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
        if self.stack.len() >= MAX_STACK_SIZE {
            self.error = "call stack overflow".into();
        }
    }

    fn add_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn vmcall(&mut self, index: u8) {
        match index {
            1 => {
                if let Some(value) = self.stack.pop() {
                    if let Value::String(s) = value {
                        print!("{}", s);
                        return;
                    }
                }
                self.error = "vmcall: expected string in stack".into();
            }
            2 => {
                if let Some(value) = self.stack.pop() {
                    if let Value::String(s) = value {
                        println!("{}", s);
                        return;
                    }
                }
                self.error = "vmcall: expected string in stack".into();
            }
            3 => {
                let string = get_string();
                let value = Value::String(string);
                self.stack.push(value);
            }
            _ => {
                self.error = format!("vmcall: invalid vmcall index {}", index);
            }
        }
    }

    pub fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Call() => {
                if let Some(value) = self.stack.pop() {
                    if let Value::FunctionRef(addr, args_count) = value {
                        self.call_stack.push(self.pc);
                        self.check_stack_overflow();
                        self.pc = addr;
                        self.pc.sub_low(1);
                        self.add_scope();

                        let diff = self.bfas_stack_end - self.bfas_stack_start;
                        if diff != args_count as i64 {
                            self.error = format!(
                                "call: expected exactly {} arguments, but provided {}",
                                args_count, diff
                            );
                        }
                    } else {
                        self.error = "call: value on stack is not a function reference".into();
                    }
                } else {
                    self.error = "call: stack is empty".into();
                }
            }
            Opcode::Vmcall(index) => {
                self.vmcall(*index);
            }
            Opcode::Loadcn(value) => {
                let value = Value::Number(*value);
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::Loadcnu() => {
                self.stack.push(Value::Null());
                self.check_stack_overflow();
            }
            Opcode::Loadcs(value) => {
                let value = Value::String(value.to_string());
                self.stack.push(value);
                self.check_stack_overflow();
            }
            Opcode::Loadv(name) => {
                // do something with the clone here
                if let Some(scope) = self.scopes.last() {
                    if let Some(value) = scope.get(name) {
                        self.stack.push(value.clone());
                        self.check_stack_overflow();
                        return;
                    }
                }

                for module_i in 0..self.modules.len() {
                    let module = &self.modules[module_i];
                    for func in module.functions.iter() {
                        if func.name.to_string() == name.to_string() {
                            let mut addr: u64 = 0;
                            addr.set_low(func.addr);
                            addr.set_high(module_i as u32);
                            self.stack.push(Value::FunctionRef(addr, func.args_count));
                            self.check_stack_overflow();
                            return;
                        }
                    }
                }
                self.error = format!("unknown variable or function: {}", name);
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
                        return;
                    }
                } else {
                    self.error = format!("storev failed: no value in stack");
                    return;
                }
            }
            Opcode::Pushret() => {
                // do smth with the clone
                self.stack.push(self.ret.clone());
            }
            Opcode::Bfas() => {
                self.bfas_stack_start = self.stack.len() as i64;
            }
            Opcode::Efas() => {
                self.bfas_stack_end = self.stack.len() as i64;
            }
            Opcode::Pop() => {
                if self.stack.is_empty() {
                    self.error = format!("pop failed: no value in stack");
                } else {
                    self.stack.pop();
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

                self.remove_scope();

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

        let opcodes = core::mem::take(&mut self.modules[module_index].opcodes);
        if opcode_index >= opcodes.len() as u32 {
            return false;
        }

        let opcode = &opcodes[opcode_index as usize];

        self.execute_opcode(opcode);

        self.modules[module_index].opcodes = opcodes;

        self.pc.add_low(1);

        return true;
    }
}
