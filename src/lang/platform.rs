use crate::io::get_string;
use alloc::string::*;
use uefi::print;
use zenlang::platform;

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
}
