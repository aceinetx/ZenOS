use crate::lang::{compiler, parser, strong_u64::U64BitsControl, tokenizer, vm};
use alloc::string::*;
use uefi::*;

pub fn run_code(code: String) {
    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);
    if let Err(e) = compiler.compile() {
        println!("compilation error: {}", e);
        return;
    }

    let module = compiler.get_module();
    println!("{:?}", module);
    let mut vm = vm::VM::new();
    vm.load_module(module);
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
        println!(
            "runtime error at pc = {}: {}",
            vm.pc.get_low() - 1,
            vm.error
        );
        return;
    }
    println!("ret: {}", vm.ret);
}

pub fn main() -> Result<(), &'static str> {
    run_code(
        r"
fn double n {
    return n * 2;
}

fn main {
    let func = double;
    let n = func(5);
    func(4);
    return n;
} 
"
        .into(),
    );
    //kernel_interpreter();

    Ok(())
}
