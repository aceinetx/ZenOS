use crate::mem::alloc;
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

    let mut allocator = alloc::Allocator::new(0x1000000, 0x2000000);
    println!("");
    for _ in 0..10 {
        let ptr = allocator.alloc::<i32>(10);
        println!("{:x}", ptr as usize);
        allocator.free::<i32>(ptr);
    }

    Ok(())
}
