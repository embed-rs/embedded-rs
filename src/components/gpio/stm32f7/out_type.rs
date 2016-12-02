//! GPIO port output type register (GPIOx_OTYPER)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct OutputTypeRegister(BitField<u32>);

impl OutputTypeRegister {
    /// Sets the output type for the given pins
    pub fn set(&mut self, pin: Pin, typ: OutputType) {
        let offset = pin as u8;
        self.0.set_range(offset..offset + 1, typ as u32);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum OutputType {
    PushPull = 0,
    OpenDrain = 1,
}
