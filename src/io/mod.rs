use uefi::proto::console::text::{Key, ScanCode};

use crate::globals;

pub fn get_char() -> Key {
    let mut key = Key::Special(ScanCode::NULL);
    while matches!(key, Key::Special(ScanCode::NULL)) {
        key = get_char_unlocked();
    }

    return key;
}

pub fn get_char_unlocked() -> Key {
    let mut st = globals::get_system_table();
    if let Ok(opt) = st.stdin().read_key() {
        if let Some(key) = opt {
            return key;
        }
    }
    return Key::Special(ScanCode::NULL);
}
