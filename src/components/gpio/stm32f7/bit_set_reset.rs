//! GPIO port bit set/reset register (GPIOx_BSRR)

use super::Pin;

#[derive(Debug, Clone, Copy)]
pub struct BitSetResetRegister(u32);

impl BitSetResetRegister {
    pub fn set(&mut self, pin: Pin) {
        self.0 |= 1u32 << pin as u8;
    }

    pub fn reset(&mut self, pin: Pin) {
        self.0 |= 1u32 << (pin as u8 + 16);
    }
}

impl Default for BitSetResetRegister {
    fn default() -> BitSetResetRegister {
        BitSetResetRegister(0)
    }
}
