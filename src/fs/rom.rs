use crate::fs::global::get_fs;
use alloc::string::*;
use uefi::*;
use zenlang::compiler;
use zenlang::parser;
use zenlang::tokenizer;

static SHELL_ZENLANG_CODE: &'static str = r#"
mod stdlib;
mod zenos;

fn execute line {
    if line == "" {
        return null;
    }
    let splitted = str_split(line, " ");

    let command = splitted[0];
    if command == "hello" {
        println("hi there!");
    } elif command == "ver" {
        ver();
    } else {
        println("unknown command");
    }
    return null;
}

fn main {
    while true {
        print("> ");
        let s = get_string();
        execute(s);
    }
    return null;
}
"#;
static ZENOS_ZENLANG_CODE: &'static str = r#"
mod stdlib;
fn ver {
    println("ZenOS nightly");
    println("ZenLang nightly");
    println("ZenLang constants:");
    print("MAX_STACK_SIZE = ");
    println(_vmcall_ret_unsafe_1(50));
    return null;
}
"#;

fn compile_code_into(code: String, module_name: String, out: String) {
    println!("[rom] compiling zenlang to {}", out);

    let mut tokenizer = tokenizer::Tokenizer::new(code);
    let mut parser = parser::Parser::new(&mut tokenizer);
    let mut compiler = compiler::Compiler::new(&mut parser);

    if let Err(e) = compiler.compile() {
        println!("[rom] compilation failed: {}", e);
        return;
    }

    for warning in compiler.warnings.iter() {
        println!("[rom] compilation warning: {}", warning);
    }

    let module = compiler.get_module();
    module.name = module_name;
    match module.compile() {
        Err(e) => {
            println!("[rom] module compilation error: {}", e);
            return;
        }
        Ok(bytes) => {
            //
            if let Some(fs) = get_fs() {
                if let Err(e) = fs.create_file(out.clone()) {
                    println!("[rom] /bin/shell.zenc: {}", e);
                    return;
                }
                if let Err(e) = fs.write_file(out, bytes) {
                    println!("[rom] write error: {}", e);
                    return;
                }
            }
        }
    }

    println!("[rom] compiled successfully");
}

pub fn set_rom() {
    if let Some(fs) = get_fs() {
        if let Err(e) = fs.create_directory("/lib".into()) {
            println!("[rom] /lib: {}", e);
        }
        if let Err(e) = fs.create_directory("/bin".into()) {
            println!("[rom] /bin: {}", e);
        }

        compile_code_into(
            SHELL_ZENLANG_CODE.into(),
            "zenshell".into(),
            "/bin/shell.zenc".into(),
        );
        compile_code_into(
            ZENOS_ZENLANG_CODE.into(),
            "zenos".into(),
            "/lib/zenos.zenc".into(),
        );
    }
}
