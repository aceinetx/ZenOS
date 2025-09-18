use crate::lang::tokenizer::{Token, Tokenizer};
use crate::lang::vm;
use crate::lang::vm::*;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use uefi_services::println;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer,
    module: vm::Module,
}

impl<'a> Compiler<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Compiler<'a> {
        let mut inst = Compiler {
            tokenizer: tokenizer,
            module: Module::new(),
        };

        return inst;
    }

    pub fn get_module(&mut self) -> &Module {
        return &self.module;
    }

    pub fn get_bytes(&mut self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let cfg = bincode::config::standard();
        let bytes = bincode::encode_to_vec(&self.module, cfg);
        return bytes;
    }

    pub fn parse_expression(&mut self) -> Result<Block, &'static str> {
        self.tokenizer.next();
        self.tokenizer.next();

        Ok(Block::Return(BlockValue::Number(123.0)))
    }

    pub fn parse_statement(&mut self) -> Result<Block, &'static str> {
        let token = self.tokenizer.next();
        match token {
            Token::Return => {
                return self.parse_expression();
            }
            Token::Semicolon => {
                return Ok(Block::BasicBlock(Vec::new()));
            }
            _ => {
                println!("{:?}", token);
                return Err("unexpected token");
            }
        }
        //return Err("parse_statement did not parse any of the above statements");
    }

    pub fn parse_block(&mut self) -> Result<Block, &'static str> {
        let mut blocks = Vec::<Block>::new();
        loop {
            let result = self.parse_statement();
            if let Err(e) = result {
                return Err(e);
            } else if let Ok(inner) = result {
                blocks.push(inner);
            }

            let token = self.tokenizer.next();
            if matches!(token, Token::Rbrace) {
                break;
            }
        }
        Ok(Block::BasicBlock(blocks))
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        if let Token::Identifier(name) = token {
            let mut func = Function {
                name: name,
                block: Block::BasicBlock(Vec::new()),
            };

            if !matches!(self.tokenizer.next(), Token::Lbrace) {
                return Err("expected `{` after fn");
            }

            let result = self.parse_block();
            if let Ok(block) = result {
                func.block = block;
            }

            self.module.functions.push(func);
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
