use crate::lang::*;
use alloc::string::String;
use uefi_services::*;

pub fn zen_main() -> Result<(), &'static str> {
    println!();

    {
        let code = String::from("fn main { return 123; }");
        let mut tokenizer = tokenizer::Tokenizer::new(code);
        let mut compiler = compiler::Compiler::new(&mut tokenizer);
        if let Err(e) = compiler.compile() {
            println!("compilation error: {}", e);
        }

        let module = compiler.get_module();
        //module.debug_bytes();

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
        println!("r1: {}", vm.r1);
    }

    Ok(())
}
