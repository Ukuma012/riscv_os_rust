use core::arch::naked_asm;

use crate::memory::{PAddr, VAddr};

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    UNUSED,
    RUNNABLE,
    IDLE,
    EXITED,
}

#[derive(Copy, Clone, Debug)]
struct Process {
    pid: u32,
    state: State,
    sp: VAddr,
    page_table: PAddr,
    stack: [u8; 8192],
}

impl Process {
    const fn new() -> Self {
        Self {
            pid: 0,
            state: State::UNUSED,
            sp: 0, // Context Switch時のスタックポインタ
            page_table: 0,
            stack: [0; 8192],
        }
    }
}

#[naked]
#[unsafe(no_mangle)]
unsafe extern "C" fn switch_context(prev_sp: *mut u32, next_sp: *const u32) {
    unsafe {
        naked_asm!(
            // 実行中プロセスのスタックへレジスタを保存
            "addi sp, sp, -13 * 4",
            "sw ra, 0 * 4(sp)",
            "sw s0, 1 * 4(sp)",
            "sw s1, 2 * 4(sp)",
            "sw s2, 3 * 4(sp)",
            "sw s3, 4 * 4(sp)",
            "sw s4, 5 * 4(sp)",
            "sw s5, 6 * 4(sp)",
            "sw s6, 7 * 4(sp)",
            "sw s7, 8 * 4(sp)",
            "sw s8, 9 * 4(sp)",
            "sw s9, 10 * 4(sp)",
            "sw s10, 11 * 4(sp)",
            "sw s11, 12 * 4(sp)",
            // スタックポインタの切り替え
            "sw sp, (a0)",
            "lw sp, (a1)",
            // 次のプロセスのスタックからレジスタを復元
            "lw ra, 0 * 4(sp)",
            "lw s0, 1 * 4(sp)",
            "lw s1, 2 * 4(sp)",
            "lw s2, 3 * 4(sp)",
            "lw s3, 4 * 4(sp)",
            "lw s4, 5 * 4(sp)",
            "lw s5, 6 * 4(sp)",
            "lw s6, 7 * 4(sp)",
            "lw s7, 8 * 4(sp)",
            "lw s8, 9 * 4(sp)",
            "lw s9, 10 * 4(sp)",
            "lw s10, 11 * 4(sp)",
            "lw s11, 12 * 4(sp)",
            "addi sp, sp, 13 * 4",
            "ret"
        )
    }
}
