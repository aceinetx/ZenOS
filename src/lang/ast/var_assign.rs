use crate::lang::ast::node::Compile;
use alloc::string::String;
use alloc::vec::*;

pub struct AstAssign {
    pub name: String,
    pub expr: Option<alloc::boxed::Box<dyn Compile>>,
}

impl AstAssign {
    pub fn new() -> Self {
        return Self {
            name: String::new(),
            expr: None,
        };
    }
}

impl Compile for AstAssign {
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

        let _module = compiler.get_module();

        Ok(())
    }
}
