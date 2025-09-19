use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
}
