#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::{asm, naked_asm};
use core::{panic::PanicInfo, ptr};

use common::println;

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

        println!("Hello World");
        println!("Hey!");
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

#[naked]
#[unsafe(no_mangle)]
unsafe extern "C" fn trap_handler_entry() {
    unsafe {
        naked_asm!(
            // 現在のスタックポインタ(sp)と
            // sscratchレジスタの値を交換する
            "csrrw sp, sscratch, sp",
            "addi sp, sp, -4 * 31",
            "sw ra,  4 * 0(sp)",
            "sw gp,  4 * 1(sp)",
            "sw tp,  4 * 2(sp)",
            "sw t0,  4 * 3(sp)",
            "sw t1,  4 * 4(sp)",
            "sw t2,  4 * 5(sp)",
            "sw t3,  4 * 6(sp)",
            "sw t4,  4 * 7(sp)",
            "sw t5,  4 * 8(sp)",
            "sw t6,  4 * 9(sp)",
            "sw a0,  4 * 10(sp)",
            "sw a1,  4 * 11(sp)",
            "sw a2,  4 * 12(sp)",
            "sw a3,  4 * 13(sp)",
            "sw a4,  4 * 14(sp)",
            "sw a5,  4 * 15(sp)",
            "sw a6,  4 * 16(sp)",
            "sw a7,  4 * 17(sp)",
            "sw s0,  4 * 18(sp)",
            "sw s1,  4 * 19(sp)",
            "sw s2,  4 * 20(sp)",
            "sw s3,  4 * 21(sp)",
            "sw s4,  4 * 22(sp)",
            "sw s5,  4 * 23(sp)",
            "sw s6,  4 * 24(sp)",
            "sw s7,  4 * 25(sp)",
            "sw s8,  4 * 26(sp)",
            "sw s9,  4 * 27(sp)",
            "sw s10, 4 * 28(sp)",
            "sw s11, 4 * 29(sp)",
            // 元のスタックポインタをa0レジスタに移動
            "csrr a0, sscratch",
            // 元のスタックポインタをスタックに保存
            "sw a0, 4 * 30(sp)",
            "addi a0, sp, 4 * 31",
            "csrw sscratch, a0",
            "mv a0, sp",
            "call handle_trap",
            "lw ra,  4 * 0(sp)",
            "lw gp,  4 * 1(sp)",
            "lw tp,  4 * 2(sp)",
            "lw t0,  4 * 3(sp)",
            "lw t1,  4 * 4(sp)",
            "lw t2,  4 * 5(sp)",
            "lw t3,  4 * 6(sp)",
            "lw t4,  4 * 7(sp)",
            "lw t5,  4 * 8(sp)",
            "lw t6,  4 * 9(sp)",
            "lw a0,  4 * 10(sp)",
            "lw a1,  4 * 11(sp)",
            "lw a2,  4 * 12(sp)",
            "lw a3,  4 * 13(sp)",
            "lw a4,  4 * 14(sp)",
            "lw a5,  4 * 15(sp)",
            "lw a6,  4 * 16(sp)",
            "lw a7,  4 * 17(sp)",
            "lw s0,  4 * 18(sp)",
            "lw s1,  4 * 19(sp)",
            "lw s2,  4 * 20(sp)",
            "lw s3,  4 * 21(sp)",
            "lw s4,  4 * 22(sp)",
            "lw s5,  4 * 23(sp)",
            "lw s6,  4 * 24(sp)",
            "lw s7,  4 * 25(sp)",
            "lw s8,  4 * 26(sp)",
            "lw s9,  4 * 27(sp)",
            "lw s10, 4 * 28(sp)",
            "lw s11, 4 * 29(sp)",
            "lw sp,  4 * 30(sp)",
            "sret",
        );
    }
}
