//! AHB1 peripheral clock register (RCC_AHB1ENR)

bitflags! {
    pub flags Register: u32 {
        const GPIO_A_ENABLE = 1 << 0,
        const GPIO_B_ENABLE = 1 << 1,
        const GPIO_C_ENABLE = 1 << 2,
        const GPIO_D_ENABLE = 1 << 3,
        const GPIO_E_ENABLE = 1 << 4,
        const GPIO_F_ENABLE = 1 << 5,
        const GPIO_G_ENABLE = 1 << 6,
        const GPIO_H_ENABLE = 1 << 7,
        const GPIO_I_ENABLE = 1 << 8,
        const GPIO_J_ENABLE = 1 << 9,
        const GPIO_K_ENABLE = 1 << 10,

        const CRC_ENABLE = 1 << 12,
        const BKPSRAM_ENABLE = 1 << 18,
        const DTCMRAM_ENABLE = 1 << 20,
        const DMA1_ENABLE = 1 << 21,
        const DMA2_ENABLE = 1 << 22,
        const DMA2D_ENABLE = 1 << 23,
        const ETHMAC_ENABLE = 1 << 25,
        const ETHMAC_TX_ENABLE = 1 << 26,
        const ETHMAC_RX_ENABLE = 1 << 27,
        const ETHMAC_PTP_ENABLE = 1 << 28,
        const OTG_HS_ENABLE = 1 << 29,
        const OTG_HSULPI_ENABLE = 1 << 30,
    }
}
