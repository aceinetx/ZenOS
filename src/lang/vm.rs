use alloc::string::String;
use alloc::vec::Vec;

pub enum Opcode {}

pub struct Function {
    name: String,
    code: Vec<Opcode>,
}

pub struct Module {
    functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            functions: Vec::new(),
        };
    }
}

pub struct VM<'a> {
    stack: Vec<(&'a mut Function, usize)>,
    modules: Vec<&'a mut Module>,
}

impl<'a> VM<'_> {
    pub fn new() -> VM<'a> {
        return VM {
            stack: Vec::new(),
            modules: Vec::new(),
        };
    }

    pub fn load_module(module: Module) {}
}
