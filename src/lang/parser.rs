use crate::lang::ast::*;
use crate::lang::tokenizer::*;
use alloc::boxed::*;
use alloc::vec::*;
use uefi::println;

pub struct Parser<'a> {
    pub root: root::AstRoot,
    tokenizer: &'a mut Tokenizer,
}

impl<'a> Parser<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Parser<'a> {
        return Parser {
            root: root::AstRoot::new(),
            tokenizer: tokenizer,
        };
    }

    pub fn parse_expression(&mut self) -> Result<Box<dyn node::Compile>, &'static str> {
        self.tokenizer.next();
        self.tokenizer.next();

        //self.module.opcodes.push(Opcode::MovIMM(Register::R1, 123));
        let mut ret_ast = ret::AstReturn::new();
        let mut num = number::AstNumber::new();
        num.number = 123.0;
        ret_ast.value = Some(Box::new(num));

        Ok(Box::new(ret_ast))
    }

    pub fn parse_statement(&mut self) -> Result<Option<Box<dyn node::Compile>>, &'static str> {
        let token = self.tokenizer.next();
        match token {
            Token::Return => match self.parse_expression() {
                Err(e) => {
                    return Err(e);
                }
                Ok(node) => {
                    return Ok(Some(node));
                }
            },
            Token::Semicolon => {
                return Ok(None);
            }
            _ => {
                println!("{:?}", token);
                return Err("unexpected token");
            }
        }
        //return Err("parse_statement did not parse any of the above statements");
    }

    pub fn parse_block(&mut self) -> Result<Vec<Box<dyn node::Compile>>, &'static str> {
        let mut vec: Vec<Box<dyn node::Compile>> = Vec::new();

        loop {
            match self.parse_statement() {
                Err(e) => {
                    return Err(e);
                }
                Ok(node_option) => {
                    if let Some(node) = node_option {
                        vec.push(node);
                    }
                }
            }

            let token = self.tokenizer.next();
            if matches!(token, Token::Rbrace) {
                break;
            }
        }

        Ok(vec)
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.tokenizer.next();
        if let Token::Identifier(name) = token {
            let mut function = function::AstFunction::new();
            function.name = name;

            if !matches!(self.tokenizer.next(), Token::Lbrace) {
                return Err("expected `{` after fn");
            }

            match self.parse_block() {
                Err(e) => {
                    return Err(e);
                }
                Ok(nodes) => {
                    function.children = nodes;
                }
            }

            self.root.children.push(Box::new(function));
        } else {
            return Err("expected identifier after fn");
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), &'static str> {
        self.root = root::AstRoot::new();

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
