// see http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0646b/Bhccjgga.html

use components::rcc::RccBank;
use volatile::Volatile;

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
    pub fn setup(&'static mut self, rcc: &RccBank, enable_interrupt: bool) -> SysTick {
        // Progam SysTick
        let pll_cfgr = rcc.pll_cfgr.read();
        let pllm = pll_cfgr.pllm();
        let plln = pll_cfgr.plln();
        let pllp = pll_cfgr.pllp();
        self.rvr.update(|r| r.set(25 * 1000 / pllm * plln / pllp - 1)); // hse runs at 25 MHz
        self.cvr.update(|r| r.clear());

        let mut flags = self::csr::CLKSOURCE | self::csr::ENABLE;
        if enable_interrupt {
            flags |= self::csr::TICKINT;
        }
        self.csr.write(flags);

        SysTick(self)
    }
}

pub struct SysTick(&'static mut SysTickBank);

impl SysTick {
    pub fn busy_wait(&self, milliseconds: u32) {
        for _ in 0..milliseconds {
            while !self.0.csr.read().contains(csr::COUNTFLAG) {}
        }
    }
}
