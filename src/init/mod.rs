use crate::ds::string::String;
use crate::lang::*;

use uefi_services::println;

pub fn zen_main() -> Result<(), &'static str> {
    println!();

    let code = String::from("fn main { return 123; }");
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut token = tokenizer.next();
    while matches!(token, tokenizer::Token::EOF) {
        println!("{:?}", token);
        token = tokenizer.next();
    }

    Ok(())
}
