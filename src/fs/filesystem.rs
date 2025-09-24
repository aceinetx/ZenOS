use crate::fs::file::File;
use alloc::vec::*;

pub struct FileSystem {
    files: Vec<File>,
}

pub fn read_file() -> Result<Vec<u8>, &'static str> {
    return Ok(Vec::new());
}
