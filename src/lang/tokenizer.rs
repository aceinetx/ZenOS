use crate::ds::string::String;

#[derive(Debug)]
pub struct Tokenizer {
    code: String,
    pos: usize,
}

#[derive(Debug)]
pub enum Token {
    Fn,
    Return,
    Number(f64),
    Identifier(String),
    String(String),
    Operator(char),
    Lbrace,
    Rbrace,
    Semicolon,
    EOF,
}

impl Tokenizer {
    pub fn new(code: String) -> Tokenizer {
        return Tokenizer { code: code, pos: 0 };
    }

    fn is_digit(&self, ch: char) -> bool {
        return ch >= '0' && ch <= '9';
    }

    fn is_letter(&self, ch: char) -> bool {
        return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
    }

    fn is_identifier_letter(&self, ch: char) -> bool {
        return self.is_letter(ch) || ch == '_';
    }

    fn number(&mut self) -> Token {
        let mut num = 0.0;
        while self.pos < self.code.len() {
            let c = self.code[self.pos];
            if !self.is_digit(c) {
                break;
            }

            num *= 10.0;
            num += (c as u8 - '0' as u8) as f64;

            self.pos += 1;
        }

        return Token::Number(num);
    }

    fn identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while self.pos < self.code.len() {
            let c = self.code[self.pos];
            if !self.is_identifier_letter(c) {
                break;
            }

            identifier.push(c);

            self.pos += 1;
        }

        return Token::Identifier(identifier);
    }

    pub fn next(&mut self) -> Token {
        while self.pos < self.code.len() {
            let c = self.code[self.pos];
            if self.is_digit(c) {
                let token = self.number();
                return token;
            } else if self.is_letter(c) {
                let mut token = self.identifier();
                if let Token::Identifier(ref name) = token {
                    if name == "fn" {
                        token = Token::Fn;
                    } else if name == "return" {
                        token = Token::Return;
                    }
                }
                return token;
            } else if c == '{' {
                self.pos += 1;
                return Token::Lbrace;
            } else if c == '}' {
                self.pos += 1;
                return Token::Rbrace;
            } else if c == ';' {
                self.pos += 1;
                return Token::Semicolon;
            }
            self.pos += 1;
        }
        return Token::EOF;
    }
}
