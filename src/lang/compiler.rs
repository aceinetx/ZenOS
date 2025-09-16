use crate::ds::string::String;
use crate::ds::vec::Vec;
use crate::lang::tokenizer::{Token, Tokenizer};
use uefi_services::println;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer,
    bytes: Vec<u8>,
}

impl<'a> Compiler<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Compiler<'a> {
        let mut inst = Compiler {
            tokenizer: tokenizer,
            bytes: Vec::new(),
        };
        inst.add_header();

        return inst;
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        return &self.bytes;
    }

    fn add_header(&mut self) {
        self.bytes.push_vec(&String::from("ZEN").bytes());
        self.bytes.push(1);
    }

    pub fn compile(&mut self) {
        let mut token = self.tokenizer.next();
        while !matches!(token, Token::EOF) {
            println!("tkn: {:?}", token);
            token = self.tokenizer.next();
        }
    }
}
