use crate::lang::Platform;
use alloc::boxed::*;
use alloc::string::*;
use uefi::*;
use zenlang::{compiler, parser, strong_u64::U64BitsControl, tokenizer, vm};

pub fn run_code(code: String) {
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);

    // compile the code
    if let Err(e) = compiler.compile() {
        println!("compilation error: {}", e);
        return;
    }

    // get modules
    let module = compiler.get_module();

    // create VM
    let mut vm = vm::VM::new();
    // set the platform
    vm.platform = Some(Box::new(Platform::new()));

    // load modules
    if let Err(e) = vm.load_module(module) {
        println!("{}", e);
        return;
    }

    if let Err(e) = vm.set_entry_function("main") {
        println!("unresolved symbol main: {}", e);
        return;
    }

    loop {
        if !vm.step() {
            break;
        }
    }
    if !vm.error.is_empty() {
        println!("\n-- begin runtime error --");
        println!(
            "runtime error at pc = {}: {}",
            vm.pc.get_low() - 1,
            vm.error
        );
        println!("-- end runtime error --");
        return;
    }
    println!("returned {}", vm.ret);
}

pub fn main() -> Result<(), &'static str> {
    run_code(
        r#"
mod stdlib;
fn main {
    return println;
}
"#
        .into(),
    );

    Ok(())
}
