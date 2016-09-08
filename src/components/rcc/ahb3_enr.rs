//! RCC AHB3 peripheral clock enable register (RCC_AHB3ENR)

bitflags! {
    pub flags Register: u32 {
        /// Flexible memory controller clock enable
        const FMC_ENABLE = 1,
        /// Quad SPI memory controller clock enable
        const QSPI_ENABLE = 1 << 1,
    }
}
