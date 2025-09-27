use alloc::string::*;
use alloc::vec::*;

#[derive(Debug)]
pub enum FsEntry {
    File(String, Vec<u8>),
    Directory(String, Vec<FsEntry>),
}

impl Default for FsEntry {
    fn default() -> Self {
        return FsEntry::Directory("".into(), Vec::new());
    }
}
