use embedded_hal as hal;
use hal::serial::{Read, Write};
use nb::{self, block};

use crate::{
    gpio::{AF0, AF2, AF3, P14, P15, P32, P33, P36, P37},
    pac::UART0,
};

/// A serial interface
// NOTE generic over the UART peripheral
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

// convenience type alias
pub type Serial0<PINS> = Serial<UART0, PINS>;

/// Serial interface error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

pub trait TxPin<UART> {}
pub trait RxPin<UART> {}

impl TxPin<UART0> for P14<AF0> {}
impl RxPin<UART0> for P15<AF0> {}

impl TxPin<UART0> for P32<AF3> {}
impl RxPin<UART0> for P33<AF3> {}

impl TxPin<UART0> for P36<AF2> {}
impl RxPin<UART0> for P37<AF2> {}

impl<TX: TxPin<UART0>, RX: RxPin<UART0>> Serial<UART0, (TX, RX)> {
    /// Creates a UART peripheral abstraction to provide serial communication
    /// Baudrate is specified in config.txt of boot partition
    pub fn uart0(uart: UART0, pins: (TX, RX)) -> Self {
        let mut serial = Serial { uart, pins };

        // Disable UART0.
        serial.uart.cr.write(|w| unsafe { w.bits(0) });

        // Clear pending interrupts.
        serial.uart.icr.write(|w| {
            w.oeic()
                .set_bit()
                .beic()
                .set_bit()
                .peic()
                .set_bit()
                .feic()
                .set_bit()
                .rtic()
                .set_bit()
                .rtic()
                .set_bit()
                .txic()
                .set_bit()
                .rxic()
                .set_bit()
        });

        block!(serial.flush()).unwrap();

        // Set integer & fractional part of baud rate.
        // Divider = UART_CLOCK/(16 * Baud)
        // Fraction part register = (Fractional part * 64) + 0.5
        // UART_CLOCK = 48_000_000; Baud = 115200.
        // UART_CLOCK = 48_000_000; Baud = 9600.

        // Divider = 48_000_000 / (16 * 115200) = 26.041666... = 26.
        // Divider = 48000000 / (16 * 9600) = 312.5
        serial.uart.ibrd.write(|w| w.bauddivint().variant(312));

        // Fractional part register = (.5 * 64) = 32
        serial.uart.fbrd.write(|w| w.bauddivfrac().variant(32));

        // Generated baud rate = 48e6/(16* (26+43/64)) ~ 115176.9646

        // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
        serial.uart.lcr_h.write(|w| {
            w.wlen()
                .variant(0b11)
                .fen()
                .set_bit()
                .stp2()
                .clear_bit()
                .pen()
                .clear_bit()
                .brk()
                .clear_bit()
        });

        // Mask all interrupts.
        serial.uart.imsc.write(|w| {
            w.ctsmim()
                .set_bit()
                .rxim()
                .set_bit()
                .txim()
                .set_bit()
                .rtim()
                .set_bit()
                .feim()
                .set_bit()
                .peim()
                .set_bit()
                .beim()
                .set_bit()
                .oeim()
                .set_bit()
        });

        // Disable DMA
        serial.uart.dmacr.write(|w| unsafe { w.bits(0) });

        // Enable UART0, receive & transfer part of UART.
        serial
            .uart
            .cr
            .write(|w| w.uarten().set_bit().txe().set_bit().rxe().set_bit());

        serial
    }

    /// Releases the UART peripheral and associated pins
    pub fn free(self) -> (UART0, (TX, RX)) {
        (self.uart, self.pins)
    }

    /// Write a whole slice of bytes, blocking until they're all written
    pub fn write_bytes(&mut self, buffer: &[u8]) -> Result<(), Error> {
        for &byte in buffer {
            block!(self.write(byte))?;
        }

        Ok(())
    }

    /// Read `n` bytes into buffer, blocking until they're all written
    pub fn read_bytes(&mut self, n: usize, buffer: &mut [u8]) -> Result<(), Error> {
        for i in 0..n {
            buffer[i] = block!(self.read())?;
        }

        Ok(())
    }
}

impl<PINS> Read<u8> for Serial<UART0, PINS> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // read the data register
        let dr = self.uart.dr.read();

        // read the flags register
        let fr = self.uart.fr.read();

        if dr.oe().bit_is_set() {
            // Error: Buffer overrun
            Err(nb::Error::Other(Error::Overrun))
        } else if dr.pe().bit_is_set() {
            // Error: Parity error
            Err(nb::Error::Other(Error::Parity))
        } else if dr.fe().bit_is_set() {
            // Error: Parity error
            Err(nb::Error::Other(Error::Framing))
        } else if fr.rxfe().bit_is_set() {
            // No data available yet
            Err(nb::Error::WouldBlock)
        } else {
            // Data available: read the data register
            Ok(dr.data().bits())
        }
    }
}

impl<PINS> Write<u8> for Serial<UART0, PINS> {
    type Error = Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        block!(self.flush()).unwrap();

        // read the flags register
        let fr = self.uart.fr.read();

        if fr.txff().bit_is_clear() {
            self.uart.dr.write(|w| w.data().variant(byte));
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        // read the flags register
        let fr = self.uart.fr.read();

        if fr.busy().bit_is_clear() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
