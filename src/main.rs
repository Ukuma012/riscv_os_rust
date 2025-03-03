#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::naked_asm;
use core::{panic::PanicInfo, ptr};

mod sbi;

unsafe extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}

#[unsafe(no_mangle)]
fn kernel_main() {
    unsafe {
        let bss = ptr::addr_of_mut!(__bss);
        let bss_end = ptr::addr_of!(__bss_end);
        ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);

        let hello = b"Hello World\n";
        for &ch in hello.iter() {
            sbi::putchar(ch);
        }
    }

    loop {}
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(no_mangle)]
#[naked]
extern "C" fn boot() {
    unsafe {
        naked_asm!(
            "la sp, {stack_top}",
            "j kernel_main",
            stack_top = sym  __stack_top,
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
