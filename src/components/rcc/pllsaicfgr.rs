//! RCC PLL configuration register (RCC_PLLSAICFGR)

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    pub fn pllsain(&self) -> u32 {
        self.0.get_range(6..15)
    }
    pub fn set_pllsain(&mut self, number: u32) {
        assert!(number >= 50 && number < 433);
        self.0.set_range(6..15, number);
    }
    pub fn set_pllsaip(&mut self, number: u32) {
        let value = match number {
            2 => 0b00,
            4 => 0b01,
            6 => 0b10,
            8 => 0b11,
            _ => panic!("invalid pllsaip value"),
        };
        self.0.set_range(16..18, value);
    }
    pub fn pllsaiq(&self) -> u32 {
        self.0.get_range(24..28)
    }
    pub fn set_pllsaiq(&mut self, number: u32) {
        assert!(number >= 2 && number < 16);
        self.0.set_range(24..28, number);
    }
    pub fn set_pllsair(&mut self, number: u32) {
        assert!(number >= 2 && number < 8);
        self.0.set_range(28..31, number);
    }
}
