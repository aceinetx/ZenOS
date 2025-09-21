use crate::lang::{ast::node::Compile, opcode::Opcode};
use alloc::string::*;
use alloc::vec::*;

pub struct AstVarRef {
    pub name: String,
}

impl AstVarRef {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
        };
    }
}

impl Compile for AstVarRef {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        let module = compiler.get_module();
        module.opcodes.push(Opcode::Loadv(self.name.clone()));
        Ok(())
    }
}
