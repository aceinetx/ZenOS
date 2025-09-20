use crate::lang::ast::node::Compile;
use alloc::vec::*;

pub enum AstBinopOp {
    PLUS,
    MINUS,
    MUL,
    DIV,
}

pub struct AstBinop {
    pub a: Option<alloc::boxed::Box<dyn Compile>>,
    pub b: Option<alloc::boxed::Box<dyn Compile>>,
    pub op: AstBinopOp,
}

impl AstBinop {
    pub fn new() -> Self {
        return Self {
            a: None,
            b: None,
            op: AstBinopOp::PLUS,
        };
    }
}

impl Compile for AstBinop {
    fn get_children(&mut self) -> Option<&mut Vec<alloc::boxed::Box<dyn Compile>>> {
        return None;
    }

    fn compile(
        &mut self,
        _compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        Ok(())
    }
}
