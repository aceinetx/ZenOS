use crate::io::get_string;
use alloc::string::*;
use uefi::print;
use zenlang::module::Module;
use zenlang::platform;
use zenlang::stdlib::*;

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
        return None;
    }
}
