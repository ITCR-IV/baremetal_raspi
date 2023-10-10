use super::*;

impl<const N: u8, MODE: PinMode> Pin<N, MODE> {
    /// Configures the pin to operate as a input pin
    pub fn into_input(self) -> Pin<N, Input> {
        self.into_mode()
    }

    /// Configures the pin to operate as an output pin
    pub fn into_output(self) -> Pin<N, Output> {
        self.into_mode()
    }

    /// Configures the pin to operate as an output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_output_in_state(mut self, initial_state: PinState) -> Pin<N, Output> {
        self._set_state(initial_state);
        self.into_mode()
    }

    seq!(AFN in 0..5 {
    /// Configures the pin to operate as the given alternate function. Consult
    /// the chip's documentation to see which alternate functions are available
    /// and valid.
    pub fn into_alternate_fn~AFN(self) -> Pin<N, AF~AFN> {
        self.into_mode()
    }
    });

    /// Puts `self` into mode `M`.
    ///
    /// This violates the type state constraints from `MODE`, so callers must
    /// ensure they use this properly.
    #[inline(always)]
    fn mode<M: PinMode>(&mut self) {
        unsafe {
            let gpio = crate::pac::GPIO::PTR;
            match N {
                0..=9 => (*gpio)
                    .gpfsel0
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                10..=19 => (*gpio)
                    .gpfsel1
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                20..=29 => (*gpio)
                    .gpfsel2
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                30..=39 => (*gpio)
                    .gpfsel3
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                40..=49 => (*gpio)
                    .gpfsel4
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                50..=53 => (*gpio)
                    .gpfsel5
                    .write_with_zero(|w| w.bits(M::BITS << N * 3)),
                _ => panic!("Tried to set inexistent pin's mode"),
            }
        }
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> Pin<N, M> {
        self.mode::<M>();
        Pin::new()
    }
}

/// Marker trait for valid pin modes (type state).
pub trait PinMode {
    // These constants are used to implement the pin configuration code.
    // They are not part of public API.

    #[doc(hidden)]
    const BITS: u32;
}

impl PinMode for Input {
    const BITS: u32 = 0b000;
}

impl PinMode for Output {
    const BITS: u32 = 0b001;
}

impl PinMode for AF0 {
    const BITS: u32 = 0b100;
}

impl PinMode for AF1 {
    const BITS: u32 = 0b101;
}

impl PinMode for AF2 {
    const BITS: u32 = 0b110;
}

impl PinMode for AF3 {
    const BITS: u32 = 0b111;
}

impl PinMode for AF4 {
    const BITS: u32 = 0b011;
}

impl PinMode for AF5 {
    const BITS: u32 = 0b010;
}
