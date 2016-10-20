//! SDRAM Status register (FMC_SDSR)

bitflags! {
    pub flags Register: u32 {
        const REFRESH_ERROR = 1 << 0,
        const MODE_BANK_1_0 = 1 << 1,
        const MODE_BANK_1_1 = 1 << 2,
        const MODE_BANK_2_0 = 1 << 3,
        const MODE_BANK_2_1 = 1 << 4,
        const BUSY = 1 << 5,
    }
}
