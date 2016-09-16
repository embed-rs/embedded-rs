//! GPIO port input data register (GPIOx_IDR)

use super::PinNumber;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /// Get input pin
    pub fn get(&self, pin: PinNumber) -> bool {
        self.0.get_bit(pin as u8)
    }
}
