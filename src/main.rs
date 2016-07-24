#![no_std]

#[macro_use]
extern crate stm32;
use stm32::util::nop;

#[macro_use]
pub mod gpio;
pub mod systick;
use gpio::BSSRBit::*;
use gpio::GPIONum::*;
use gpio::PortMode::*;
use gpio::GPIOOutput::*;
pub mod register;
use register::{Volatile, VolatileStruct};

board!(stm32f429i, {
    hard_fault: None,
    mem_manage: None
});

// on AHB1, GPIOG is at 0x4002 1800 - 0x4002 1BFF (DM00071990, pg. 86)
const AHB1_GPIOG_BASE: u32 = 0x4002_1800;

// on AHB1, RCC is at 0x4002 3800 - 0x4002 3BFF RCC (DM00071990, pg. 86)
const AHB1_RCC_BASE: u32 = 0x4002_3800;

// base address of systick timer (guide pg. 313ff)
const SYSTICK_BASE: u32 = 0xE000E010;

// DM00031020.pdf, pg 180
const RCC_AHB1ENR: u32 = AHB1_RCC_BASE + 0x30;
// to enable GPIOG, we need to toggle the 6th bit
const RCC_AHB1ENR_GPIOG_EN: u32 = 0b0100_0000;

// in gdb, the following steps work as well
// 1. set {int} 0x40023830 = 0x001000FF
// 2. set {int} 0x40021800 = 0x14000000
// 3. set {int} 0x40021814 = 0xFFFFFFFF

// FIXME: document in thesis: ease of use due to calling main instead of
// wrapping macro; arguments in favor - better compiler error messages ("real
// functions" vs macros, can see arguments while writing code, etc.); drawback:
// macro hygiene.

#[repr(C)]
struct SingleItemReg<T> {
    reg: Volatile<T>
}

impl<T> VolatileStruct for SingleItemReg<T> {}

impl<T> SingleItemReg<T> {
    pub fn read(&self) -> T {
        self.reg.read()
    }

    pub fn write(&mut self, src: T) {
        self.reg.write(src)
    }
}

fn main() {
    let mut rcc_ahb1enr = unsafe { SingleItemReg::from_ptr(RCC_AHB1ENR as *mut SingleItemReg<u32>) };
    let mut gpio_g = unsafe { gpio::GPIOBank::from_ptr(AHB1_GPIOG_BASE as *mut gpio::GPIOBank) };

    let mut stt = unsafe { systick::SysTick::from_ptr(SYSTICK_BASE as *mut systick::SysTick) };

    // enable clock on gpio register
    rcc_ahb1enr.reg |= RCC_AHB1ENR_GPIOG_EN;

    // setup gpio pins 13 and 14
    gpio_setup!(gpio_g.split(), {
        pin_13 = output(pin_13);
        pin_14 = output(pin_14);
    });

    loop {
        // turn on 13,14
        pin_13.set_output(High);
        stt.wait_ticks(INITIAL_CPU_FREQ as u32);
        pin_14.set_output(High);
        stt.wait_ticks(INITIAL_CPU_FREQ as u32);

        // turn off 13,14
        pin_13.set_output(Low);
        stt.wait_ticks(INITIAL_CPU_FREQ as u32);
        pin_14.set_output(Low);
        stt.wait_ticks(INITIAL_CPU_FREQ as u32);
    }
}
