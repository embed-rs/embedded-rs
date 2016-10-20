//! SDRAM Control Register

use bit_field::BitField;

#[derive(Debug, Clone, Copy)]
pub struct Register(BitField<u32>);

impl Register {
    pub fn set_number_of_column_address_bits(&mut self, number: u32) {
        assert!(number >= 8 && number < 12);
        self.0.set_range(0..2, number - 8);
    }

    pub fn set_number_of_row_address_bits(&mut self, number: u32) {
        assert!(number >= 11 && number < 14);
        self.0.set_range(2..4, number - 11);
    }

    pub fn set_data_bus_width(&mut self, width: u32) {
        let value = match width {
            8 => 0b00,
            16 => 0b01,
            32 => 0b10,
            _ => panic!("invalid width"),
        };
        self.0.set_range(4..6, value);
    }

    pub fn set_number_of_intern_banks(&mut self, number: u32) {
        let value = match number {
            2 => 0,
            4 => 1,
            _ => panic!("invalid number"),
        };
        self.0.set_range(6..7, value);
    }

    pub fn set_cas_latency(&mut self, cycles: u32) {
        assert!(cycles >= 1 && cycles < 4);
        self.0.set_range(7..9, cycles);
    }

    pub fn set_write_protection(&mut self, enable: bool) {
        self.0.set_bit(9, enable);
    }

    pub fn disable_sdram_clock(&mut self) {
        self.0.set_range(10..12, 0);
    }

    pub fn enable_sdram_clock(&mut self, period: u32) {
        assert!(period == 2 || period == 3);
        self.0.set_range(10..12, period);
    }

    pub fn set_burst_read(&mut self, enable: bool) {
        self.0.set_bit(12, enable);
    }
}
