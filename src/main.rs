#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_workspace::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os_workspace::println;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// default entry function
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    blog_os_workspace::init();

    #[cfg(test)]
    test_main();

    println!("This shoud crashed");
    loop {}
}

// write a user-define panic handler when use no_std
// this should be a diverging function
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os_workspace::test_panic_handler(info);
}
