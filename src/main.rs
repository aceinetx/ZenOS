#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

use zen_os::globals;
use zen_os::init;
use zen_os::mem::alloc;
use zen_os::mem::shared_alloc;
use zen_os::text;

#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    unsafe {
        globals::set_system_table(&mut system_table);
    };

    // initialize allocator
    let allocator_addr = 0x1000000;
    let allocator_size = allocator_addr;
    let mut allocator = alloc::Allocator::new(allocator_addr, allocator_size);
    shared_alloc::init_shared_allocator(&mut allocator);

    text::clear();
    if let Err(e) = init::zen_main() {
        println!("error: zen_main: {}", e);
    }

    println!("Checking for leaks on a shared allocator...");
    allocator.leak_check();

    loop {}
}
