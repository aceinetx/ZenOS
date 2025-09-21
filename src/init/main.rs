use crate::lang::kernel_interpreter::kernel_interpreter;

pub fn main() -> Result<(), &'static str> {
    kernel_interpreter();

    Ok(())
}
