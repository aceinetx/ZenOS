use crate::lang::register::Register;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Push(Register),
    PushIMM(u64),
    Mov(Register, Register),
    MovIMM(Register, u64),
    Add(Register, Register),
    Sub(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Ret(),
}
