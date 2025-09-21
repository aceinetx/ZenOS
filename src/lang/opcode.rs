use crate::lang::register::Register;
use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Loadcn(f64),
    Loadv(String),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
