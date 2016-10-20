//! SDRAM Timing Register

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    /// The bits of the TRCD register define the delay between the Activate command and a
    /// Read/Write command in number of memory clock cycles.
    pub fn set_row_to_column_delay(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(24..28, cycles as u32 - 1);
    }

    /// The bits of the TRP register define the delay between a Precharge command and another
    /// command in number of memory clock cycles. The TRP timing is only configured in the
    /// FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the
    /// timing of the slowest device.
    pub fn set_row_precharge_delay(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(20..24, cycles as u32 - 1);
    }

    /// The bits of the TWR register define the delay between a Write and a Precharge command in
    /// number of memory clock cycles.
    ///
    /// TWR must be programmed to match the write recovery time (t_WR) defined in the SDRAM
    /// datasheet, and to guarantee that:
    /// TWR >= TRAS - TRCD and TWR >= TRC - TRCD - TRP
    ///
    /// Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to
    /// 0x1.
    ///
    /// If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed
    /// with the same TWR timing corresponding to the slowest SDRAM device.
    pub fn set_recovery_delay(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(16..20, cycles as u32 - 1);
    }

    /// The bits of the TRC register define the delay between the Refresh command and the Activate
    /// command, as well as the delay between two consecutive Refresh commands. It is expressed in
    /// number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register.
    /// If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest
    /// device.
    pub fn set_row_cycle_delay(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(12..16, cycles as u32 - 1);
    }

    /// The bits of the TRAS register define the minimum Self-refresh period in number of memory
    /// clock cycles.
    pub fn set_self_refresh_time(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(8..12, cycles as u32 - 1);
    }

    /// The bits of the TXSR register define the delay from releasing the Self-refresh command
    /// to issuing the Activate command in number of memory clock cycles.
    pub fn set_exit_self_refresh_delay(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(4..8, cycles as u32 - 1);
    }

    /// The bits of the TMRD register define the delay between a Load Mode Register command and
    /// an Active or Refresh command in number of memory clock cycles.
    ///
    /// Note: If two SDRAM devices are connected, all the accesses performed simultaneously to both
    /// devices by the Command Mode register (Load Mode Register command) are issued using the
    /// timing parameters configured for Bank1 (TMRD and TRAS timings) in the FMC_SDTR1 register.
    ///
    /// The TRP and TRC timings are only configured in the FMC_SDTR1 register. If two SDRAM
    /// devices are used, the TRP and TRC timings must be programmed with the timings of the
    /// slowest device
    pub fn set_load_mode_register_to_active(&mut self, cycles: u8) {
        assert!(cycles >= 1 && cycles <= 16);
        self.0.set_range(0..4, cycles as u32 - 1);
    }
}
