use crate::globals;
use alloc::string::String;
use uefi::proto::console::text::Key;
use uefi::{Char16, print};
use uefi_raw::protocol::console::InputKey;

pub fn get_char() -> Key {
    let null_char = Char16::try_from(0u16).unwrap();
    let mut key;
    loop {
        key = get_char_unlocked();
        if let Key::Printable(k) = key {
            if k != null_char {
                break;
            }
        }
    }

    return key;
}

pub fn get_char_unlocked() -> Key {
    unsafe {
        let st = globals::get_system_table();
        let stdin = st.stdin;
        let mut input_key: InputKey = InputKey::default();
        let _ = ((*stdin).read_key_stroke)(stdin, &mut input_key);
        return input_key.into();
    }
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
