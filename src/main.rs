#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

use zen_os::globals;
use zen_os::init;
use zen_os::text;

#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    unsafe {
        globals::set_system_table(&mut system_table);
    };

    text::clear();
    if let Err(e) = init::zen_main() {
        println!("error: zen_main: {}", e);
    }

    loop {}
}
