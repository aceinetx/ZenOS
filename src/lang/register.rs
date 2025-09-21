use alloc::string::*;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Register {
    R(u8),
    V(String),
}
