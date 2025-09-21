use crate::io::*;
use crate::lang::opcode::Opcode;
use crate::lang::{
    compiler::Compiler, parser::Parser, scope::Scope, strong_u64::*, tokenizer::Tokenizer, vm::VM,
};
use alloc::format;
use uefi::{print, println};

pub fn kernel_interpreter() {
    let mut vm = VM::new();

    vm.pc.set_high(0);
    vm.pc.set_low(0);
    vm.scopes.push(Scope::new());

    loop {
        print!("zenlang (kernel) > ");
        let mut code = get_string();
        code = format!("fn main {{ {} }}", code);

        // compile
        let mut tokenizer = Tokenizer::new(code);
        let mut parser = Parser::new(&mut tokenizer);
        let mut compiler = Compiler::new(&mut parser);
        if let Err(e) = compiler.compile() {
            println!("compilation error: {}", e);
            continue;
        }

        let module = compiler.get_module();

        vm.error.clear();
        for opcode in module.opcodes.iter() {
            if matches!(opcode, Opcode::Ret()) {
                println!("returned {}", vm.stack.pop().unwrap());
                break;
            }

            vm.execute_opcode(opcode);
            if !vm.error.is_empty() {
                println!("runtime error: {}", vm.error);
                break;
            }
        }
    }
}
