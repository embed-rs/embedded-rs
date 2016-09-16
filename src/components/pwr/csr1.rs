//! PWR power control/status register (PWR_CSR1)

bitflags! {
    pub flags Register: u32 {
        const WUIF = 1 << 0,
        const SBF = 1 << 1,
        const PVDO = 1 << 2,
        const BRR = 1 << 3,
        const EIWUP = 1 << 8,
        const BRE = 1 << 9,
        const VOS_RDY = 1 << 14,
        const OD_RDY = 1 << 16,
        const ODSW_RDY = 1 << 17,
        const UD_RDY_0 = 1 << 18,
        const UD_RDY_1 = 1 << 19,
    }
}
