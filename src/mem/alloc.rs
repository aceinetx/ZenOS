use crate::mem;
use uefi_services::println;

#[derive(Debug)]
#[repr(C)]
struct BlockHeader {
    size: usize,
}

#[derive(Debug)]
#[repr(C)]
struct BlockFooter {
    size: usize,
}

#[derive(Debug)]
#[repr(C)]
struct FreeBlock {
    header: BlockHeader,
    next: *mut FreeBlock,
    prev: *mut FreeBlock,
    // Footer is at the end of the block
}

#[derive(Debug)]
#[repr(C)]
pub struct Allocator {
    start: *mut (),
    end: *mut (),
    free_list_head: *mut FreeBlock,
    alloc_diff: i32, // allocation difference, increment on alloc, decrement on free
}

const ALLOCATED: usize = 1;

impl Allocator {
    pub fn new(start: usize, size: usize) -> Allocator {
        let inst = Allocator {
            start: start as *mut (),
            end: (start + size) as *mut (),
            free_list_head: start as *mut FreeBlock,
            alloc_diff: 0,
        };
        unsafe {
            (*(start as *mut BlockHeader)).size = size;
            (*((start + size - size_of::<BlockFooter>()) as *mut BlockFooter)).size = size;
            (*inst.free_list_head).next = 0 as *mut FreeBlock;
            (*inst.free_list_head).prev = 0 as *mut FreeBlock;
        }

        return inst;
    }

    fn align(&mut self, size: usize, align: usize) -> usize {
        (size + align - 1) & !(align - 1)
    }

    fn inc_diff(&mut self) {
        self.alloc_diff += 1;
        //println!("increment alloc_diff, now {}", self.alloc_diff);
    }

    fn dec_diff(&mut self) {
        self.alloc_diff -= 1;
        //println!("decrement alloc_diff, now {}", self.alloc_diff);
    }

    pub fn alloc<T>(&mut self) -> *mut T {
        self.alloc_raw(size_of::<T>())
    }

    pub fn alloc_raw<T>(&mut self, _size: usize) -> *mut T {
        unsafe {
            // Align to 8 bytes
            let size = self.align(_size, 8);
            println!("requested alloc of {} (aligned) bytes", _size);
            let total_needed = size_of::<BlockHeader>() + size + size_of::<BlockFooter>();
            let mut best: *mut FreeBlock = 0 as *mut FreeBlock;
            let mut curr: *mut FreeBlock = self.free_list_head;
            loop {
                if curr == 0 as *mut FreeBlock {
                    break;
                }

                let curr_size = (*curr).header.size & !ALLOCATED;
                //println!("{:?} {:x} {}", (*curr), best as usize, total_needed);
                if curr_size >= total_needed
                    && ((best as usize == 0) || curr_size < (*best).header.size)
                {
                    best = curr;
                }
                curr = (*curr).next;
            }

            if best as usize == 0 {
                println!("alloc failed: no best block");
                return 0 as *mut T;
            }

            let remaining = ((*best).header.size & !ALLOCATED) - total_needed;
            if remaining >= size_of::<BlockHeader>() + size_of::<BlockFooter>() + 8 {
                // Split the block
                let new_header = (best as usize + total_needed) as *mut BlockHeader;
                (*new_header).size = remaining | 8;
                let new_footer = (new_header as usize + remaining - size_of::<BlockFooter>())
                    as *mut BlockFooter;
                (*new_footer).size = remaining;

                let new_block = new_header as *mut FreeBlock;
                (*new_block).next = (*best).next;
                (*new_block).prev = (*best).prev;
                if (*best).prev as usize != 0 {
                    (*(*best).prev).next = new_block;
                }
                if (*best).next as usize != 0 {
                    (*(*best).next).prev = new_block;
                }
                if best as usize == self.free_list_head as usize {
                    self.free_list_head = new_block;
                }

                (*best).header.size = total_needed | ALLOCATED;
                let best_footer =
                    (best as usize + total_needed - size_of::<BlockFooter>()) as *mut BlockFooter;
                (*best_footer).size = (*best).header.size;
            } else {
                // Take entire block
                (*best).header.size |= ALLOCATED;
                if (*best).prev as usize != 0 {
                    (*(*best).prev).next = (*best).next;
                }
                if (*best).next as usize != 0 {
                    (*(*best).next).prev = (*best).prev;
                }
                if best as usize == self.free_list_head as usize {
                    self.free_list_head = (*best).next;
                }
            }

            let ptr = (best as usize + size_of::<BlockHeader>()) as *mut T;
            println!("allocated: {:x}", ptr as usize);
            self.inc_diff();
            return ptr;
        }
    }

