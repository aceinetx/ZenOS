use core::fmt::{Display, Formatter, Result};

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
