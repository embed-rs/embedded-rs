//! SysTick Control and Status Register

bitflags! {
    pub flags Register: u32 {
        const ENABLE = 1 << 0,
        const TICKINT = 1 << 1,
        const CLKSOURCE = 1 << 2, // 0: external clock, 1: processor clock
        const COUNTFLAG = 1 << 16,
    }
}
