#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

use zen_os::globals;
use zen_os::init;
use zen_os::mem::shared_alloc::*;
use zen_os::text;

#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    unsafe {
        globals::set_system_table(&mut system_table);
    };

    // initialize allocator
    //let mut allocator = alloc::Allocator::new(0x1000000, 64, 500);
    //shared_alloc::init_shared_allocator(&mut allocator);
    let allocator_start = 0x1000000;
    let allocator_size = 0x1000000;
    init_shared_allocator(allocator_start, allocator_size);

    text::clear();
    if let Err(e) = init::zen_main() {
        println!("error: zen_main: {}", e);
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
