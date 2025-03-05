#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::{asm, naked_asm};
use core::{panic::PanicInfo, ptr};

use common::{println, write_csr};
use trap::trap_handler_entry;

mod sbi;
mod trap;

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

        println!("Hello World");

        let current_sp: u32;
        asm!("mv {}, sp", out(reg) current_sp);
        write_csr!("sscratch", current_sp);

        write_csr!("stvec", trap_handler_entry as u32);

        asm!("unimp");
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
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    loop {
        unsafe { asm!("wfi") }
    }
}
