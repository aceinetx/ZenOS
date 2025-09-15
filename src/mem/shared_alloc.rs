use crate::mem::alloc;

use core::ptr::{self};
use core::sync::atomic::{AtomicPtr, Ordering};
use uefi::prelude::*;

static SHARED_ALLOCATOR: AtomicPtr<alloc::Allocator> = AtomicPtr::new(ptr::null_mut());

pub fn alloc<T>() -> *mut T {
    unsafe {
        return SHARED_ALLOCATOR
            .load(Ordering::Acquire)
            .as_mut()
            .unwrap()
            .alloc();
    }
}

pub fn free<T>(ptr: *mut T) {
    unsafe {
        return SHARED_ALLOCATOR
            .load(Ordering::Acquire)
            .as_mut()
            .unwrap()
            .free(ptr);
    }
}

pub fn realloc<T>(ptr: *mut T, size: usize) -> *mut T {
    unsafe {
        return SHARED_ALLOCATOR
            .load(Ordering::Acquire)
            .as_mut()
            .unwrap()
            .realloc(ptr, size);
    }
}

pub fn init_shared_allocator(allocator: *mut alloc::Allocator) {
    SHARED_ALLOCATOR.store(allocator, Ordering::Release);
}
