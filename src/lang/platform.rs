use crate::fs::global::get_fs;
use crate::io::get_string;
use alloc::format;
use alloc::string::*;
use alloc::vec::*;
use uefi::print;
use zenlang::module::Module;
use zenlang::platform;
use zenlang::stdlib::*;
use zenlang::value::*;
use zenlang::vm::*;

pub struct Platform {}

impl Platform {
    pub fn new() -> Self {
        return Self {};
    }
}

impl platform::Platform for Platform {
    fn print(&self, s: String) {
        print!("{}", s);
    }

    fn get_string(&self) -> String {
        return get_string();
    }
    fn get_module(&self, name: String) -> Option<Module> {
        if name == "stdlib" {
            let module = compile_stdlib_module();
            return Some(module);
        }
        if let Some(fs) = get_fs() {
            if let Ok(bytes) = fs.read_file(format!("/lib/{}.zenc", name)) {
                let mut module = Module::new();
                if let Err(_) = module.load(bytes) {
                    return None;
                }
                return Some(module);
            }
        }
        return None;
    }
    fn read_file_bytes(&self, _name: String) -> Option<Vec<u8>> {
        return None;
    }
    fn write_file_bytes(&self, _name: String, _bytes: Vec<u8>) {}
    fn vmcall(&self, vm: &mut VM, index: u8) -> bool {
        match index {
            50 => {
                // get max stack size
                vm.stack.push(Value::Number(MAX_STACK_SIZE as f64));
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}
