use crate::ds::string::String;
use crate::lang::*;

use uefi_services::println;

pub fn zen_main() -> Result<(), &'static str> {
    println!();

    let code = String::from("fn main { return 123; }");
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut compiler = compiler::Compiler::new(&mut tokenizer);
    compiler.compile();
    let bytes = compiler.get_bytes();
    for i in 0..bytes.len() {
        println!("{:x}     {}", bytes[i], bytes[i] as char);
    }

    Ok(())
}
