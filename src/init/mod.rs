use crate::lang::*;
use alloc::string::String;
use uefi_services::*;

pub fn zen_main() -> Result<(), &'static str> {
    println!();

    let code = String::from("fn main { return 123; }");
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut compiler = compiler::Compiler::new(&mut tokenizer);
    if let Err(e) = compiler.compile() {
        println!("compilation error: {}", e);
    }

    let bytes = compiler.get_bytes();
    for i in 0..bytes.len() {
        let byte = bytes[i];
        print!("{:x}     ", byte);
        if byte != 0 {
            print!("{}", byte as char);
        }
        println!();
    }

    Ok(())
}
