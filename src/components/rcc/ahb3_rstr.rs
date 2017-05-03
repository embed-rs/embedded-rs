//! RCC AHB3 peripheral reset register (RCC_AHB3RSTR)

bitflags! {
    pub flags Register: u32 {
        const FMCRST = 1,
        const QSPIRST = 1 << 1,
    }
}