    pub fn free<T>(&mut self, ptr: *mut T) {
        unsafe {
            println!(
                "requested to free: {:x}, alloc_diff: {}",
                ptr as usize, self.alloc_diff
            );
            if ptr as usize == 0 {
                return;
            }

            let mut header = (ptr as usize - size_of::<BlockHeader>()) as *mut BlockHeader;
            let mut size = (*header).size & !ALLOCATED;
            (*header).size &= !ALLOCATED;
            // Coalesce next
            let next = (header as usize + size) as *mut BlockHeader;
            if (next as usize) < (self.end as usize) && !((*next).size & ALLOCATED > 0) {
                size += (*next).size & !ALLOCATED;
                let next_block = next as *mut FreeBlock;

                if (*next_block).prev as usize != 0 {
                    (*(*next_block).prev).next = (*next_block).next;
                }
                if (*next_block).next as usize != 0 {
                    (*(*next_block).next).prev = (*next_block).prev;
                }
                if next_block as usize == self.free_list_head as usize {
                    self.free_list_head = (*next_block).next;
                }
            }

            // Coalesce previous
            let prev_footer = (header as usize - size_of::<BlockFooter>()) as *mut BlockFooter;
            if prev_footer as usize >= self.start as usize {
                let prev = (prev_footer as usize - ((*prev_footer).size & !ALLOCATED)
                    + size_of::<BlockFooter>()) as *mut BlockHeader;

                if (*prev).size & ALLOCATED == 0 {
                    size += (*prev).size & !ALLOCATED;
                    let prev_block = prev as *mut FreeBlock;
                    if (*prev_block).prev as usize != 0 {
                        (*(*prev_block).prev).next = (*prev_block).next;
                    }
                    if (*prev_block).next as usize != 0 {
                        (*(*prev_block).next).prev = (*prev_block).prev;
                    }
                    if prev_block as usize == self.free_list_head as usize {
                        self.free_list_head = (*prev_block).next;
                    }
                    header = prev;
                }
            }

            (*header).size = size;
            let footer = (header as usize + size - size_of::<BlockFooter>()) as *mut BlockFooter;
            (*footer).size = size;

            let block = header as *mut FreeBlock;
            (*block).next = self.free_list_head;
            (*block).prev = 0 as *mut FreeBlock;
            if self.free_list_head as usize != 0 {
                (*self.free_list_head).prev = block;
            }
            self.free_list_head = block;

            println!("freed: {:x}", ptr as usize);
            self.dec_diff();
        }
    }

    pub fn realloc<T>(&mut self, ptr: *mut T, size: usize) -> *mut T {
        unsafe {
            let new = self.alloc_raw::<T>(size);
            if ptr as usize != 0 {
                mem::util::memcpy(ptr, new, size);
                self.free::<T>(ptr);
            }
            return new;
        }
    }

    pub fn leak_check(&self) {
        if self.alloc_diff > 0 {
            println!(
                "Allocator leak check failed with difference {}",
                self.alloc_diff
            );
        } else {
            println!("Allocator leak check succeeded with 0 difference",);
        }
    }
}
