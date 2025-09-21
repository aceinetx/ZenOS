use crate::lang::register::Register;
use crate::lang::{ast::node::Compile, opcode::Opcode};
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
        compiler: &mut crate::lang::compiler::Compiler,
    ) -> Result<(), alloc::string::String> {
        if let Some(left) = &mut self.a {
            if let Err(e) = left.compile(compiler) {
                return Err(e);
            }
        }
        if let Some(right) = &mut self.b {
            if let Err(e) = right.compile(compiler) {
                return Err(e);
            }
        }

        let opcode;
        let right = Register::R(compiler.registers.pop().unwrap());
        let left_raw = compiler.registers.pop().unwrap();
        let left = Register::R(left_raw);

        compiler.registers.push(left_raw);
        match self.op {
            AstBinopOp::PLUS => {
                opcode = Opcode::Add(left, right);
            }
            AstBinopOp::MINUS => {
                opcode = Opcode::Sub(left, right);
            }
            AstBinopOp::MUL => {
                opcode = Opcode::Mul(left, right);
            }
            AstBinopOp::DIV => {
                opcode = Opcode::Div(left, right);
            }
        }
        {
            let module = compiler.get_module();
            module.opcodes.push(opcode);
        }
        Ok(())
    }
}
