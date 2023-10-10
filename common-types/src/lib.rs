#![no_std]

use core::mem::size_of;

pub struct Temperature(pub f32);

impl Temperature {
    pub fn to_bytes(self) -> [u8; size_of::<f32>()] {
        self.0.to_le_bytes()
    }

    pub fn from_bytes(bytes: [u8; size_of::<f32>()]) -> Self {
        Temperature(f32::from_le_bytes(bytes))
    }
}
