#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod start {
    use core::arch::global_asm;
    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let gpio_fsel2 = 1 << 3;

    unsafe {
        core::ptr::write_volatile(0x3F200008 as *mut u32, gpio_fsel2);
    }

    loop {
        unsafe {
            core::ptr::write_volatile(0x3F20001c as *mut u32, 1 << 21);

            for _ in 1..50000 {
                asm!("nop")
            }

            core::ptr::write_volatile(0x3F200028 as *mut u32, 1 << 21);

            for _ in 1..50000 {
                asm!("nop")
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
