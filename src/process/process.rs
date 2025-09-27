use crate::lang::Platform;
use alloc::boxed::*;
use zenlang::vm::*;

pub struct Process {
    pub vm: VM,
    pub pid: u64,
    pub stalling: bool,
}

impl Process {
    pub fn new() -> Process {
        let mut inst = Process {
            vm: VM::new(),
            pid: 0,
            stalling: false,
        };
        inst.vm.platform = Some(Box::new(Platform::new()));

        return inst;
    }
}
