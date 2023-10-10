#![no_std] // No linkear standard lib
#![no_main] // Deshabilitar Rust-level entry points

use core::panic::PanicInfo;

use common_types::Temperature;

pub mod clocks;
pub mod gpio;
pub mod serial;

pub use bcm2837_lpa as pac;
use gpio::GpioExt;
use serial::Serial;

mod boot {

    use core::arch::global_asm;

    // Header de sección en ensamblador
    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // NOTE(unsafe) Solo llamar steal() una vez!!
    let dp = unsafe { pac::Peripherals::steal() };
    let pins = dp.GPIO.split();

    let tx = pins.p14.into_alternate_fn0();
    let rx = pins.p15.into_alternate_fn0();

    let buffer = Temperature(50.1).to_bytes();

    let mut uart = Serial::uart0(dp.UART0, (tx, rx));
    uart.write_bytes(&buffer).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Podríamos hacer que blinkee un LED para que sepamos si crashea ?
    loop {}
}
