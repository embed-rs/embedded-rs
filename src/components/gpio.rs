use core::convert::Into;
use core::ops;
use ::base::volatile::{Volatile, VolatileStruct};

#[derive(Clone, Copy, Debug)]
pub enum GPIONum {
    GPIO0 = 0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    GPIO10,
    GPIO11,
    GPIO12,
    GPIO13,
    GPIO14,
    GPIO15,
}

#[derive(Clone, Copy, Debug)]
pub enum GPIOOutput {
    Low,
    High,
}

#[derive(Clone, Copy, Debug)]
pub enum BSSRBit {
    Set(GPIONum),
    Clear(GPIONum),
}

#[derive(Clone, Copy, Debug)]
pub struct BSSRBitSet(u32);

impl Into<BSSRBitSet> for BSSRBit {
    #[inline(always)]
    fn into(self) -> BSSRBitSet {
        BSSRBitSet(match self {
            BSSRBit::Set(n) => 1 << (n as u32),
            BSSRBit::Clear(n) => 1 << 16 << (n as u32),
        })
    }
}

impl ops::BitOr for BSSRBitSet {
    type Output = BSSRBitSet;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        BSSRBitSet(self.0 | rhs.0)
    }
}

impl ops::BitOr for BSSRBit {
    type Output = BSSRBitSet;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let l: BSSRBitSet = self.into();
        let r: BSSRBitSet = rhs.into();
        l | r
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PortMode {
    Input(GPIONum),
    Output(GPIONum),
    Alternate(GPIONum),
    Analog(GPIONum),
}

#[derive(Clone, Copy, Debug)]
pub struct PortModeSet{
    val: u32,
    mask: u32,
}

impl Into<PortModeSet> for PortMode {
    #[inline(always)]
    fn into(self) -> PortModeSet {
        match self {
            PortMode::Input(n) => PortModeSet{
                val: 0b00 << (n as u8 * 2),
                mask: !(0b11 << (n as u8 * 2)),
            },
            PortMode::Output(n) => PortModeSet{
                val: 0b01 << (n as u8 * 2),
                mask: !(0b11 << (n as u8 * 2)),
            },
            PortMode::Alternate(n) => PortModeSet{
                val: 0b10 << (n as u8 * 2),
                mask: !(0b11 << (n as u8 * 2)),
            },
            PortMode::Analog(n) => PortModeSet{
                val: 0b11 << (n as u8 * 2),
                mask: !(0b11 << (n as u8 * 2)),
            },
        }
    }
}

impl ops::BitOr for PortMode {
    type Output = PortModeSet;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let l: PortModeSet = self.into();
        let r: PortModeSet = rhs.into();
        l | r
    }
}

impl ops::BitOr for PortModeSet {
    type Output = PortModeSet;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        PortModeSet{
            val: self.val | rhs.val,
            mask: self.mask & rhs.mask,
        }
    }
}

#[repr(C)]
pub struct GPIOBank {
    // 0x00 MODER
    mode_reg: Volatile<u32>,
    // 0x04 OTYPER
    type_reg: Volatile<u32>,
    // 0x08 SPEEDR
    speed_reg: Volatile<u32>,
    // 0x0C PUPDR
    pull_up_down_reg: Volatile<u32>,
    // 0x10 IDR
    input_reg: Volatile<u32>,
    // 0x14 ODR
    output_reg: Volatile<u32>,
    // 0x18 BSSR
    bssr_reg: Volatile<u32>,
    // 0x1C LCKR
    lock_reg: Volatile<u32>,
    // 0x20 AFRL
    alt_func_reg_low: Volatile<u32>,
    // 0x24 AFRH
    alt_func_reg_high: Volatile<u32>,
}

impl GPIOBank {
    #[inline(always)]
    pub fn set_output<T: Into<BSSRBitSet>>(&mut self, bits: T) {
        let bssr: BSSRBitSet = bits.into();
        self.bssr_reg.write(bssr.0);
    }

    #[inline(always)]
    pub fn set_mode<T: Into<PortModeSet>>(&mut self, bits: T) {
        // FIXME: looking at the disassembly, this ends up as
        // 0xC3FFFFFE instead of 0xC3FFFFFF for some reason. why is the lowest
        // bit wrong?
        let mode: PortModeSet = bits.into();
        let prev = self.mode_reg.read() & mode.mask;
        self.mode_reg.write(prev | mode.val);
    }

    pub fn split(&'static mut self) -> GPIOPins {
        // FIXME: this works, but it would be nice if we could return Input/Output
        //        types to have safety regarding pin direction
        // FIXME: Add PhantomData to keep a reference to GPIOBank in Pins. Goal
        //        Use non-static mut?
        // FIXEDME: turns out, it's fine.
        GPIOPins{
            pin_0: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO0),
            pin_1: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO1),
            pin_2: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO2),
            pin_3: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO3),
            pin_4: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO4),
            pin_5: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO5),
            pin_6: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO6),
            pin_7: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO7),
            pin_8: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO8),
            pin_9: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO9),
            pin_10: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO10),
            pin_11: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO11),
            pin_12: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO12),
            pin_13: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO13),
            pin_14: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO14),
            pin_15: GPIOPin::from_bank(self as *mut GPIOBank, GPIONum::GPIO15),
        }
    }
}

impl VolatileStruct for GPIOBank {}

pub struct GPIOPin {
    bank: *mut GPIOBank,
    n: GPIONum,
}

impl GPIOPin {
    fn from_bank(bank: *mut GPIOBank, n: GPIONum) -> GPIOPin {
        GPIOPin{
            bank: bank,
            n: n,
        }
    }

    // unsafe, because it requires shared access to the bank
    pub unsafe fn output(self) -> GPIOOutputPin {
        (&mut *self.bank).set_mode(PortMode::Output(self.n));
        GPIOOutputPin{
            bank: self.bank,
            n: self.n,
        }
    }
}

// TODO: Check how much memory this uses. Ideally, it would just be a static
//       immediate
pub struct GPIOOutputPin {
    bank: *mut GPIOBank,
    n: GPIONum,
}

impl GPIOOutputPin {
    // FIXME: is it safe to perform this on & instead of &mut?
    pub fn set_output(&self, value: GPIOOutput) {
        unsafe {
            (&mut *self.bank).set_output(match value {
                GPIOOutput::Low => BSSRBit::Clear(self.n),
                GPIOOutput::High => BSSRBit::Set(self.n),
            });
        }
    }
}

pub struct GPIOPins {
    pub pin_0: GPIOPin,
    pub pin_1: GPIOPin,
    pub pin_2: GPIOPin,
    pub pin_3: GPIOPin,
    pub pin_4: GPIOPin,
    pub pin_5: GPIOPin,
    pub pin_6: GPIOPin,
    pub pin_7: GPIOPin,
    pub pin_8: GPIOPin,
    pub pin_9: GPIOPin,
    pub pin_10: GPIOPin,
    pub pin_11: GPIOPin,
    pub pin_12: GPIOPin,
    pub pin_13: GPIOPin,
    pub pin_14: GPIOPin,
    pub pin_15: GPIOPin,
}

// FIXME: next up, think about wrapping an "internal" object with an external
//        api (private methods good here)
#[macro_export]
macro_rules! gpio_setup {
    ($pins:expr , { $($var_name:ident = $direction:ident($pin_name:ident);)* } )
    => (
        // need:
        // #[allow(non_shorthand_field_patterns)]
        // somehow here or globally
        let GPIOPins{
            $($pin_name: $var_name,)*
            ..
        } = $pins;

        $(let $var_name = unsafe { $var_name.$direction() })*;
    )
}
