use crate::lang::register::Register;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Opcode {
    Push(Register),
    PushIMM(u64),
    Mov(Register, Register),
    MovIMM(Register, u64),
    Ret(),
}
