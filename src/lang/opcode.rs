use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Call(),
    Loadcn(f64),
    Loadv(String),
    Storev(String),
    Pushret(),
    Pop(),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
