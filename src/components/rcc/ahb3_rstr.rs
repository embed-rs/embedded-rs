//! AHB1 peripheral reset register (RCC_AHB1RSTR)

bitflags! {
    pub flags Register: u32 {
        const FMCRST = 1,
        const QSPIRST = 1 << 1,
    }
}
