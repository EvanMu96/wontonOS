#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// default entry function
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    loop {}
}

// write a user-define panic handler when use no_std
// this should be a diverging function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}