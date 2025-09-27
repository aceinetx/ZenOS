use crate::fs::global::*;
use crate::fs::rom::*;
use crate::init::*;
use crate::mem::shared_alloc::*;
use crate::text;
use uefi::println;

pub fn init() -> Result<(), &'static str> {
    text::clear();

    println!("ZenOS");

    let allocator_start = 0x1000000;
    let allocator_size = 0x1000000;
    println!(
        "[init] Initializing shared allocator [0x{:x} - 0x{:x}]",
        allocator_start,
        allocator_start + allocator_size
    );
    init_shared_allocator(allocator_start, allocator_size);
    println!("[init] Shared allocator initalized");

    println!("[init] Initializing file system...");
    if let Err(e) = create_fs() {
        return Err(e);
    }

    println!("[init] Initializing default rom...");
    set_rom();

    println!("[init] Calling main procedure");
    if let Err(e) = main() {
        println!("[init] error: main: {}", e);
    }

    println!("[init] Destroying file system...");
    if let Err(e) = destroy_fs() {
        println!("[init] {}", e);
    }

    println!("[init] Checking for leaks on a shared allocator...");

    {
        let x = alloc::<i32>();
        free(x);
        if x as usize != allocator_start {
            panic!(
                "Memory leaked, last allocated memory address ({:x}) != {:x} (allocator_start)",
                x as usize, allocator_start
            );
        } else {
            println!("[init] No leaks, shi's fine");
        }
    }

    loop {}
}
