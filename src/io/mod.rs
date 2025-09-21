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
            let ch: char = k.into();
            if ch == '\r' {
                // newline endings are in CRLF, turn them into LF
                key = Key::Printable(Char16::try_from('\n').unwrap());
            }

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

        let stdin_ptr = st.stdin;
        let stdin = st.stdin.as_ref().unwrap();

        let mut input_key: InputKey = InputKey::default();
        let _ = (stdin.read_key_stroke)(stdin_ptr, &mut input_key);
        return input_key.into();
    }
}

pub fn get_string() -> String {
    let backspace = Char16::try_from(8u16).unwrap();
    let mut string = String::new();
    loop {
        let key = get_char();
        match key {
            Key::Printable(key) => {
                let ch: char = key.into();
                print!("{}", ch);
                if ch == '\n' {
                    break;
                }

                if key != backspace {
                    string.push(ch);
                } else {
                    string.pop();
                }
            }
            Key::Special(_) => {}
        }
    }
    return string;
}
