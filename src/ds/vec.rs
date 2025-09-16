use crate::mem::shared_alloc::*;
use crate::mem::util::*;
use core::cmp::PartialEq;

use core::ops::{Index, IndexMut};

#[derive(Debug)]
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
            let new_data = alloc_bytes::<u8>(self.capacity + self.element_size);
            if self.data as usize != 0 {
                memcpy::<u8>(self.data as *mut u8, new_data, self.capacity);
                free_bytes(self.data, self.capacity);
            }
            self.data = new_data as *mut T;

            *self.data.wrapping_add(self.length) = element;

            self.length += 1;
            self.capacity += self.element_size;
        }
    }

    pub fn push_vec(&mut self, another: &Vec<T>) {
        for i in 0..another.len() {
            let element = another.get(i);
            self.push(element);
        }
    }

    pub fn pop(&mut self) {
        unsafe {
            let new_data = alloc_bytes::<u8>(self.capacity - self.element_size);
            if self.data as usize != 0 {
                memcpy::<u8>(
                    self.data as *mut u8,
                    new_data,
                    self.capacity - self.element_size,
                );
                free_bytes(self.data, self.capacity);
            }
            self.data = new_data as *mut T;

            self.length -= 1;
            self.capacity -= self.element_size;
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.data as usize == 0;
    }

    pub fn get_ref(&self, index: usize) -> &mut T {
        if index >= self.length {
            panic!(
                "trying to index {} into a vector of length {}",
                index, self.length
            );
        }

        unsafe {
            let element = self.data.wrapping_add(index).as_mut().unwrap();
            return element;
        }
    }

    pub fn get(&self, index: usize) -> T {
        if index >= self.length {
            panic!(
                "trying to index {} into a vector of length {}",
                index, self.length
            );
        }

        unsafe {
            let element = self.data.wrapping_add(index).read();
            return element;
        }
    }

    // The self is mut because it wouldn't make sense if we were able to index into a const vector
    pub fn set(&mut self, index: usize, element: T) {
        let reference = self.get_ref(index);
        *reference = element;
    }

    pub fn clear(&mut self) {
        if self.data as usize == 0 {
            return;
        }
        free_bytes(self.data, self.capacity);
        self.capacity = 0;
        self.length = 0;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.data as usize != 0 {
            free_bytes(self.data, self.capacity);
        }
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        return self.get_ref(index);
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        return self.get_ref(index);
    }
}

impl<T> PartialEq<Vec<T>> for Vec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        unsafe {
            if self.len() != other.len() {
                return false;
            }
            return memcmp(
                self.data as *const u8,
                other.data as *const u8,
                self.capacity(),
            ) == 0;
        }
    }
}

impl<T> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new = Vec::new();
        unsafe {
            new.data = memdup(self.data, self.capacity);
            new.capacity = self.capacity;
            new.length = self.length;
        }
        return new;
    }
}
