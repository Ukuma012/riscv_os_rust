#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::naked_asm, panic::PanicInfo};

mod shell;

unsafe extern "C" {
    static __stack_top: u32;
}

#[unsafe(link_section = ".text.start")]
#[naked]
#[unsafe(no_mangle)]
extern "C" fn start() {
    unsafe {
        naked_asm!(
            "la sp, {stack_top}",
            "call main",
            "call exit",
            stack_top = sym  __stack_top,
        );
    }
}

#[unsafe(no_mangle)]
fn exit() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
