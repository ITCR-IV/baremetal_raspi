#![no_std]
#![no_main]

pub use bcm2837_lpa as pac;
use embedded_hal::serial::{Read, Write};
use gpio::GpioExt;
use nb::block;
use serial::Serial;

pub mod gpio;
pub mod serial;

use core::arch::asm;
use core::panic::PanicInfo;

mod start {
    use core::arch::global_asm;
    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // NOTE(unsafe) Solo llamar steal() una vez!!
    let dp = unsafe { pac::Peripherals::steal() };
    let pins = dp.GPIO.split();

    let mut p20o = pins.p20.into_output();
    p20o.set_low();
    let mut p21o = pins.p21.into_output();
    p21o.set_low();

    let tx = pins.p14.into_alternate_fn0();
    let rx = pins.p15.into_alternate_fn0();

    //let buffer = Temperature(50.1).to_bytes();
    let buffer: [u8; 6] = [b'a', b' ', b'x', b'd', b'\r', b'\n'];

    let mut uart = Serial::uart0(dp.UART0, (tx, rx));

    loop {
        for _ in 1..10_000_000 {
            unsafe { asm!("nop") }
        }

        match block!(uart.read()) {
            Ok(b) => block!(uart.write(b)).unwrap(),
            Err(_) => p20o.set_high(),
        }
        p21o.set_high();
        uart.write_bytes(&buffer).unwrap();
        for _ in 1..10_000_000 {
            unsafe { asm!("nop") }
        }
        p21o.set_low();
        // uart.write_bytes(&buffer).unwrap();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
