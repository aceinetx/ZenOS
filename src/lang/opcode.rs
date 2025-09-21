use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Call(),
    Vmcall(u8),
    Loadcn(f64), // load contant number
    Loadcnu(),
    Loadcs(String), // load constant string
    Loadv(String),  // load variable
    Storev(String), // store variable
    Pushret(),      // push the ret register
    Bfas(),         // Begin function arguments setup
    Efas(),         // End function arguments setup
    Pop(),          // pop from stack
    Add(),
    Sub(),
    Mul(),
    Div(),
    Ret(),
}
