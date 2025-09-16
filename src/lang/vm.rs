use alloc::string::String;
use alloc::vec::Vec;

pub enum Opcode {
    Return(String),
}

pub struct Function {
    name: String,
    code: Vec<Opcode>,
}

pub struct FileFormat {
    functions: Vec<Function>,
}
