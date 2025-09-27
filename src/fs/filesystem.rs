use crate::fs::entry::FsEntry;
use alloc::{string::*, vec::*};

#[derive(Debug)]
pub struct FileSystem {
    root: FsEntry,
}

impl<'a> FileSystem {
    pub fn new() -> FileSystem {
        return FileSystem {
            root: FsEntry::Directory("".into(), Vec::new()),
        };
    }

    pub(crate) fn trace_path(
        &'a mut self,
        mut path: Vec<String>,
        initial: Option<&'a mut FsEntry>,
    ) -> Option<&'a mut FsEntry> {
        if initial.is_none() {
            unsafe {
                // fuck borrow checker
                let root: *mut FsEntry;
                {
                    root = &mut self.root;
                }
                return self.trace_path(path, Some(root.as_mut().unwrap()));
            }
        }
        if path.is_empty() {
            return Some(initial.unwrap());
        }

        if let Some(initial) = initial {
            if let FsEntry::Directory(_, content) = initial {
                for entry in content.iter_mut() {
                    match entry {
                        FsEntry::Directory(name, _) => {
                            if path.len() == 1 {
                                if name.to_string() == path[0] {
                                    return Some(entry);
                                }
                            }
                            path.remove(0);
                            return self.trace_path(path, Some(entry));
                        }
                        FsEntry::File(name, _) => {
                            if path.len() == 1 {
                                if name.to_string() == path[0] {
                                    return Some(entry);
                                }
                            }
                        }
                    }
                }
            }
        }
        return None;
    }

    pub fn convert_path(&self, path: String) -> Option<Vec<String>> {
        let mut result: Vec<String> = Vec::new();
        for part in path.split("/") {
            result.push(part.to_string());
        }

        if result.len() > 0 {
            if result[0] == "" {
                result.remove(0);
            }
        }
        if result.len() > 0 {
            if result[result.len() - 1] == "" {
                result.pop();
            }
        }

        // validate
        for part in result.iter() {
            if part.to_string() == "" {
                return None;
            }
        }

        return Some(result);
    }

    pub fn create_file(&mut self, path: String) -> Result<(), &'static str> {
        if let Some(mut new_path) = self.convert_path(path) {
            let name = new_path.pop().unwrap();
            if let Some(dir) = self.trace_path(new_path, None) {
                if let FsEntry::Directory(_, content) = dir {
                    content.push(FsEntry::File(name, Vec::new()));
                    return Ok(());
                } else {
                    return Err("not a directory");
                }
            } else {
                return Err("file/directory not found");
            }
        } else {
            return Err("invalid path");
        }
    }

    pub fn create_directory(&mut self, path: String) -> Result<(), &'static str> {
        if let Some(mut new_path) = self.convert_path(path) {
            let name = new_path.pop().unwrap();
            if let Some(dir) = self.trace_path(new_path, None) {
                if let FsEntry::Directory(_, content) = dir {
                    content.push(FsEntry::Directory(name, Vec::new()));
                    return Ok(());
                } else {
                    return Err("not a directory");
                }
            } else {
                return Err("file/directory not found");
            }
        } else {
            return Err("invalid path");
        }
    }

    pub fn write_file(&mut self, path: String, bytes: Vec<u8>) -> Result<(), &'static str> {
        if let Some(new_path) = self.convert_path(path) {
            if let Some(file) = self.trace_path(new_path, None) {
                if let FsEntry::File(_, data) = file {
                    *data = bytes;
                    return Ok(());
                } else {
                    return Err("not a file");
                }
            } else {
                return Err("file not found");
            }
        } else {
            return Err("invalid path");
        }
    }

    pub fn read_file(&mut self, path: String) -> Result<Vec<u8>, &'static str> {
        if let Some(new_path) = self.convert_path(path) {
            if let Some(file) = self.trace_path(new_path, None) {
                if let FsEntry::File(_, data) = file {
                    return Ok(data.clone());
                } else {
                    return Err("not a file");
                }
            } else {
                return Err("file not found");
            }
        } else {
            return Err("invalid path");
        }
    }
}
