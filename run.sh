#!/bin/bash
set -xue

QEMU=qemu-system-riscv32
KERNEL=target/riscv32i-unknown-none-elf/debug/riscv_os_rust
USER=user/target/riscv32i-unknown-none-elf/release/user

(cd user && cargo build --release)
llvm-objcopy --set-section-flags .bss=alloc,contents -O binary $USER shell.bin
llvm-objcopy -Ibinary -Oelf32-littleriscv shell.bin shell.bin.o

cargo build
$QEMU -machine virt -bios default --no-reboot -nographic -serial mon:stdio \
      -kernel $KERNEL