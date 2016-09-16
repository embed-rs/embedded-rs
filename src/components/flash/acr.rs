//! Flash access control register (FLASH_ACR)

#[derive(Debug, Clone, Copy)]
pub struct Register(u32);

impl Register {
    pub fn latency(&mut self) -> u32 {
        self.0 & 0b1111
    }

    pub fn set_latency(&mut self, latency: u32) {
        assert!(latency < 16);
        self.0 |= latency;
    }
}
