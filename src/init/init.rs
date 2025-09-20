use crate::init::*;
use crate::mem::shared_alloc::*;
use crate::text;
use uefi_services::*;

pub fn init() -> Result<(), &'static str> {
    text::clear();

    println!("ZenOS");

    let allocator_start = 0x1000000;
    let allocator_size = 0x1000000;
    init_shared_allocator(allocator_start, allocator_size);

    if let Err(e) = main() {
        println!("error: main: {}", e);
    }

    println!("Checking for leaks on a shared allocator...");

    {
        let x = alloc::<i32>();
        free(x);
        if x as usize != allocator_start {
            println!(
                "! Memory leaked, last allocated memory address ({:x}) != {:x} (allocator_start)",
                x as usize, allocator_start
            );
        } else {
            println!("No leaks, shi's fine");
        }
    }

    loop {}
}
