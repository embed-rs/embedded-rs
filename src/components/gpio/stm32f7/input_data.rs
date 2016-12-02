//! GPIO port input data register (GPIOx_IDR)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct InputDataRegister(BitField<u32>);

impl InputDataRegister {
    /// Get input pin
    pub fn get(&self, pin: Pin) -> bool {
        self.0.get_bit(pin as u8)
    }
}
