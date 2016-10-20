//! SDRAM Command Mode register (FMC_SDCMR)
//!
//! This register contains the command issued when the SDRAM device is accessed. This register is
//! used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes.
//! As soon as the MODE field is written, the command will be issued only to one or to both SDRAM
//! banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    pub fn set_mode(&mut self, mode: Command) {
        self.0.set_range(0..3, mode as u32);
    }

    pub fn set_ctb1(&mut self, enable: bool) {
        self.0.set_bit(4, enable);
    }

    pub fn set_ctb2(&mut self, enable: bool) {
        self.0.set_bit(3, enable);
    }

    pub fn set_number_of_auto_refresh(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);

        self.0.set_range(5..9, cycles as u32 - 1);
    }

    pub fn set_mode_register_definition(&mut self, value: u16) {
        assert!(value < 8192);
        self.0.set_range(9..22, value as u32);
    }
}

/// When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be
/// set otherwise the command will be ignored.
///
/// Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued
/// simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will
/// be ignored.
///
/// Note: If only one SDRAM bank is used and a command is issued with it’s associated CTB bit
/// set, the other CTB bit of the the unused bank must be kept to 0.
#[repr(u32)]
pub enum Command {
    Normal = 0b000,
    ClockConfigurationEnable = 0b001,
    PrechargeAllCommand = 0b010,
    AutoRefreshCommand = 0b011,
    LoadModeRegister = 0b100,
    SelfRefreshCommand = 0b101,
    PowerDownCommand = 0b110,
}

bitflags!{
    pub flags ModeRegister: u16 {
        const BURST_LENGTH_1                   = 0x0000,
        const BURST_LENGTH_2                   = 0x0001,
        const BURST_LENGTH_4                   = 0x0002,
        const BURST_LENGTH_8                   = 0x0004,
        const BURST_TYPE_SEQUENTIAL            = 0x0000,
        const BURST_TYPE_INTERLEAVED           = 0x0008,
        const CAS_LATENCY_2                    = 0x0020,
        const CAS_LATENCY_3                    = 0x0030,
        const OPERATING_MODE_STANDARD          = 0x0000,
        const WRITEBURST_MODE_PROGRAMMED       = 0x0000 ,
        const WRITEBURST_MODE_SINGLE           = 0x0200 ,
    }
}
