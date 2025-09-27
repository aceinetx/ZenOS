use crate::process::Process;
use alloc::vec::*;
use uefi::println;
use zenlang::strong_u64::U64BitsControl;

pub struct ProcessManager {
    pub(crate) next_pid: u64,
    pub processes: Vec<Process>,
}

impl ProcessManager {
    pub fn new() -> ProcessManager {
        return ProcessManager {
            next_pid: 0,
            processes: Vec::new(),
        };
    }

    pub fn append_process(&mut self, mut process: Process) {
        self.next_pid += 1;
        process.pid = self.next_pid;
        self.processes.push(process);
    }

    pub fn step_all(&mut self) {
        for process in self.processes.iter_mut() {
            if process.stalling {
                continue;
            }

            if !process.vm.step() {
                process.stalling = true;

                if !process.vm.error.is_empty() {
                    let mut pc = process.vm.pc;
                    pc.sub_low(1);

                    println!("-- begin runtime error --");
                    println!("pid: {}", process.pid);
                    println!("{}", process.vm.error);
                    if let Some(name) = process.vm.get_function_name_from_pc(pc) {
                        println!("runtime error in function {}", name,);
                    }
                    println!("runtime error at pc = {}:{}", pc.get_low(), pc.get_high(),);
                    println!("-- end runtime error --");
                } else {
                    println!("pid {} returned {}", process.pid, process.vm.ret);
                }
            }
        }
    }

    pub fn remove_stalling_processes(&mut self) {
        let mut i: usize = self.processes.len();
        while i > 0 {
            i -= 1;
            if self.processes[i].stalling {
                self.processes.remove(i);
            }
        }
    }
}
