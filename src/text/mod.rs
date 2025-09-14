use crate::globals;
use uefi_services::print;

pub fn set_char(x: usize, y: usize, ch: char) {
    let mut st = globals::get_system_table();
    let conout = st.stdout();
    let _ = conout.set_cursor_position(x, y);
    print!("{}", ch);
}

pub fn clear() {
    let mut st = globals::get_system_table();
    let _ = st.stdout().clear();
}
