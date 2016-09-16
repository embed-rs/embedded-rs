// see http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0646b/Bhccjgga.html

use base::volatile::{Volatile, VolatileStruct};

pub mod csr;
pub mod rvr;
pub mod cvr;

#[repr(C)]
pub struct SysTickBank {
    /// Control and Status Register
    pub csr: Volatile<csr::Register>,
    /// Reload Value Register
    pub rvr: Volatile<rvr::Register>,
    /// Current Value Register
    pub cvr: Volatile<cvr::Register>,
    /// Calibration Register
    calib: u32,
}

impl SysTickBank {
    #[inline(always)]
    pub fn enable(&mut self) {
        self.csr |= csr::ENABLE;
    }

    #[inline(always)]
    pub fn disable(&mut self) {
        self.csr.update(|r| r.remove(csr::ENABLE));
    }

    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.cvr.read().value()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.cvr.update(|r| r.clear());
    }

    #[inline(always)]
    pub fn wait_ticks(&mut self, n: u32) {
        self.disable();
        self.rvr.update(|r| r.set(n));
        self.clear();
        self.enable();

        while !self.csr.read().contains(csr::COUNTFLAG) {}
    }
}

impl VolatileStruct for SysTickBank {}
