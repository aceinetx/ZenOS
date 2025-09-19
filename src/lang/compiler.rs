use crate::lang::module::Module;
use crate::lang::opcode::Opcode;
use crate::lang::register::Register;
use crate::lang::tokenizer::*;
use alloc::string::String;
use alloc::vec::Vec;
use uefi_services::println;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer,
    module: Module,
}

impl<'a> Compiler<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Compiler<'a> {
        let inst = Compiler {
            tokenizer: tokenizer,
            module: Module::new(),
        };

        return inst;
    }

    pub fn get_module(&mut self) -> &mut Module {
        return &mut self.module;
    }

    pub fn parse_expression(&mut self) -> Result<(), &'static str> {
        self.tokenizer.next();
        self.tokenizer.next();

        self.module.opcodes.push(Opcode::MovIMM(Register::R1, 123));

        Ok(())
    }

    pub fn parse_statement(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        match token {
            Token::Return => {
                if let Err(e) = self.parse_expression() {
                    return Err(e);
                }
                self.module.opcodes.push(Opcode::Ret());
                return Ok(());
            }
            Token::Semicolon => {
                return Ok(());
            }
            _ => {
                println!("{:?}", token);
                return Err("unexpected token");
            }
        }
        //return Err("parse_statement did not parse any of the above statements");
    }

    pub fn parse_block(&mut self) -> Result<(), &'static str> {
        loop {
            let result = self.parse_statement();
            if let Err(e) = result {
                return Err(e);
            }

            let token = self.tokenizer.next();
            if matches!(token, Token::Rbrace) {
                break;
            }
        }
        Ok(())
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        if let Token::Identifier(name) = token {
            let function: (String, u32) = (name, self.module.opcodes.len() as u32);
            self.module.functions.push(function);

            if !matches!(self.tokenizer.next(), Token::Lbrace) {
                return Err("expected `{` after fn");
            }

            let result = self.parse_block();
            if let Err(e) = result {
                return Err(e);
            }
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
