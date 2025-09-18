use crate::lang::tokenizer::*;
use crate::lang::vm;
use crate::lang::vm::*;
use alloc::vec::Vec;
use uefi_services::println;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer,
    module: vm::Module,
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

    pub fn parse_expression(&mut self) -> Result<BlockKind, &'static str> {
        self.tokenizer.next();
        self.tokenizer.next();

        let block = BlockKind::Return(BlockValue::Number(123.0));

        Ok(block)
    }

    pub fn parse_statement(&mut self) -> Result<BlockKind, &'static str> {
        let token = self.tokenizer.next();
        match token {
            Token::Return => {
                return self.parse_expression();
            }
            Token::Semicolon => {
                return Ok(BlockKind::BasicBlock());
            }
            _ => {
                println!("{:?}", token);
                return Err("unexpected token");
            }
        }
        //return Err("parse_statement did not parse any of the above statements");
    }

    pub fn parse_block(&mut self) -> Result<Vec<BlockKind>, &'static str> {
        let mut blocks = Vec::<BlockKind>::new();
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
        Ok(blocks)
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        if let Token::Identifier(name) = token {
            let func = BlockKind::Function { name: name };

            if !matches!(self.tokenizer.next(), Token::Lbrace) {
                return Err("expected `{` after fn");
            }

            let result = self.parse_block();
            if let Ok(blocks) = result {
                let parent_id = self.module.blocks.add_block(func, None);
                for block in blocks {
                    println!("add block {:?} to {}", block, parent_id);
                    self.module.blocks.add_block(block, Some(parent_id));
                }
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
