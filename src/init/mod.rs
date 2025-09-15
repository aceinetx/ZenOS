use crate::ds::vec::Vec;
use crate::text;

use uefi_services::println;

pub fn zen_main() -> Result<(), &'static str> {
    for x in 0..50 {
        text::set_char(x, 0, '#');
        text::set_char(x, 24, '#');
    }
    for y in 0..25 {
        text::set_char(0, y, '#');
        text::set_char(50, y, '#');
    }

    println!();

    let mut vec = Vec::<i32>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }

    Ok(())
}
