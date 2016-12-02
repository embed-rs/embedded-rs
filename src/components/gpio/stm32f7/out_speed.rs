//! GPIO port output speed register (GPIOx_OSPEEDR)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct OutputSpeedRegister(BitField<u32>);

impl OutputSpeedRegister {
    /// Returns the output speed for the given pins
    pub fn set(&mut self, pin: Pin, speed: OutputSpeed) {
        let offset = (pin as u8) * 2;
        self.0.set_range(offset..offset + 2, speed as u32);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum OutputSpeed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}
