use crate::lang::{
    compiler, parser, stdlib::compile_stdlib_module, strong_u64::U64BitsControl, tokenizer, vm,
};
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
    let mut stdlib = compile_stdlib_module();

    let mut vm = vm::VM::new();
    vm.load_module(module);
    vm.load_module(&mut stdlib);

    //println!("{:?}", vm.modules);

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
    println!("ret: {}", vm.ret);
}

pub fn main() -> Result<(), &'static str> {
    run_code(
        r#"
fn main {
    return Null;
} 
"#
        .into(),
    );
    //kernel_interpreter();

    Ok(())
}
