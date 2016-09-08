use ::base::volatile::{Volatile, VolatileStruct};

pub mod cr;
pub mod pll_cfgr;
pub mod cfgr;
pub mod ahb1_enr;
pub mod ahb3_enr;
pub mod apb1_enr;
pub mod apb2_enr;
pub mod ahb1_rstr;
pub mod ahb3_rstr;
pub mod pllsaicfgr;
pub mod plli2scfgr;
pub mod dckcfgr1;

#[repr(C)]
pub struct RccBank {
    pub cr: Volatile<cr::Register>,
    pub pll_cfgr: Volatile<pll_cfgr::Register>,
    pub cfgr: Volatile<cfgr::Register>,
    cir: u32,

    // 0x10
    pub ahb1_rstr: Volatile<ahb1_rstr::Register>,
    ahb2_rstr: u32,
    pub ahb3_rstr: Volatile<ahb3_rstr::Register>,
    _pad1: u32,

    // 0x20
    apb1_rstr: u32,
    apb2_rstr: u32,
    _pad2: u32,
    _pad3: u32,

    // 0x30
    pub ahb1_enr: Volatile<ahb1_enr::Register>,
    ahb2_enr: u32,
    pub ahb3_enr: Volatile<ahb3_enr::Register>,
    _pad4: u32,

    // 0x40
    pub apb1_enr: Volatile<apb1_enr::Register>,
    pub apb2_enr: Volatile<apb2_enr::Register>,
    _pad5: u32,
    _pad6: u32,

    // 0x50
    ahb1_lpenr: u32,
    ahb2_lpenr: u32,
    ahb3_lpenr: u32,
    _pad7: u32,

    // 0x60
    apb1_lpenr: u32,
    apb2_lpenr: u32,
    _pad8: u32,
    _pad9: u32,

    // 0x70
    bdcr: u32,
    csr: u32,
    _pad10: u32,
    _pad11: u32,

    // 0x80
    sscgr: u32,
    pub plli2scfgr: Volatile<plli2scfgr::Register>,
    pub pllsaicfgr: Volatile<pllsaicfgr::Register>,
    pub dckcfgr1: Volatile<dckcfgr1::Register>,

    // 0x90
    dckcfgr2: u32,
}

impl RccBank {}

impl VolatileStruct for RccBank {}
