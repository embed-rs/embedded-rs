//! General Purpose I/O

mod mode;
mod input_data;
mod output_data;
mod bit_set_reset;
mod out_speed;
mod out_type;
mod resistor;
mod alternate_fn;

pub use self::mode::{ModeRegister, Mode};
pub use self::input_data::InputDataRegister;
pub use self::output_data::OutputDataRegister;
pub use self::bit_set_reset::BitSetResetRegister;
pub use self::out_type::{OutputTypeRegister, OutputType};
pub use self::out_speed::{OutputSpeedRegister, OutputSpeed};
pub use self::resistor::{ResistorRegister, Resistor};
pub use self::alternate_fn::{AlternateFunctionRegister, AlternateFunction};

use volatile::{ReadOnly, WriteOnly, ReadWrite};

#[repr(C)]
pub struct Gpio {
    pub mode: ReadWrite<mode::ModeRegister>,
    pub out_type: ReadWrite<out_type::OutputTypeRegister>,
    pub out_speed: ReadWrite<out_speed::OutputSpeedRegister>,
    pub pupd: ReadWrite<resistor::ResistorRegister>,

    // 0x10
    pub input_data: ReadOnly<input_data::InputDataRegister>,
    pub output_data: ReadOnly<output_data::OutputDataRegister>,
    pub bit_set_reset: WriteOnly<bit_set_reset::BitSetResetRegister>,
    pub lckr: ReadWrite<u32>,

    // 0x20
    pub alternate_fn: ReadWrite<alternate_fn::AlternateFunctionRegister>,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Pin {
    Pin0 = 0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
}
