use crate::lang::ast::node::Compile;
use crate::lang::opcode::Opcode;
use crate::lang::register::Register;
use alloc::vec::*;

pub struct AstReturn {
    pub value: Option<alloc::boxed::Box<dyn Compile>>,
}

impl AstReturn {
    pub fn new() -> Self {
        return Self { value: None };
    }
}

impl Compile for AstReturn {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        match &mut self.value {
            None => {
                return Err("self.value is None".into());
            }
            Some(value) => {
                if let Err(e) = value.compile(compiler) {
                    return Err(e);
                }

                let register: u8;
                {
                    register = compiler.registers.pop().unwrap();
                }

                {
                    let module = compiler.get_module();
                    module
                        .opcodes
                        .push(Opcode::Mov(Register::R(0), Register::R(register)));
                    module.opcodes.push(Opcode::Ret());
                }
            }
        }

        Ok(())
    }
}
