use crate::globals;
use alloc::string::String;
use uefi::proto::console::text::{Key, ScanCode};
use uefi_services::print;

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

pub fn get_string() -> String {
    let mut string = String::new();
    loop {
        let key = get_char();
        match key {
            Key::Printable(key) => {
                let mut ch: char = key.into();
                if ch == '\r' {
                    ch = '\n';
                }
                print!("{}", ch);
                if ch == '\n' {
                    break;
                }
                string.push(ch);
            }
            Key::Special(_) => {}
        }
    }
    return string;
}
