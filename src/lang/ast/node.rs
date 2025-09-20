use crate::lang::compiler::Compiler;
use alloc::boxed::*;
use alloc::string::String;
use alloc::vec::*;

trait Compile {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Compile>>;

    fn compile_children(&mut self, compiler: &mut Compiler) -> Result<(), String> {
        let children = self.get_children();
        for child in children.iter_mut() {
            if let Err(e) = child.compile(compiler) {
                return Err(e);
            }
            if let Err(e) = child.compile_children(compiler) {
                return Err(e);
            }
        }
        Ok(())
    }
    fn compile(&mut self, compiler: &mut Compiler) -> Result<(), String>;
}
