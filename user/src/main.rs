#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};

mod shell;

pub const SYS_PUTCHAR: u32 = 1;

unsafe extern "C" {
    static __stack_top: u32;
}

#[unsafe(link_section = ".text.start")]
#[naked]
#[unsafe(no_mangle)]
unsafe extern "C" fn start() {
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

unsafe fn syscall(sysno: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    let mut result: u32;
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") sysno, // システムコール番号
            lateout("a0") result
        );
    }

    result
}

pub fn putchar(ch: u8) {
    unsafe {
        syscall(SYS_PUTCHAR, ch as u32, 0, 0);
    }
}
