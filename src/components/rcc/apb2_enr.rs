//! RCC APB2 peripheral clock enable register (RCC_APB2ENR)

bitflags! {
    pub flags Register: u32 {
        const TIM_1_ENABLE = 1 << 0,
        const TIM_8_ENABLE = 1 << 1,
        const USART_1_ENABLE = 1 << 4,
        const USART_6_ENABLE = 1 << 5,
        const ADC_1_ENABLE = 1 << 8,
        const ADC_2_ENABLE = 1 << 9,
        const ADC_3_ENABLE = 1 << 10,
        const SDMMC_1_ENABLE = 1 << 11,
        const SPI_1_ENABLE = 1 << 12,
        const SPI_4_ENABLE = 1 << 13,
        const SYSCFG_ENABLE = 1 << 14,
        const TIM_9_ENABLE = 1 << 16,
        const TIM_10_ENABLE = 1 << 17,
        const TIM_11_ENABLE = 1 << 18,
        const SPI_5_ENABLE = 1 << 20,
        const SPI_6_ENABLE = 1 << 21,
        const SAI_1_ENABLE = 1 << 22,
        const SAI_2_ENABLE = 1 << 23,
        const LTDC_ENABLE = 1 << 26,
    }
}
