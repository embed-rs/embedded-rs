//! RCC clock control register (RCC_CR)

bitflags! {
    pub flags Register: u32 {
        const HSI_ON = 1 << 0,
        const HSI_RDY = 1 << 1,
        const HSE_ON = 1 << 16,
        const HSE_RDY = 1 << 17,
        const HSE_BYP = 1 << 18,
        const CSS_ON = 1 << 19,
        const PLL_ON = 1 << 24,
        const PLL_RDY = 1 << 25,
        const PLLI2S_ON = 1 << 26,
        const PLLI2S_RDY = 1 << 27,
        const PLLSAI_ON = 1 << 28,
        const PLLSAI_RDY = 1 << 29,

        // etc
    }
}
