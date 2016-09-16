//! SysTick Current Value Register

#[derive(Debug, Clone, Copy)]
pub struct Register(u32);

impl Register {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
