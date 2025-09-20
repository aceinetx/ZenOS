use crate::globals;
use uefi::print;

pub fn set_char(x: usize, y: usize, ch: char) {
    let st = globals::get_system_table();
    unsafe {
        let conout = st.stdout;

        let _ = ((*conout).set_cursor_position)(conout, x, y);
        print!("{}", ch);
    }
}

pub fn clear() {
    let st = globals::get_system_table();
    unsafe {
        let conout = st.stdout;
        let _ = ((*conout).clear_screen)(conout);
    }
}
