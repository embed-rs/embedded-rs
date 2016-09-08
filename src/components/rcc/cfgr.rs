//! RCC clock configuration register (RCC_CFGR)

use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    pub fn system_clock(&mut self) -> Option<SystemClock> {
        match self.0.get_range(2..4) {
            0b00 => Some(SystemClock::HSI),
            0b01 => Some(SystemClock::HSE),
            0b10 => Some(SystemClock::PLL),
            0b11 => None,
            _ => unreachable!(),
        }
    }

    pub fn set_system_clock(&mut self, value: SystemClock) {
        self.0.set_range(0..2, value as u32);
    }

    pub fn set_ahb_prescaler(&mut self, value: AhbClockDivisionFactor) {
        self.0.set_range(4..8, value as u32);
    }

    pub fn set_apb_low_speed_prescaler(&mut self, value: ApbClockDivisionFactor) {
        self.0.set_range(10..13, value as u32);
    }

    pub fn set_apb_high_speed_prescaler(&mut self, value: ApbClockDivisionFactor) {
        self.0.set_range(13..16, value as u32);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum SystemClock {
    HSI = 0b00,
    HSE = 0b01,
    PLL = 0b10,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum AhbClockDivisionFactor {
    NoDivide = 0b0000,
    Divide2 = 0b1000,
    Divide4 = 0b1001,
    Divide8 = 0b1010,
    Divide16 = 0b1011,
    Divide64 = 0b1100,
    Divide128 = 0b1101,
    Divide256 = 0b1110,
    Divide512 = 0b1111,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ApbClockDivisionFactor {
    NoDivide = 0b000,
    Divide2 = 0b100,
    Divide4 = 0b101,
    Divide8 = 0b110,
    Divide16 = 0b111,
}
