#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_workspace::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os_workspace::{memory, println};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;

entry_point!(kernal_main);

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn kernal_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    blog_os_workspace::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // let mut frame_allocator = memory::EmptyFrameAllocator;

    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // test page allocation
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("It works fine.");
    blog_os_workspace::hlt_loop();
}

// write a user-define panic handler when use no_std
// this should be a diverging function
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os_workspace::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os_workspace::test_panic_handler(info);
}
