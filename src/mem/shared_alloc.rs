use core::alloc::Layout;
use core::ptr::NonNull;
use linked_list_allocator::*;

use uefi_services::println;

static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn alloc_bytes<T>(size: usize) -> *mut T {
    let layout = Layout::from_size_align(size, core::mem::align_of::<usize>()).unwrap();
    let ptr = ALLOCATOR
        .lock()
        .allocate_first_fit(layout)
        .unwrap()
        .as_ptr();
    return ptr as *mut T;
}

pub fn alloc<T>() -> *mut T {
    return alloc_bytes::<T>(size_of::<T>());
}

pub fn free<T>(ptr: *mut T) {
    let size = size_of::<*mut T>();
    let layout = Layout::from_size_align(size, core::mem::align_of::<usize>()).unwrap();
    let p = NonNull::new(ptr as *mut u8).unwrap();
    unsafe {
        ALLOCATOR.lock().deallocate(p, layout);
    }
}

pub fn free_bytes<T>(ptr: *mut T, size: usize) {
    let layout = Layout::from_size_align(size, core::mem::align_of::<usize>()).unwrap();
    let p = NonNull::new(ptr as *mut u8).unwrap();
    unsafe {
        ALLOCATOR.lock().deallocate(p, layout);
    }
}

pub fn init_shared_allocator(start: usize, size: usize) {
    unsafe {
        ALLOCATOR.lock().init(start as *mut u8, size);
    }
}
