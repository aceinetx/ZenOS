struct BlockHeader {
    size: usize,
}

struct BlockFooter {
    size: usize,
}

struct FreeBlock {
    header: BlockHeader,
    next: *mut FreeBlock,
    prev: *mut FreeBlock,
    // Footer is at the end of the block
}

pub struct Allocator {
    start: *mut (),
    end: *mut (),
    free_list_head: *mut FreeBlock,
}

impl Allocator {
    pub fn new(start: usize, size: usize) -> Allocator {
        let inst = Allocator {
            start: start as *mut (),
            end: (start + size) as *mut (),
            free_list_head: start as *mut FreeBlock,
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

    pub fn alloc<T>(&mut self, _size: usize) -> *mut T {
        unsafe {
            // Align to 8 bytes
            let size = self.align(_size, 8);
            let total_needed = size_of::<BlockHeader>() + size + size_of::<BlockFooter>();
            let mut best: *mut FreeBlock = 0 as *mut FreeBlock;
            let mut curr: *mut FreeBlock = self.free_list_head;
            loop {
                if curr == 0 as *mut FreeBlock {
                    break;
                }

                let curr_size = (*curr).header.size & !1;
                if curr_size >= total_needed
                    && ((best as usize == 0) || curr_size < (*best).header.size)
                {
                    best = curr;
                }
                curr = (*curr).next;
            }

            if best as usize == 0 {
                return 0 as *mut T;
            }

            let remaining = ((*best).header.size & !1) - total_needed;
            if remaining >= size_of::<BlockHeader>() + size_of::<BlockFooter>() + 8 {
                // Split the block
                let new_header = (best as usize + total_needed) as *mut BlockHeader;
                (*new_header).size = remaining | 8;
                let new_footer = (new_header as usize + remaining - size_of::<BlockFooter>())
                    as *mut BlockHeader;
                (*new_footer).size = remaining;

                let new_block = new_header as *mut FreeBlock;
                (*new_block).next = (*best).next;
                (*new_block).prev = (*best).prev;
                if (*best).prev as usize != 0 {
                    (*best).next = new_block;
                }
                if (*best).next as usize != 0 {
                    (*best).prev = new_block;
                }
                if best as usize == self.free_list_head as usize {
                    self.free_list_head = new_block;
                }

                (*best).header.size = total_needed | 1;
                let best_footer =
                    (best as usize + total_needed - size_of::<BlockFooter>()) as *mut BlockFooter;
                (*best_footer).size = (*best).header.size;
            } else {
                // Take entire block
                (*best).header.size |= 1;
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

            return (best as usize + size_of::<BlockHeader>()) as *mut T;
        }
    }

    pub fn free<T>(&mut self, ptr: *mut T) {
        unsafe {
            if ptr as usize == 0 {
                return;
            }

            let mut header = (ptr as usize + size_of::<BlockHeader>()) as *mut BlockHeader;
            let mut size = (*header).size & !1;
            (*header).size &= !1;
            // Coalesce next
            let next = (header as usize + size) as *mut BlockHeader;
            if (next as usize) < (self.end as usize) && !((*next).size & 1 > 0) {
                size += (*next).size & !1;
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
                let prev = (prev_footer as usize - ((*prev_footer).size & !1)
                    + size_of::<BlockFooter>()) as *mut BlockHeader;
                if (*prev).size & 1 == 0 {
                    size += (*prev).size & !1;
                    let prev_block = prev as *mut FreeBlock;
                    if (*prev_block).prev as usize != 0 {
                        (*(*prev_block).prev).next = (*prev_block).next;
                    }
                    if (*prev_block).next as usize != 0 {
                        (*(*prev_block).next).prev = (*prev_block).prev;
                    }
                    if prev_block as usize == self.free_list_head as usize {
                        self.free_list_head = prev_block;
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
        }
    }
}
