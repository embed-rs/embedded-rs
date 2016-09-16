//! FLASH registers

use base::volatile::{Volatile, VolatileStruct};

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

impl VolatileStruct for FlashBank {}
