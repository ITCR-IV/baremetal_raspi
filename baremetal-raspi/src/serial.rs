use embedded_hal as hal;
use nb;

use bcm2837_lpa::UART0;

/// A serial interface
// NOTE generic over the UART peripheral
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

// convenience type alias
pub type Serial0<PINS> = Serial<UART0, PINS>;

/// Serial interface error
pub enum Error {
    /// Framing error
    Framing,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
}

trait TxPin<UART> {}
trait RxPin<UART> {}

impl<TX: TxPin<UART0>, RX: RxPin<UART0>> Serial<UART0, (TX, RX)> {
    /// Creates a UART peripheral abstraction to provide serial communication
    /// Baudrate is specified in config.txt of boot partition
    pub fn uart0(uart: UART0, pins: (TX, RX)) -> Self {
        // UART should already be enabled from boot so no init code is required
        Serial { uart, pins }
    }

    /// Releases the UART peripheral and associated pins
    pub fn free(self) -> (UART0, (TX, RX)) {
        (self.uart, self.pins)
    }
}

impl<PINS> hal::serial::Read<u8> for Serial<UART0, PINS> {
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

impl<PINS> hal::serial::Write<u8> for Serial<UART0, PINS> {
    type Error = Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
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
