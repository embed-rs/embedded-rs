//! GPIO port pull-up/pull-down register (GPIOx_PUPDR)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct ResistorRegister(BitField<u32>);

impl ResistorRegister {
    /// Sets the resistor for the given pins
    pub fn set(&mut self, pin: Pin, resistor: Resistor) {
        let offset = (pin as u8) * 2;
        self.0.set_range(offset..offset + 2, resistor as u32);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Resistor {
    NoPull = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}
