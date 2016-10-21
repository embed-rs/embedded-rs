//! port output data register (GPIOx_ODR)

use super::PinNumber;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

#[allow(dead_code)]
impl Register {
    /// Get output pin
    pub fn get(&self, pin: PinNumber) -> bool {
        self.0.get_bit(pin as u8)
    }
    /// Set output pin
    pub fn set(&mut self, pin: PinNumber, value: bool) {
        self.0.set_bit(pin as u8, value);
    }
}
