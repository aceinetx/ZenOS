use crate::lang::ast::*;
use crate::lang::tokenizer::*;
use alloc::boxed::*;
use alloc::vec::*;

pub struct Parser<'a> {
    pub root: root::AstRoot,
    tokenizer: &'a mut Tokenizer,
    current_token: Token,
}

impl<'a> Parser<'_> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Parser<'a> {
        return Parser {
            root: root::AstRoot::new(),
            tokenizer: tokenizer,
            current_token: Token::EOF,
        };
    }

    fn get_token_precedence(&mut self, token: &Token) -> Option<i32> {
        match *token {
            Token::Operator(op) => {
                if op == '+' {
                    return Some(1);
                } else if op == '-' {
                    return Some(1);
                } else if op == '*' {
                    return Some(2);
                } else if op == '/' {
                    return Some(2);
                }
                return None;
            }
            _ => {
                return None;
            }
        }
    }

    fn next(&mut self) -> Token {
        let token = self.tokenizer.next();
        self.current_token = token.clone();
        return token;
    }

    pub fn parse_expression(
        &mut self,
        min_prec: i32,
        initial: bool,
    ) -> Result<Box<dyn node::Compile>, &'static str> {
        let mut token;
        if initial {
            token = self.next();
        } else {
            token = self.current_token.clone();
        }

        let mut left: Box<dyn node::Compile>;

        match token {
            Token::Operator(_) => {
                let prec = self.get_token_precedence(&token).unwrap();

                self.next();
                match self.parse_expression(prec, false) {
                    Ok(node) => {
                        left = node;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Lbracket => {
                // parse array
                let mut node = array::AstArray::new();

                loop {
                    self.next();
                    if matches!(self.current_token, Token::Rbracket) {
                        break;
                    }

                    match self.parse_expression(0, false) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(expr) => {
                            node.values.push(expr);
                        }
                    }

                    if matches!(self.current_token, Token::Rbracket) {
                        break;
                    }
                    if !matches!(self.current_token, Token::Comma) {
                        return Err("expected `,` after array item: [ <ITEMS> [HERE] ]");
                    }
                }

                self.next();
                left = Box::new(node);
            }
            Token::Number(num) => {
                let mut node = number::AstNumber::new();
                node.number = num;
                left = Box::new(node);

                self.next();
            }
            Token::String(string) => {
                let mut node = string::AstString::new();
                node.string = string;
                left = Box::new(node);

                self.next();
            }
            Token::Lparen => {
                self.next();
                match self.parse_expression(0, false) {
                    Ok(node) => {
                        left = node;
                        token = self.current_token.clone();
                        if !matches!(token, Token::Rparen) {
                            return Err("expected `)`");
                        }
                        self.next();
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Identifier(name) => {
                // function call
                if matches!(self.tokenizer.peek(), Token::Lparen) {
                    self.next();

                    let mut node = func_call::AstFuncCall::new();
                    node.name = name;

                    loop {
                        self.next();
                        if matches!(self.current_token, Token::Rparen) {
                            break;
                        }

                        match self.parse_expression(0, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(expr) => {
                                node.args.push(expr);
                            }
                        }

                        token = self.current_token.clone();
                        if matches!(token, Token::Rparen) {
                            break;
                        }

                        if !matches!(token, Token::Comma) {
                            return Err(
                                "expected `,` after a function argument: CALL(<args> [HERE])",
                            );
                        }
                    }

                    left = Box::new(node);
                    self.next();
                } else {
                    let mut node = var_ref::AstVarRef::new();
                    node.name = name;
                    left = Box::new(node);

                    self.next();
                }
            }
            Token::Null => {
                let node = null::AstNull::new();
                left = Box::new(node);

                self.next();
            }
            _ => {
                return Err("unexpected token in parse_expression");
            }
        }

        loop {
            token = self.current_token.clone();
            if let Token::Operator(op) = token {
                match self.get_token_precedence(&token) {
                    Some(prec) => {
                        if prec < min_prec {
                            break;
                        }

                        // note to the future:
                        // right assoc: next_min = prec
                        // left assoc: next_min = prec + 1
                        let next_min = prec + 1;
                        match self.parse_expression(next_min, false) {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(right) => {
                                let mut binop = binop::AstBinop::new();
                                binop.left = Some(left);
                                binop.right = Some(right);
                                if op == '+' {
                                    binop.op = binop::AstBinopOp::PLUS;
                                } else if op == '-' {
                                    binop.op = binop::AstBinopOp::MINUS;
                                } else if op == '*' {
                                    binop.op = binop::AstBinopOp::MUL;
                                } else if op == '/' {
                                    binop.op = binop::AstBinopOp::DIV;
                                }
                                left = Box::new(binop);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            } else if let Token::Lbracket = token {
                match self.parse_expression(0, true) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(index) => {
                        let mut node = array_index::AstArrayIndex::new();
                        node.array = Some(left);
                        node.index = Some(index);
                        left = Box::new(node);
                    }
                }
                self.next();
            } else {
                break;
            }
        }

        Ok(left)
    }

    pub fn parse_statement(&mut self) -> Result<Option<Box<dyn node::Compile>>, &'static str> {
        let token = self.current_token.clone();

        match token {
            Token::Return => match self.parse_expression(0, true) {
                Err(e) => {
                    return Err(e);
                }
                Ok(node) => {
                    if !matches!(self.current_token, Token::Semicolon) {
                        return Err("expected semicolon after return");
                    }

                    let mut ret = ret::AstReturn::new();
                    ret.value = Some(node);
                    return Ok(Some(Box::new(ret)));
                }
            },
            Token::Let => {
                let mut node = var_assign::AstAssign::new();
                let name;

                if let Token::Identifier(ident_name) = self.next() {
                    name = ident_name;
                } else {
                    return Err("expected identifier after let");
                }

                self.next();
                if matches!(self.current_token, Token::Lbracket) {
                    let mut node = array_assign::AstArrayAssign::new();
                    node.name = name;

                    match self.parse_expression(0, true) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(index) => {
                            node.index = Some(index);
                        }
                    }

                    if !matches!(self.next(), Token::Assign) {
                        return Err("expected `=` after let <ident>[<index>]");
                    }

                    match self.parse_expression(0, true) {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(expr) => {
                            node.expr = Some(expr);
                        }
                    }

                    return Ok(Some(Box::new(node)));
                } else if !matches!(self.current_token, Token::Assign) {
                    return Err("expected `=` after let <ident>");
                }

                match self.parse_expression(0, true) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(expr) => {
                        if !matches!(self.current_token, Token::Semicolon) {
                            return Err("expected semicolon after return");
                        }

                        node.expr = Some(expr);
                        node.name = name;
                        return Ok(Some(Box::new(node)));
                    }
                }
            }
            Token::Semicolon => {
                return Ok(None);
            }
            _ => match self.parse_expression(0, false) {
                Err(e) => {
                    return Err(e);
                }
                Ok(mut expr) => {
                    expr.disable_push();

                    if !matches!(self.current_token, Token::Semicolon) {
                        return Err("expected semicolon after expression");
                    }

                    return Ok(Some(expr));
                }
            },
        }
        //return Err("parse_statement did not parse any of the above statements");
    }

    pub fn parse_block(&mut self) -> Result<Vec<Box<dyn node::Compile>>, &'static str> {
        let mut vec: Vec<Box<dyn node::Compile>> = Vec::new();

        loop {
            self.next();
            if matches!(self.current_token, Token::Rbrace) {
                break;
            }

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
        }

        Ok(vec)
    }

    pub fn parse_function(&mut self) -> Result<(), &'static str> {
        let token = self.next();
        if let Token::Identifier(name) = token {
            let mut function = function::AstFunction::new();
            function.name = name;

            loop {
                let token = self.next();
                if matches!(token, Token::Lbrace) {
                    break;
                }

                if let Token::Identifier(name) = token {
                    function.args.push(name);
                } else {
                    return Err("expected identifier in `fn <args> (HERE)`");
                }
            }

            //if !matches!(self.next(), Token::Lbrace) {
            //return Err("expected `{` after fn <args>");
            //}

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

        let mut token = self.next();
        while !matches!(token, Token::EOF) {
            //println!("tkn: {:?}", token);
            match token {
                Token::Fn => {
                    if let Err(e) = self.parse_function() {
                        return Err(e);
                    }
                }
                _ => {}
            }
            token = self.next();
        }
        Ok(())
    }
}
