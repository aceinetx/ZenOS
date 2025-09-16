use crate::lang::tokenizer::{Token, Tokenizer};
use alloc::string::String;
use alloc::vec::Vec;
use uefi_services::println;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer,
    bytes: Vec<u8>,
    header: Vec<u8>,
    symtab: Vec<u8>,
    text: Vec<u8>,
    data: Vec<u8>,
    symtab_addr: usize,
}

impl<'a> Compiler<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Compiler<'a> {
        let mut inst = Compiler {
            tokenizer: tokenizer,
            bytes: Vec::new(),
            header: Vec::new(),
            symtab: Vec::new(),
            text: Vec::new(),
            data: Vec::new(),
            symtab_addr: 0,
        };
        inst.add_header();

        return inst;
    }

    pub fn get_bytes(&mut self) -> &Vec<u8> {
        self.bytes.clear();
        self.bytes.append(&mut self.header);
        self.bytes.append(&mut self.symtab);
        self.bytes.append(&mut self.text);
        self.bytes.append(&mut self.data);
        return &self.bytes;
    }

    fn add_header(&mut self) {
        self.header
            .append(&mut String::from("ZEN").bytes().collect::<Vec<u8>>());
        self.header.push(1);

        self.symtab_addr = self.header.len();

        self.header
            .append(&mut self.symtab_addr.to_le_bytes().to_vec());
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        if let Token::Identifier(name) = token {
        } else {
            return Err("expected identifier after fn");
        }
        Ok(())
    }

    pub fn compile(&mut self) -> Result<(), &'static str> {
        let mut token = self.tokenizer.next();
        while !matches!(token, Token::EOF) {
            println!("tkn: {:?}", token);
            match token {
                Token::Fn => {
                    if let Err(e) = self.parse_function() {
                        return Err(e);
                    }
                }
                _ => {}
            }
            token = self.tokenizer.next();
        }
        Ok(())
    }
}
