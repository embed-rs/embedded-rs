//! GPIO port output type register (GPIOx_OTYPER)

use super::PinNumber;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /// Sets the output type for the given pins
    pub fn set(&mut self, pins: &[PinNumber], typ: Type) {
        for pin in pins {
            let offset = *pin as u8;
            self.0.set_range(offset..offset + 1, typ as u32);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Type {
    PushPull = 0,
    OpenDrain = 1,
}
