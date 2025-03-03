#!/bin/bash
set -xue

QEMU=qemu-system-riscv32
KERNEL=target/riscv32i-unknown-none-elf/debug/riscv_os_rust

cargo build

$QEMU -machine virt -bios default --no-reboot -nographic -serial mon:stdio \
      -kernel $KERNEL