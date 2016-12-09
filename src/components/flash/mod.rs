//! FLASH registers

use volatile::Volatile;

pub mod acr;

#[repr(C)]
pub struct FlashBank {
    pub acr: Volatile<acr::Register>,
    keyr: u32,
    opt_keyr: u32,
    sr: u32,

    // 0x10
    cr: u32,
    opt_cr: u32,
    opt_cr1: u32,
}
