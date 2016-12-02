//! GPIO alternate function register (GPIOx_AFRL and GPIOx_AFRH)

use super::Pin;
use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AlternateFunctionRegister {
    low: Low,
    high: High,
}

impl AlternateFunctionRegister {
    /// Sets `pin`
    pub fn set(&mut self, pin: Pin, alternate_fn: AlternateFunction) {
        self.low.set(pin, alternate_fn);
        self.high.set(pin, alternate_fn);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum AlternateFunction {
    AF0 = 0b0000,
    AF1 = 0b0001,
    AF2 = 0b0010,
    AF3 = 0b0011,
    AF4 = 0b0100,
    AF5 = 0b0101,
    AF6 = 0b0110,
    AF7 = 0b0111,
    AF8 = 0b1000,
    AF9 = 0b1001,
    AF10 = 0b1010,
    AF11 = 0b1011,
    AF12 = 0b1100,
    AF13 = 0b1101,
    AF14 = 0b1110,
    AF15 = 0b1111,
}


#[derive(Clone, Copy)]
#[repr(C)]
struct High(BitField<u32>);

impl High {
    /// Sets the alternate function for the given high pins
    pub fn set(&mut self, pin: Pin, alternate_fn: AlternateFunction) {
        let pin_number = pin as u8;
        if pin_number >= 8 {
            let offset = (pin_number - 8) * 4;
            self.0.set_range(offset..(offset + 4), alternate_fn as u32);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Low(BitField<u32>);

impl Low {
    /// Sets the alternate function for the given low pins
    pub fn set(&mut self, pin: Pin, alternate_fn: AlternateFunction) {
        let pin_number = pin as u8;
        if pin_number < 8 {
            let offset = pin_number * 4;
            self.0.set_range(offset..(offset + 4), alternate_fn as u32);
        }
    }
}
