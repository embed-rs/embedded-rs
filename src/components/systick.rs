use ::base::volatile::{Volatile, VolatileStruct};

pub struct SysTick {
    // CTRL
    control: Volatile<u32>,
    // LOAD
    reload: Volatile<u32>,
    // VAL
    value: Volatile<u32>,
    // CALIB (currently not used)
    #[allow(dead_code)]
    calibration: Volatile<u32>,
}

impl SysTick {
    #[inline(always)]
    pub fn enable(&mut self) {
        self.control.write(1 | 4);
    }

    #[inline(always)]
    pub fn disable(&mut self) {
        self.control.write(0 | 4);
    }

    #[inline(always)]
    pub fn count(&self) -> u32 {
        self.value.read()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.value.write(0);  // writing any value will set it to 0
    }

    #[inline(always)]
    pub fn wait_ticks(&mut self, n: u32) {
        self.disable();
        self.reload.write(n);
        self.clear();
        self.enable();

        while self.control.read() & 0x00010000 == 0 {}
    }
}

impl VolatileStruct for SysTick {}
