use crate::lang::ast::node::Compile;
use alloc::string::*;
use alloc::vec::*;
use uefi_services::*;

pub struct AstFunction {
    pub children: Vec<alloc::boxed::Box<dyn Compile>>,
    pub name: String,
}

impl AstFunction {
    pub fn new() -> Self {
        return Self {
            children: Vec::new(),
            name: String::new(),
        };
    }
}

impl Compile for AstFunction {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return Some(&mut self.children);
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        {
            let module = compiler.get_module();
            let name = self.name.to_string();
            module.functions.push((name, module.opcodes.len() as u32));
        }
        Ok(())
    }
}
