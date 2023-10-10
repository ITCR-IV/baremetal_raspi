#![no_std] // No linkear standard lib
#![no_main] // Deshabilitar Rust-level entry points

use core::panic::PanicInfo;

pub mod clocks;
pub mod gpio;
pub mod serial;

pub use bcm2837_lpa as pac;

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
    // Podríamos hacer que blinkee un LED para que sepamos si crashea ?
    loop {}
}
