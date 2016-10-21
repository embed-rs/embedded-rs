//! GPIO port mode register (GPIOx_MODER)

use super::PinNumber;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /// Sets the mode for the given pins
    pub fn set(&mut self, pins: &[PinNumber], mode: ModeBits) {
        for pin in pins {
            let offset = (*pin as u8) * 2;
            self.0.set_range(offset..offset + 2, mode as u32);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ModeBits {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
}
