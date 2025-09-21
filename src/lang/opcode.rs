use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Loadcn(f64),
    Loadv(String),
    Storev(String),
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
