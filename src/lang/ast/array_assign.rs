use crate::lang::{ast::node::Compile, opcode::Opcode};
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

pub struct AstArrayAssign {
    pub name: String,
    pub index: Option<Box<dyn Compile>>,
    pub expr: Option<Box<dyn Compile>>,
}

impl AstArrayAssign {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
            index: None,
            expr: None,
        };
    }
}

impl Compile for AstArrayAssign {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if let Some(expr) = &mut self.expr {
            if let Err(e) = expr.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("expr is None".into());
        }
        if let Some(index) = &mut self.index {
            if let Err(e) = index.compile(compiler) {
                return Err(e);
            }
        } else {
            return Err("index is None".into());
        }

        let module = compiler.get_module();
        module.opcodes.push(Opcode::Aiafs(self.name.clone()));

        Ok(())
    }
}
