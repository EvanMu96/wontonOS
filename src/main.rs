#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_workspace::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os_workspace::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernal_main);

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn kernal_main(boot_info: &'static BootInfo) -> ! {
    use blog_os_workspace::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

    println!("Hello World{}", "!");

    blog_os_workspace::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
