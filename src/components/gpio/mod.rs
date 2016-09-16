//! General Purpose I/O

mod mode;
mod idr;
mod odr;
mod bsrr;
mod out_speed;
mod out_type;
mod resistor;
mod afr;

pub use self::mode::ModeBits;
pub use self::out_type::Type;
pub use self::out_speed::Speed;
pub use self::resistor::Resistor;
pub use self::afr::AlternateFunction;

use base::volatile::{Volatile, VolatileStruct};
use core::marker::PhantomData;

pub trait GpioPort {}

pub enum PortA {}
pub enum PortB {}
pub enum PortC {}
pub enum PortD {}
pub enum PortE {}
pub enum PortF {}
pub enum PortG {}
pub enum PortH {}
pub enum PortI {}
pub enum PortJ {}
pub enum PortK {}

impl GpioPort for PortA {}
impl GpioPort for PortB {}
impl GpioPort for PortC {}
impl GpioPort for PortD {}
impl GpioPort for PortE {}
impl GpioPort for PortF {}
impl GpioPort for PortG {}
impl GpioPort for PortH {}
impl GpioPort for PortI {}
impl GpioPort for PortJ {}
impl GpioPort for PortK {}

#[repr(C)]
pub struct GpioBank<P: GpioPort> {
    mode: Volatile<mode::Register>,
    out_type: Volatile<out_type::Register>,
    out_speed: Volatile<out_speed::Register>,
    pupd: Volatile<resistor::Register>,

    // 0x10
    idr: Volatile<idr::Register>,
    odr: Volatile<odr::Register>,
    bsrr: bsrr::BitSetResetRegister,
    lckr: u32,

    // 0x20
    afr: Volatile<afr::Register>,
    _phantom: PhantomData<P>,
}

impl<P> VolatileStruct for GpioBank<P> where P: GpioPort {}

impl<P> GpioBank<P>
    where P: GpioPort
{
    pub fn split(&'static mut self) -> (GpioController<P>, GpioPins<P>) {
        let &mut GpioBank { ref mut mode,
                            ref mut out_type,
                            ref mut out_speed,
                            ref mut pupd,
                            ref idr,
                            ref bsrr,
                            ref mut afr,
                            .. } = self;

        let bank_ref = GpioController {
            mode: mode,
            out_type: out_type,
            out_speed: out_speed,
            pupd: pupd,
            afr: afr,
            idr: idr,
            bsrr: bsrr,
            _phantom: PhantomData,
        };

        let pins = GpioPins {
            pin_0: GpioPin::new(PinNumber::Pin0),
            pin_1: GpioPin::new(PinNumber::Pin1),
            pin_2: GpioPin::new(PinNumber::Pin2),
            pin_3: GpioPin::new(PinNumber::Pin3),
            pin_4: GpioPin::new(PinNumber::Pin4),
            pin_5: GpioPin::new(PinNumber::Pin5),
            pin_6: GpioPin::new(PinNumber::Pin6),
            pin_7: GpioPin::new(PinNumber::Pin7),
            pin_8: GpioPin::new(PinNumber::Pin8),
            pin_9: GpioPin::new(PinNumber::Pin9),
            pin_10: GpioPin::new(PinNumber::Pin10),
            pin_11: GpioPin::new(PinNumber::Pin11),
            pin_12: GpioPin::new(PinNumber::Pin12),
            pin_13: GpioPin::new(PinNumber::Pin13),
            pin_14: GpioPin::new(PinNumber::Pin14),
            pin_15: GpioPin::new(PinNumber::Pin15),
        };

        (bank_ref, pins)
    }
}

pub struct GpioController<P: GpioPort> {
    mode: &'static mut Volatile<mode::Register>,
    out_type: &'static mut Volatile<out_type::Register>,
    out_speed: &'static mut Volatile<out_speed::Register>,
    pupd: &'static mut Volatile<resistor::Register>,
    afr: &'static mut Volatile<afr::Register>,
    idr: &'static Volatile<idr::Register>,
    bsrr: &'static bsrr::BitSetResetRegister,
    _phantom: PhantomData<P>,
}

impl<P> GpioController<P>
    where P: GpioPort
{
    pub fn to_read(&mut self, pin: GpioPin<P>, resistor: Resistor) -> GpioRead {
        let pins = &[pin.pin_number];

        self.pupd.update(|r| r.set(pins, resistor));
        self.mode.update(|r| r.set(pins, ModeBits::Input));

        GpioRead {
            idr: self.idr,
            pin: pin.pin_number,
        }
    }

    pub fn to_write(&mut self,
                    pin: GpioPin<P>,
                    typ: Type,
                    speed: Speed,
                    resistor: Resistor)
                    -> GpioWrite {
        let pins = &[pin.pin_number];

        self.pupd.update(|r| r.set(pins, resistor));
        self.out_type.update(|r| r.set(pins, typ));
        self.out_speed.update(|r| r.set(pins, speed));
        self.mode.update(|r| r.set(pins, ModeBits::Output));

        GpioWrite {
            bsrr: self.bsrr,
            pin: pin.pin_number,
        }
    }

    pub fn to_alternate_function(&mut self,
                                 pin: GpioPin<P>,
                                 typ: Type,
                                 speed: Speed,
                                 alternate_fn: AlternateFunction,
                                 resistor: Resistor) {
        let pins = &[pin.pin_number];

        self.pupd.update(|r| r.set(pins, resistor));
        self.out_type.update(|r| r.set(pins, typ));
        self.out_speed.update(|r| r.set(pins, speed));
        self.afr.update(|r| r.set(pins, alternate_fn));
        self.mode.update(|r| r.set(pins, ModeBits::AlternateFunction));
    }
}

pub struct GpioPins<P: GpioPort> {
    pub pin_0: GpioPin<P>,
    pub pin_1: GpioPin<P>,
    pub pin_2: GpioPin<P>,
    pub pin_3: GpioPin<P>,
    pub pin_4: GpioPin<P>,
    pub pin_5: GpioPin<P>,
    pub pin_6: GpioPin<P>,
    pub pin_7: GpioPin<P>,
    pub pin_8: GpioPin<P>,
    pub pin_9: GpioPin<P>,
    pub pin_10: GpioPin<P>,
    pub pin_11: GpioPin<P>,
    pub pin_12: GpioPin<P>,
    pub pin_13: GpioPin<P>,
    pub pin_14: GpioPin<P>,
    pub pin_15: GpioPin<P>,
}

pub struct GpioPin<P: GpioPort> {
    pin_number: PinNumber,
    _phantom: PhantomData<P>,
}

impl<P> GpioPin<P>
    where P: GpioPort
{
    fn new(pin_number: PinNumber) -> GpioPin<P> {
        GpioPin {
            pin_number: pin_number,
            _phantom: PhantomData,
        }
    }
}

pub struct GpioRead {
    idr: &'static Volatile<idr::Register>,
    pin: PinNumber,
}

impl GpioRead {
    pub fn read(&self) -> bool {
        self.idr.read().get(self.pin)
    }
}

pub struct GpioWrite {
    bsrr: &'static bsrr::BitSetResetRegister,
    pin: PinNumber,
}

impl GpioWrite {
    pub fn set(&mut self, high: bool) {
        if high {
            self.bsrr.set(self.pin)
        } else {
            self.bsrr.reset(self.pin)
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PinNumber {
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
