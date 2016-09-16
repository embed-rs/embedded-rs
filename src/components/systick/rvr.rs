//! SysTick Reload Value Register

#[derive(Debug, Clone, Copy)]
pub struct Register(u32);

impl Register {
    pub fn set(&mut self, value: u32) {
        assert!(value & 0xff00_0000 == 0);
        self.0 = value;
    }
}
