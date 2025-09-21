use crate::lang::*;
use alloc::string::String;
use uefi::println;

pub fn main() -> Result<(), &'static str> {
    println!();

    {
        let code = String::from("fn main { return 2 + 2 / 2; }");
        let mut tokenizer = tokenizer::Tokenizer::new(code);
        let mut parser = parser::Parser::new(&mut tokenizer);
        let mut compiler = compiler::Compiler::new(&mut parser);
        if let Err(e) = compiler.compile() {
            println!("compilation error: {}", e);
            return Ok(());
        }

        let module = compiler.get_module();
        //module.debug_bytes();
        println!("{:?}", module);

        let mut vm = vm::VM::new();
        vm.load_module(module);
        if let Err(e) = vm.set_entry_function("main") {
            println!("{}", e);
            return Ok(());
        }

        loop {
            println!("pc: {}", vm.pc);
            if !vm.step() {
                break;
            }
        }
        println!("ret: {:?}", vm.ret);
    }

    Ok(())
}
