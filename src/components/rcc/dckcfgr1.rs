//! RCC dedicated clocks configuration register (RCC_DKCFGR1)

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /**
    division factor for LCD_CLK

    These bits are set and cleared by software to control the frequency of LCD_CLK.
    They should be written only if PLLSAI is disabled.
    LCD_CLK frequency = f(PLLSAI_R) / PLLSAIDIVR with 2 ≤ PLLSAIDIVR ≤ 16
    **/
    pub fn set_pllsai_divr(&mut self, number: u32) {
        let bits = match number {
            2 => 0b00,
            4 => 0b01,
            8 => 0b10,
            16 => 0b11,
            _ => panic!("invalid value"),
        };
        self.0.set_range(16..18, bits);
    }

    pub fn pllsai_divq(&self) -> u32 {
        self.0.get_range(8..13)
    }

    pub fn set_sai2_clock_source(&mut self, clock_source: SaiClockSource) {
        self.0.set_range(22..24, clock_source as u32);
    }

    pub fn set_plli2s_divq(&mut self, number: u32) {
        self.0.set_range(0..5, number - 1);
    }

    // etc
}

#[derive(Debug)]
#[repr(u32)]
pub enum SaiClockSource {
    PllI2S = 0b00,
    PllSai = 0b01,
    PinInput = 0b10,
}
