//! PWR power control register (PWR_CR1)

bitflags! {
    pub flags Register: u32 {
        const LPDS = 1 << 0,
        const PDDS = 1 << 1,
        const CSBF = 1 << 3,
        const PVDE = 1 << 4,
        const PLS_0 = 1 << 5,
        const PLS_1 = 1 << 6,
        const PLS_2 = 1 << 7,
        const DBP = 1 << 8,
        const FPDS = 1 << 9,
        const LPUDS = 1 << 10,
        const MRUDS = 1 <<11,
        const ADCDC_1 = 1 << 13,
        const VOS_0 = 1 << 14,
        const VOS_1 = 1 << 15,
        const ODEN = 1 << 16,
        const ODSWEN = 1 << 17,
        const UDEN_0 = 1 << 18,
        const UDEN_1 = 1 << 19,
    }
}
