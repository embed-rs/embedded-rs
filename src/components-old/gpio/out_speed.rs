//! GPIO port output speed register (GPIOx_OSPEEDR)

use super::PinNumber;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /// Sets the output speed for the given pins
    pub fn set(&mut self, pins: &[PinNumber], speed: Speed) {
        for pin in pins {
            let offset = (*pin as u8) * 2;
            self.0.set_range(offset..offset + 2, speed as u32);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Speed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}
