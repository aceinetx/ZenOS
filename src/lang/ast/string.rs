use crate::lang::ast::node::Compile;
use crate::lang::opcode::Opcode;
use alloc::string::*;
use alloc::vec::*;

pub struct AstString {
    pub string: String,
    do_push: bool,
}

impl AstString {
    pub fn new() -> Self {
        return Self {
            string: String::new(),
            do_push: true,
        };
    }
}

impl Compile for AstString {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        if self.do_push {
            module.opcodes.push(Opcode::Loadcs(self.string.to_string()));
        }

        Ok(())
    }
}
