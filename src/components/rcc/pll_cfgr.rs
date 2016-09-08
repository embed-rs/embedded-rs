//! RCC PLL configuration register (RCC_PLLCFGR)

use bit_field::BitField;

/// Register
#[derive(Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    pub fn pllm(&self) -> u32 {
        self.0.get_range(0..6)
    }

    pub fn plln(&self) -> u32 {
        self.0.get_range(6..15)
    }

    pub fn pllp(&self) -> u32 {
        (self.0.get_range(16..18) + 1) * 2
    }

    pub fn pllq(&self) -> u32 {
        self.0.get_range(24..28)
    }


    /// Division factor for the main PLLs (PLL, PLLI2S and PLLSAI) input clock
    ///
    /// Set and cleared by software to divide the PLL and PLLI2S input clock before the VCO.
    /// These bits can be written only when the PLL and PLLI2S are disabled.
    pub fn set_pllm(&mut self, value: u32) {
        self.0.set_range(0..6, value);
    }

    /// Main PLL (PLL) multiplication factor for VCO
    ///
    /// Set and cleared by software to control the multiplication factor of the VCO. These bits can
    /// be written only when PLL is disabled. Only half-word and word accesses are allowed to
    /// write these bits.
    pub fn set_plln(&mut self, value: u32) {
        self.0.set_range(6..15, value);
    }

    /// Main PLL (PLL) division factor for main system clock
    ///
    /// Set and cleared by software to control the frequency of the general PLL output clock. These
    /// bits can be written only if PLL is disabled.
    pub fn set_pllp(&mut self, value: u32) {
        self.0.set_range(16..18, value / 2 - 1);
    }

    /// Set main PLL(PLL) and audio PLL (PLLI2S) entry clock source to HSE oscillator clock
    ///
    /// Set and cleared by software to select PLL and PLLI2S clock source. This bit can be written
    /// only when PLL and PLLI2S are disabled.
    pub fn set_pllsrc(&mut self, value: bool) {
        self.0.set_bit(22, value);
    }

    /// Main PLL (PLL) division factor for USB OTG FS, SDMMC and random number generator clocks
    ///
    /// Set and cleared by software to control the frequency of USB OTG FS clock, the random
    /// number generator clock and the SDMMC clock. These bits should be written only if PLL is
    /// disabled.
    pub fn set_pllq(&mut self, value: u32) {
        self.0.set_range(24..28, value);
    }
}
