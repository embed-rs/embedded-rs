//! SDRAM Refresh Timer register (FMC_SDRTR)
//!
//! This register sets the refresh rate in number of SDCLK clock cycles between the refresh
//! cycles by configuring the Refresh Timer Count value.
//!
//! ```
//! Refresh rate = (COUNT + 1) * SDRAM clock frequency
//!
//! COUNT = (SDRAM refresh period / Number of rows) - 20
//! ```
//!
//! ## Example
//!
//! ```
//! Refresh rate = 64 ms / (8196 rows) = 7.81 μs
//! ```
//!
//! where 64 ms is the SDRAM refresh period.
//!
//! ```
//! 7.81 μs * 64 MHz = 468.6
//! ```
//!
//! The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to
//! obtain a safe margin if an internal refresh request occurs when a read request has been
//! accepted. It corresponds to a COUNT value of ‘0000111000000’ (448).
//!
//! This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This
//! timer generates a refresh pulse when zero is reached. The COUNT value must be set at
//! least to 41 SDRAM clock cycles.
//!
//! As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value
//! programmed in the register is ’0’, no refresh is carried out. This register must not be
//! reprogrammed after the initialization procedure to avoid modifying the refresh rate.
//!
//! Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.
//!
//! If a memory access is in progress, the Auto-refresh request is delayed. However, if the
//! memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes
//! precedence. If the memory access occurs during a refresh operation, the request is buffered
//! to be processed when the refresh is complete.
//!
//! This register is common to SDRAM bank 1 and bank 2.

#[derive(Debug, Clone, Copy)]
pub struct Register(u32);

impl Register {
    // TODO
}
