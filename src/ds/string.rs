use core::cmp::PartialEq;
use core::fmt::{Debug, Display, Formatter, Result};
use core::ops::{Index, IndexMut};

use crate::ds::vec::Vec;

pub struct String {
    vec: Vec<char>,
}

impl String {
    pub fn new() -> String {
        return String { vec: Vec::new() };
    }

    pub fn from(s: &str) -> String {
        let mut inst = String::new();
        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap();

            inst.vec.push(ch);
        }
        return inst;
    }

    pub fn push(&mut self, ch: char) {
        self.vec.push(ch);
    }

    pub fn pop(&mut self) {
        self.vec.pop();
    }

    pub fn is_empty(&self) -> bool {
        return self.vec.is_empty();
    }

    pub fn get(&self, index: usize) -> char {
        return self.vec.get(index);
    }

    pub fn get_ref(&self, index: usize) -> &mut char {
        return self.vec.get_ref(index);
    }

    pub fn set(&mut self, index: usize, ch: char) {
        self.vec.set(index, ch);
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn len(&self) -> usize {
        return self.vec.len();
    }

    pub fn chars(&self) -> &Vec<char> {
        return &self.vec;
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut vec = Vec::<u8>::new();
        for i in 0..self.len() {
            vec.push(self.get(i) as u8);
        }
        return vec;
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "buffer=\"{}\"", self)
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..self.vec.len() {
            let ch = self.vec[i];
            if let Err(e) = write!(f, "{}", ch) {
                return Err(e);
            }
        }
        Ok(())
    }
}

impl Index<usize> for String {
    type Output = char;
    fn index<'a>(&'a self, index: usize) -> &'a char {
        return self.get_ref(index);
    }
}

impl IndexMut<usize> for String {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut char {
        return self.get_ref(index);
    }
}

impl PartialEq for String {
    fn eq(&self, other: &String) -> bool {
        return self.vec == other.vec;
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        return self.vec == String::from(other).vec;
    }
}
