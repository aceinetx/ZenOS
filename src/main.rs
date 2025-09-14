#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

pub mod globals;
pub mod text;

#[entry]
fn hello_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    unsafe {
        globals::set_system_table(&mut system_table);
    };

    let mut st = globals::get_system_table();

    text::clear();
    text::set_char(10, 10, 'a');

    st.boot_services().stall(10_000_000);

    Status::SUCCESS
}
