//! Flexible Memory Controller

use base::volatile::{Volatile, VolatileStruct};

pub mod sdcr;
mod sdtr;
pub mod sdcmr;
mod sdsr;
mod sdrtr;

#[repr(C)]
pub struct FmcBank {
    bcr_1: u32,
    btr_1: u32,
    bcr_2: u32,
    btr_2: u32,

    // 0x10
    bcr_3: u32,
    btr_3: u32,
    bcr_4: u32,
    btr_4: u32,

    // 0x20
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
    _pad4: u32,

    // 0x30
    _pad5: u32,
    _pad6: u32,
    _pad7: u32,
    _pad8: u32,

    // 0x40
    _pad9: u32,
    _pad10: u32,
    _pad11: u32,
    _pad12: u32,

    // 0x50
    _pad13: u32,
    _pad14: u32,
    _pad15: u32,
    _pad16: u32,

    // 0x60
    _pad17: u32,
    _pad18: u32,
    _pad19: u32,
    _pad20: u32,

    // 0x70
    _pad21: u32,
    _pad22: u32,
    _pad23: u32,
    _pad24: u32,

    // 0x80
    pcr: u32,
    sr: u32,
    pmem: u32,
    patt: u32,

    // 0x90
    _pad25: u32,
    eccr: u32,
    _pad26: u32,
    _pad27: u32,

    // 0xa0
    _pad28: u32,
    _pad29: u32,
    _pad30: u32,
    _pad31: u32,

    // 0xb0
    _pad32: u32,
    _pad33: u32,
    _pad34: u32,
    _pad35: u32,

    // 0xc0
    _pad36: u32,
    _pad37: u32,
    _pad38: u32,
    _pad39: u32,

    // 0xd0
    _pad40: u32,
    _pad41: u32,
    _pad42: u32,
    _pad43: u32,

    // 0xe0
    _pad44: u32,
    _pad45: u32,
    _pad46: u32,
    _pad47: u32,

    // 0xf0
    _pad48: u32,
    _pad49: u32,
    _pad50: u32,
    _pad51: u32,

    // 0x100
    _pad52: u32,
    bwtr_1: u32,
    _pad53: u32,
    bwtr_2: u32,

    // 0x110
    _pad54: u32,
    bwtr_3: u32,
    _pad55: u32,
    bwtr_4: u32,

    // 0x120
    _pad56: u32,
    _pad57: u32,
    _pad58: u32,
    _pad59: u32,

    // 0x130
    _pad60: u32,
    _pad61: u32,
    _pad62: u32,
    _pad63: u32,

    // 0x140
    pub sdcr_1: Volatile<sdcr::Register>,
    pub sdcr_2: Volatile<sdcr::Register>,
    pub sdtr_1: Volatile<sdtr::Register>,
    pub sdtr_2: Volatile<sdtr::Register>,

    // 0x150
    sdcmr: Volatile<sdcmr::Register>,
    pub sdrtr: Volatile<u32>,
    sdsr: Volatile<sdsr::Register>,
}

impl FmcBank {
    pub fn send_command(&mut self,
                        bank: Bank,
                        command: sdcmr::Command,
                        auto_refresh: u8,
                        modereg: u16) {
        assert!(!self.sdsr.read().contains(sdsr::BUSY));

        self.sdcmr.update(|cmr| {
            match bank {
                Bank::One => cmr.set_ctb1(true),
                Bank::Two => cmr.set_ctb2(true),
                Bank::Both => {
                    cmr.set_ctb1(true);
                    cmr.set_ctb2(true);
                }
            }

            cmr.set_mode(command);
            cmr.set_number_of_auto_refresh(auto_refresh);
            cmr.set_mode_register_definition(modereg);
        });

        while self.sdsr.read().contains(sdsr::BUSY) {
            // wait
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Bank {
    One,
    Two,
    Both,
}

impl VolatileStruct for FmcBank {}
