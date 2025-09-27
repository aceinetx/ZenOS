use crate::fs::filesystem::FileSystem;
use crate::mem::shared_alloc::alloc;
use crate::mem::shared_alloc::free;
use core::sync::atomic::{AtomicPtr, Ordering};

static SHARED_FS: AtomicPtr<FileSystem> = AtomicPtr::new(0 as *mut FileSystem);

pub fn create_fs() -> Result<(), &'static str> {
    let mut fs = SHARED_FS.load(Ordering::Acquire);
    if fs != 0 as *mut FileSystem {
        return Err("filesystem already created");
    }

    unsafe {
        fs = alloc::<FileSystem>();
        fs.write(FileSystem::new());
    }
    SHARED_FS.store(fs, Ordering::Release);
    return Ok(());
}

pub fn destroy_fs() -> Result<(), &'static str> {
    let fs = SHARED_FS.load(Ordering::Acquire);
    if fs == 0 as *mut FileSystem {
        return Err("filesystem not created");
    }

    free(fs);
    return Ok(());
}

pub fn get_fs() -> Option<&'static mut FileSystem> {
    let fs = SHARED_FS.load(Ordering::Acquire);
    if fs == 0 as *mut FileSystem {
        return None;
    }

    unsafe {
        return Some(fs.as_mut().unwrap());
    }
}
