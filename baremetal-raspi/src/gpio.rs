//! General Purpose Input / Output
//!
//! **NOTA: Código tomado de `stm32h7xx_hal::gpio` y adaptado**
//!
//! To get access to the GPIO pins, you first need to convert them into a
//! HAL designed struct from the `pac` struct using the [split](trait.GpioExt.html#tymethod.split) function.
//! ```rust
//! // Acquire the GPIO peripheral
//! // NOTE: `dp` is the device peripherals from the `PAC` crate
//! let mut gpio = dp.GPIO.split();
//! ```
//!
//! This gives you a struct containing all the pins `px0..px15`.
//! By default pins are in input mode. You can change their modes.
//! For example, to set `pa5` high, you would call
//!
//! ```rust
//! let output = gpio.pa5.into_output();
//! output.set_high();
//! ```
//!
//! ## Modes
//!
//! Each GPIO pin can be set to various modes:
//!
//! - **Input**: Pin mode required for reading values in pin.
//! - **Output**: Pin mode required for writing values in pin.
//! - **Alternate Function N**: Pin mode required when the pin is driven by other peripherals. The marker structs `AF0`..`AF5` are provided.
//!
//! ## Changing modes
//! The simplest way to change the pin mode is to use the `into_<mode>` functions. These return a
//! new struct with the correct mode that you can use the input or output functions on.
//!
//! If you need a more temporary mode change, and can not use the `into_<mode>` functions for
//! ownership reasons, you can use the closure based `with_<mode>` functions to temporarily change the pin type, do
//! some output or input, and then have it change back once done.
//!
//! # Examples
//!
//! - [Simple Blinky](https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/blinky.rs)
//! - [Digital Read](https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/digital_read.rs)
//! - [External Interrupt via Button](https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/exti_interrupt.rs)
//! - [Usage of `with_*` methods](https://github.com/stm32-rs/stm32h7xx-hal/blob/master/examples/gpio_with_input.rs)

use core::marker::PhantomData;
use seq_macro::seq;

mod convert;
pub use convert::PinMode;
// mod partially_erased;
// pub use partially_erased::{PEPin, PartiallyErasedPin};
// mod erased;
// pub use erased::{EPin, ErasedPin};
// mod exti;
// pub use exti::ExtiPin;
// mod dynamic;
// pub use dynamic::{Dynamic, DynamicPin};
// mod hal_02;

pub use embedded_hal::digital::v2::{OutputPin, PinState};

/// Extension trait to split a GPIO peripheral into independent pins and
/// registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Takes the GPIO peripheral and splits it into Zero-Sized Types
    /// (ZSTs) representing individual pins. These are public
    /// members of the return type.
    ///
    /// ```
    /// let device_peripherals = crate::pac::Peripherals.take().unwrap();
    ///
    /// let gpio = device_peripherals.GPIO.split();
    ///
    /// let pa0 = gpio.pa0; // Pin 0
    /// ```
    fn split(self) -> Self::Parts;
}

/// Id and mode for any pin
pub trait PinExt {
    /// Current pin mode
    type Mode;
    /// Pin number
    fn pin_id(&self) -> u8;
}

/// Input mode (type state)
pub struct Input;
/// Output mode (type state)
#[allow(unused)]
pub struct Output;

seq!(N in 0..=5 {
/// Alternate Function N mode (type state)
#[allow(unused)]
pub struct AF~N;
});

/// Generic pin type
///
/// - `N` is pin number: from `0` to `53`.
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
pub struct Pin<const N: u8, MODE = Input> {
    _mode: PhantomData<MODE>,
}
impl<const N: u8, MODE> Pin<N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const N: u8, MODE> PinExt for Pin<N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
}

impl<const N: u8, MODE> Pin<N, MODE> {
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            let gpio = crate::pac::GPIO::PTR;
            match N {
                0..=31 => (*gpio).gpset0.write_with_zero(|w| w.bits(1 << N)),
                32..=53 => (*gpio).gpset1.write_with_zero(|w| w.bits(1 << (N - 32))),
                _ => panic!("Tried to set inexistent pin"),
            }
        }
    }

    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe {
            let gpio = crate::pac::GPIO::PTR;
            match N {
                0..=31 => (*gpio).gpclr0.write_with_zero(|w| w.bits(1 << N)),
                32..=53 => (*gpio).gpclr1.write_with_zero(|w| w.bits(1 << (N - 32))),
                _ => panic!("Tried to set inexistent pin"),
            }
        }
    }

    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self._set_low(),
            PinState::High => self._set_high(),
        }
    }

    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe {
            let gpio = crate::pac::GPIO::PTR;
            match N {
                0..=31 => (*gpio).gplev0.read().bits() & (1 << N) == 0,
                32..=53 => (*gpio).gplev1.read().bits() & (1 << (N - 32)) == 0,
                _ => panic!("Tried to set inexistent pin"),
            }
        }
    }
}

impl<const N: u8> Pin<N, Output> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        self._set_state(state)
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_low()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<const N: u8> Pin<N, Input> {
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

seq!(N in 0..=53 {
        /// GPIO parts
        pub struct Pins {
        #(
            pub p~N: P~N,
        )*
}
});

impl GpioExt for crate::pac::GPIO {
    type Parts = Pins;

    fn split(self) -> Self::Parts {
        seq!(N in 0..=53 {
                Self::Parts {
                #(
                    p~N: P~N::new(),
                )*
        }
        })
    }
}

seq!(N in 0..=53 {
#[doc=concat!("Pin ", N)]
pub type P~N<MODE = Input> = Pin<N, MODE>;
});
