use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// int3 breakpoint exception
// commonly used in debugger
// details: https://eli.thegreenplace.net/2011/01/27/how-debuggers-work-part-2-breakpoints
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invode a bp exception manually
    x86_64::instructions::interrupts::int3();
}