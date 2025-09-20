#![no_main]
#![no_std]
#![allow(unused_imports)]

use uefi::prelude::*;
use uefi::*;
use zen_os::init;

use core::panic::*;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("[panic] -- panic begin --");
    match info.location(){
        Some(location) => {
            println!("[panic] location: {}[{}:{}]", location.file(), location.line(), location.column());
        }
        None => {
            println!("[panic] location: unavailable");
        }
    }
    println!("[panic] message: {}", info.message());
    println!("[panic] -- panic end --");
    loop {}
}

#[entry]
fn efi_main() -> Status {
    let _ = uefi::helpers::init();

    if let Err(e) = init::init() {
        println!("init fail: {}", e);
    }

    return Status::SUCCESS;
}
