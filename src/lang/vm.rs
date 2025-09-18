use alloc::string::String;
use alloc::vec::Vec;
use bincode::*;

#[derive(Encode, Decode, Debug)]
pub enum BlockValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Encode, Decode, Debug)]
pub enum Block {
    Function { name: String, blocks: Vec<Block> },
    Return(BlockValue),
    BasicBlock(Vec<Block>),
}

#[derive(Encode, Decode, Debug)]
pub struct Function {
    pub name: String,
    pub block: Block,
}

#[derive(Encode, Decode, Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            functions: Vec::new(),
        };
    }
}

pub struct VM<'a> {
    modules: Vec<&'a mut Module>,
}

impl<'a> VM<'_> {
    pub fn new() -> VM<'a> {
        return VM {
            modules: Vec::new(),
        };
    }

    pub fn load_module(module: Module) {}
}
