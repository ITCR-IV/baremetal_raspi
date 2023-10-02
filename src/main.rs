#![no_std] // No linkear standard lib
#![no_main] // Deshabilitar Rust-level entry points

use core::panic::PanicInfo;

mod boot {

    use core::arch::global_asm;

    // Header de sección en ensamblador
    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
