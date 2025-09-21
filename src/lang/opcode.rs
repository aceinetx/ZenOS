use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Call(),
    Vmcall(u8),
    Loadcn(f64),
    Loadcs(String),
    Loadv(String),
    Storev(String),
    Pushret(),
    Bfas(), // Begin function arguments setup
    Efas(), // End function arguments setup
    Pop(),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
