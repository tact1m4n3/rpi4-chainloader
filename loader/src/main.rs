#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod miniuart;

pub const KERNEL_LOAD_ADDRESS: usize = 0x80000;

core::arch::global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe extern "C" fn rust_start(x0: u64, x1: u64, x2: u64, x3: u64) {
    miniuart::init();

    miniuart::write_byte(b'O');
    miniuart::write_byte(b'K');

    let mut size: u32 = miniuart::read_byte() as u32;
    size |= (miniuart::read_byte() as u32) << 8;
    size |= (miniuart::read_byte() as u32) << 16;
    size |= (miniuart::read_byte() as u32) << 24;

    let kernel_base = KERNEL_LOAD_ADDRESS as *mut u8;
    for i in 0..size {
        *kernel_base.add(i as usize) = miniuart::read_byte();
    }

    let kernel_entry: extern "C" fn(u64, u64, u64, u64) -> ! = core::mem::transmute(kernel_base);
    kernel_entry(x0, x1, x2, x3);
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
