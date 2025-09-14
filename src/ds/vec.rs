use crate::mem::shared_alloc::{alloc, free};

struct Entry<T> {
    data: T,
    next: *mut Entry<T>,
    prev: *mut Entry<T>,
}

pub struct Vec<T> {
    head: *mut Entry<T>,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        return Vec {
            head: 0 as *mut Entry<T>,
        };
    }

    pub fn push(&mut self, element: T) {
        unsafe {
            let mut curr = self.head;
            if curr as usize == 0 {
                let entry = alloc::<Entry<T>>();
                (*entry).prev = 0 as *mut Entry<T>;
                (*entry).next = 0 as *mut Entry<T>;
                (*entry).data = element;
                self.head = entry;
                return;
            }

            loop {
                if (*curr).next as usize == 0 {
                    let entry = alloc::<Entry<T>>();
                    (*entry).prev = curr;
                    (*entry).next = 0 as *mut Entry<T>;
                    (*entry).data = element;
                    (*curr).next = entry;
                    break;
                }

                curr = (*curr).next;
            }
        }
    }

    pub fn pop(&mut self) {
        unsafe {
            let mut curr = self.head;
            if curr as usize == 0 {
                panic!("called pop on an empty vector");
            }

            loop {
                if (*curr).next as usize == 0 {
                    let prev = (*curr).prev;
                    if prev as usize != 0 {
                        (*prev).next = 0 as *mut Entry<T>;
                    } else {
                        self.head = 0 as *mut Entry<T>;
                    }
                    free(curr);
                    break;
                }

                curr = (*curr).next;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.head as usize == 0;
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            let mut curr = self.head;
            loop {
                if curr as usize == 0 {
                    break;
                }
                let next = (*curr).next;
                free::<Entry<T>>(curr);
                curr = next;
            }
        }
    }
}
