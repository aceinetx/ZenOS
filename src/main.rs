#![no_main]
#![no_std]

use core::panic::*;
use uefi::prelude::*;
use uefi::*;
use zen_os::init;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
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
