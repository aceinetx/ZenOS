use crate::fs::global::get_fs;
use crate::process::*;
use alloc::string::*;
use uefi::println;
use zenlang::module::Module;

pub fn run_zenc_process_manager(
    process_manager: &mut ProcessManager,
    path: &str,
) -> Result<(), String> {
    if let Some(fs) = get_fs() {
        match fs.read_file(path.into()) {
            Ok(bytes) => {
                let mut module = Module::new();
                if let Err(e) = module.load(bytes) {
                    return Err(e.to_string());
                }

                let mut process = Process::new();
                if let Err(e) = process.vm.load_module(&module) {
                    return Err(e);
                }

                if let Err(e) = process.vm.set_entry_function("main") {
                    return Err(e.into());
                }

                process_manager.append_process(process);
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    } else {
        return Err("get_fs failed".into());
    }
}

pub fn main() -> Result<(), String> {
    let mut process_manager = ProcessManager::new();

    if let Err(e) = run_zenc_process_manager(&mut process_manager, "/bin/shell.zenc") {
        return Err(e);
    }
    println!("[main] shell loading successful");

    // step all processes
    loop {
        process_manager.step_all();
        process_manager.remove_stalling_processes();
    }
}
