use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Register {
    R(u8),
}
