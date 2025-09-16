use crate::mem::shared_alloc::*;

pub unsafe fn memcpy<T>(src: *mut T, dest: *mut T, n: usize) {
    unsafe {
        let mut current_chunk_size = 8;
        let mut i = 0;
        let mut remaining = n;
        loop {
            if remaining >= current_chunk_size {
                if current_chunk_size == 8 {
                    let src_u64 = (src as usize + i) as *mut u64;
                    let dest_u64 = (dest as usize + i) as *mut u64;
                    (*dest_u64) = *src_u64;
                } else if current_chunk_size == 4 {
                    let src_u64 = (src as usize + i) as *mut u32;
                    let dest_u64 = (dest as usize + i) as *mut u32;
                    (*dest_u64) = *src_u64;
                } else if current_chunk_size == 2 {
                    let src_u64 = (src as usize + i) as *mut u16;
                    let dest_u64 = (dest as usize + i) as *mut u16;
                    (*dest_u64) = *src_u64;
                } else if current_chunk_size == 1 {
                    let src_u64 = (src as usize + i) as *mut u8;
                    let dest_u64 = (dest as usize + i) as *mut u8;
                    (*dest_u64) = *src_u64;
                }
                remaining -= current_chunk_size;
                i += current_chunk_size;
            } else {
                if remaining > 4 {
                    current_chunk_size = 4;
                } else if remaining > 2 {
                    current_chunk_size = 2;
                } else if remaining >= 1 {
                    current_chunk_size = 1;
                } else if remaining == 0 {
                    break;
                }
            }
        }
    }
}

pub unsafe fn memset<T>(ptr: *mut T, byte: u8, size: usize) {
    unsafe {
        // todo: make this faster
        for i in 0..size {
            let p = (ptr as usize + i) as *mut u8;
            *p = byte;
        }
    }
}

pub unsafe fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    unsafe {
        let mut i = 0;
        while i < len {
            let av = *a.add(i);
            let bv = *b.add(i);
            if av != bv {
                // Cast to i32 to match C's `memcmp` sign convention.
                return av as i32 - bv as i32;
            }
            i += 1;
        }
        0
    }
}

pub unsafe fn memdup<T>(src: *mut T, size: usize) -> *mut T {
    unsafe {
        let new = alloc_bytes::<T>(size);
        memcpy(src, new, size);
        return new;
    }
}
