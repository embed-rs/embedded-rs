//! port output data register (GPIOx_ODR)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct OutputDataRegister(BitField<u32>);

#[allow(dead_code)]
impl OutputDataRegister {
    /// Get output pin
    pub fn get(&self, pin: Pin) -> bool {
        self.0.get_bit(pin as u8)
    }
    /// Set output pin
    pub fn set(&mut self, pin: Pin, value: bool) {
        self.0.set_bit(pin as u8, value);
    }
}
