//! Power controller (PWR)

use volatile::Volatile;

pub mod cr1;
pub mod csr1;

#[repr(C)]
pub struct PwrBank {
    pub cr1: Volatile<cr1::Register>,
    pub csr1: Volatile<csr1::Register>,
    cr2: u32,
    csr2: u32,
}
