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
        if let Ok(bytes) = module.compile() {
            for i in 0..bytes.len() {
                let byte = bytes[i];
                print!("{:x}     ", byte);
                if byte != 0 {
                    print!("{}", byte as char);
                }
                println!();
            }
        }

        let mut vm = vm::VM::new();
        vm.load_module(module);
        if let Err(e) = vm.set_entry_function("main".into()) {
            println!("{}", e);
            return Ok(());
        }

        loop {
            println!("pc: {} mpc: {}", vm.get_pc(), vm.get_module_pc());
            if !vm.step() {
                break;
            }
        }
        println!("ret: {}", vm.get_return_value());
    }

    Ok(())
}
