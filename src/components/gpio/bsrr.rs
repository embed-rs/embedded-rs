//! GPIO port bit set/reset register (GPIOx_BSRR)

use super::PinNumber;
use core::cell::UnsafeCell;
use base::volatile::Volatile;

pub struct BitSetResetRegister(UnsafeCell<Volatile<u32>>);

impl BitSetResetRegister {
    pub fn set(&self, pin: PinNumber) {
        self.write(1u32 << (pin as u8))
    }

    pub fn reset(&self, pin: PinNumber) {
        self.write(1u32 << (pin as u8 + 16))
    }

    fn write(&self, value: u32) {
        unsafe { (*self.0.get()).write(value) };
    }
}
