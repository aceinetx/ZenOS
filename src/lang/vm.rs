use crate::lang::module::Module;
use crate::lang::opcode::Opcode;
use crate::lang::register::Register;
use crate::lang::strong_u64::*;
use alloc::vec::Vec;

pub struct VM<'a> {
    modules: Vec<&'a mut Module>,
    pub pc: u64,
    pub registers: [u64; 127],
    pub stack: Vec<u64>,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        return VM {
            modules: Vec::new(),
            pc: 0,
            registers: [0; 127],
            stack: Vec::new(),
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
                    return Ok(());
                }
            }
        }
        return Err("cannot find entry function");
    }

    pub fn get_register(&self, reg: &Register) -> &u64 {
        match reg {
            Register::R(index) => {
                return &self.registers[*index as usize];
            }
        }
    }

    pub fn get_register_mut(&mut self, reg: &Register) -> &mut u64 {
        match reg {
            Register::R(index) => {
                return &mut self.registers[*index as usize];
            }
        }
    }

    fn execute_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Push(reg) => {
                self.stack.push(*self.get_register(reg));
            }
            Opcode::PushIMM(imm) => {
                self.stack.push(*imm);
            }
            Opcode::Mov(dest, src) => {
                *self.get_register_mut(dest) = *self.get_register(src);
            }
            Opcode::MovIMM(dest, imm) => {
                *self.get_register_mut(&dest) = *imm;
            }
            Opcode::Ret() => {
                if !self.stack.is_empty() {
                    self.pc = self.stack.pop().unwrap();
                } else {
                    self.pc.set_high(u32::MAX);
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
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
