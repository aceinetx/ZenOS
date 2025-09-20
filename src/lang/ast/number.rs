use crate::lang::ast::node::Compile;
use crate::lang::opcode::Opcode;
use crate::lang::register::Register;
use alloc::vec::*;

pub struct AstNumber {
    pub number: f64,
}

impl AstNumber {
    pub fn new() -> Self {
        return Self { number: 0.0 };
    }
}

impl Compile for AstNumber {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let opcode: Opcode;
        let register: u8;
        {
            register = compiler.get_new_register();
            opcode = Opcode::MovIMM(Register::R(register), self.number as u64);
        }

        {
            let module = compiler.get_module();
            module.opcodes.push(opcode);
        }

        {
            compiler.registers.push(register);
        }

        Ok(())
    }
}
