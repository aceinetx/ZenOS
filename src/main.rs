#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

#[entry]
fn hello_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    print!("HI");

    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}
