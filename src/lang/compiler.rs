use crate::lang::ast::node::Compile;
use crate::lang::module::Module;
use crate::lang::parser::*;
use alloc::string::*;
use alloc::vec::*;

pub struct Compiler<'a> {
    parser: &'a mut Parser<'a>,
    module: Module,
    pub registers: Vec<u8>,
    register_index: u8,
}

impl<'a> Compiler<'_> {
    pub fn new(parser: &'a mut Parser<'a>) -> Compiler<'a> {
        let inst = Compiler {
            parser: parser,
            module: Module::new(),
            registers: Vec::new(),
            register_index: 0,
        };

        return inst;
    }

    pub fn get_new_register(&mut self) -> u8 {
        self.register_index += 1;
        if self.register_index > 127 {
            self.register_index = 0;
        }

        return self.register_index;
    }

    pub fn get_module(&mut self) -> &mut Module {
        return &mut self.module;
    }

    pub fn compile(&mut self) -> Result<(), String> {
        if let Err(e) = self.parser.parse() {
            return Err(e.into());
        }

        let mut root = core::mem::take(&mut self.parser.root);
        let result = root.compile_all(self);

        self.parser.root = root;

        return result;
    }
}
