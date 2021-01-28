#![no_std]
// do not use normal entryl
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// default entry function
#[no_mangle]
pub extern "C" fn _start() -> ! {

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// write a user-define panic handler when use no_std
// this should be a diverging function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}