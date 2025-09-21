use crate::lang::kernel_interpreter::kernel_interpreter;
use crate::lang::*;
use alloc::string::String;
use uefi::println;

pub fn main() -> Result<(), &'static str> {
    println!();

    kernel_interpreter();

    Ok(())
}
