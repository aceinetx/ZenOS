use crate::mem::shared_alloc::{alloc, free, realloc};

pub struct Vec<T> {
    data: *mut T,
    length: usize,
    capacity: usize,
    element_size: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        return Vec {
            data: 0 as *mut T,
            length: 0,
            capacity: 0,
            element_size: size_of::<T>(),
        };
    }

    pub fn push(&mut self, element: T) {
        unsafe {
            self.data = realloc(self.data, self.capacity + self.element_size);
            *self.data.wrapping_add(self.length) = element;

            self.length += 1;
            self.capacity += self.element_size;
        }
    }

    pub fn pop(&mut self) {}

    pub fn is_empty(&self) -> bool {
        return self.data as usize == 0;
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.data as usize != 0 {
            free(self.data);
        }
    }
}
