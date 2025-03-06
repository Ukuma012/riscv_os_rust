use core::{panic, ptr};

pub type PAddr = u32;
pub type VAddr = u32;
pub const PAGE_SIZE: usize = 4096;

unsafe extern "C" {
    static mut __free_ram: u8;
    static mut __free_ram_end: u8;
}

static mut NEXT_PADDR: *mut u8 = ptr::addr_of_mut!(__free_ram);

pub fn alloc_pages(n: usize) -> PAddr {
    unsafe {
        let paddr = NEXT_PADDR as PAddr;
        NEXT_PADDR = NEXT_PADDR.add(n * PAGE_SIZE);

        if NEXT_PADDR > ptr::addr_of_mut!(__free_ram_end) {
            panic!("out of memory");
        }

        ptr::write_bytes(paddr as *mut u8, 0, (n * PAGE_SIZE) as usize);
        paddr
    }
}
