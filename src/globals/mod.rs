use core::ptr::{self};
use core::sync::atomic::{AtomicPtr, Ordering};
use uefi::prelude::*;

static SYSTEM_TABLE: AtomicPtr<SystemTable<Boot>> = AtomicPtr::new(ptr::null_mut());

pub fn get_system_table() -> SystemTable<Boot> {
    unsafe {
        return SYSTEM_TABLE.load(Ordering::Acquire).read();
    }
}

pub unsafe fn set_system_table(st: *mut SystemTable<Boot>) {
    SYSTEM_TABLE.store(st, Ordering::Release);
}
