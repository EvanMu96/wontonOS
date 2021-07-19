// in tests/heap_allocation.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wontonOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use wontonOS::allocator;
    use wontonOS::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    wontonOS::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wontonOS::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13); 
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
}

#[test_case]
fn many_boxes() {
    // allocate all heap memory
    use alloc::boxed::Box;
    use wontonOS::allocator::HEAP_SIZE;
    let mut piece_size = 0;

    while piece_size < HEAP_SIZE {
        let x = Box::new(piece_size);
        assert_eq!(*x, piece_size);
        if piece_size == 0 {
            piece_size += 1;
        }
        else {
            piece_size *= 2;
        }
        // deallocate here then allocate larger
    }
}