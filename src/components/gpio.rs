use svd_board::gpioa::Gpioa;
use svd_board::gpiob::Gpiob;
use svd_board::gpiod::{self, Gpiod};
use core::marker::PhantomData;
use core::mem;

pub struct Gpio<N: GpioPortNumber> {
    registers: Gpiod,
    phantom: PhantomData<N>,
}

impl From<Gpioa> for Gpio<PortA> {
    fn from(registers: Gpioa) -> Self {
        Gpio::new(unsafe { mem::transmute(registers) })
    }
}

impl From<Gpiob> for Gpio<PortB> {
    fn from(registers: Gpiob) -> Self {
        Gpio::new(unsafe { mem::transmute(registers) })
    }
}

impl From<Gpiod> for Gpio<PortD> {
    fn from(registers: Gpiod) -> Self {
        Gpio::new(registers)
    }
}

impl<N: GpioPortNumber> Gpio<N> {
    pub fn new(registers: Gpiod) -> Gpio<N> {
        Gpio {
            registers: registers,
            phantom: PhantomData,
        }
    }

    pub fn split(&'static mut self) -> (GpioPort<N>, GpioPins<N>) {
        let Gpiod { ref mut moder,
                    ref mut otyper,
                    ref mut ospeedr,
                    ref mut pupdr,
                    ref idr,
                    ref mut bsrr,
                    ref mut afrl,
                    ref mut afrh,
                    .. } = self.registers;

        let bank_ref = GpioPort {
            mode: moder,
            out_type: otyper,
            out_speed: ospeedr,
            pupd: pupdr,
            afrl: afrl,
            afrh: afrh,
            idr: idr,
            bsrr: BsrrRef(bsrr),
            phantom: PhantomData,
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

pub struct GpioPort<N: GpioPortNumber> {
    mode: &'static mut gpiod::Moder,
    out_type: &'static mut gpiod::Otyper,
    out_speed: &'static mut gpiod::Ospeedr,
    pupd: &'static mut gpiod::Pupdr,
    afrl: &'static mut gpiod::Afrl,
    afrh: &'static mut gpiod::Afrh,
    idr: &'static gpiod::Idr,
    bsrr: BsrrRef,
    phantom: PhantomData<N>,
}

impl<N: GpioPortNumber> GpioPort<N> {
    pub fn to_read(&mut self, mut pin: GpioPin<N>, resistor: Resistor) -> GpioRead {
        self.set_resistor(&mut pin, resistor);
        self.set_mode(&mut pin, Mode::Input);

        GpioRead {
            idr: self.idr,
            pin: pin.pin_number,
        }
    }

    pub fn to_write(&mut self,
                    mut pin: GpioPin<N>,
                    typ: Type,
                    speed: Speed,
                    resistor: Resistor)
                    -> GpioWrite {

        self.set_resistor(&mut pin, resistor);
        self.set_out_type(&mut pin, typ);
        self.set_out_speed(&mut pin, speed);
        self.set_mode(&mut pin, Mode::Output);

        GpioWrite {
            bsrr: self.bsrr.clone(),
            pin: pin.pin_number,
        }
    }

    pub fn to_alternate_function(&mut self,
                                 mut pin: GpioPin<N>,
                                 typ: Type,
                                 speed: Speed,
                                 alternate_fn: AlternateFunction,
                                 resistor: Resistor) {

        self.set_resistor(&mut pin, resistor);
        self.set_out_type(&mut pin, typ);
        self.set_out_speed(&mut pin, speed);
        self.set_alternate_function(&mut pin, alternate_fn);
        self.set_mode(&mut pin, Mode::AlternateFunction);
    }

    fn set_mode(&mut self, pin: &mut GpioPin<N>, mode: Mode) {
        let set_mode = match pin.pin_number {
            PinNumber::Pin0 => gpiod::ModerW::moder0,
            PinNumber::Pin1 => gpiod::ModerW::moder1,
            PinNumber::Pin2 => gpiod::ModerW::moder2,
            PinNumber::Pin3 => gpiod::ModerW::moder3,
            PinNumber::Pin4 => gpiod::ModerW::moder4,
            PinNumber::Pin5 => gpiod::ModerW::moder5,
            PinNumber::Pin6 => gpiod::ModerW::moder6,
            PinNumber::Pin7 => gpiod::ModerW::moder7,
            PinNumber::Pin8 => gpiod::ModerW::moder8,
            PinNumber::Pin9 => gpiod::ModerW::moder9,
            PinNumber::Pin10 => gpiod::ModerW::moder10,
            PinNumber::Pin11 => gpiod::ModerW::moder11,
            PinNumber::Pin12 => gpiod::ModerW::moder12,
            PinNumber::Pin13 => gpiod::ModerW::moder13,
            PinNumber::Pin14 => gpiod::ModerW::moder14,
            PinNumber::Pin15 => gpiod::ModerW::moder15,
        };
        self.mode.write(|r| set_mode(r, mode as u8));
    }

    fn set_resistor(&mut self, pin: &mut GpioPin<N>, resistor: Resistor) {
        let set_pupd = match pin.pin_number {
            PinNumber::Pin0 => gpiod::PupdrW::pupdr0,
            PinNumber::Pin1 => gpiod::PupdrW::pupdr1,
            PinNumber::Pin2 => gpiod::PupdrW::pupdr2,
            PinNumber::Pin3 => gpiod::PupdrW::pupdr3,
            PinNumber::Pin4 => gpiod::PupdrW::pupdr4,
            PinNumber::Pin5 => gpiod::PupdrW::pupdr5,
            PinNumber::Pin6 => gpiod::PupdrW::pupdr6,
            PinNumber::Pin7 => gpiod::PupdrW::pupdr7,
            PinNumber::Pin8 => gpiod::PupdrW::pupdr8,
            PinNumber::Pin9 => gpiod::PupdrW::pupdr9,
            PinNumber::Pin10 => gpiod::PupdrW::pupdr10,
            PinNumber::Pin11 => gpiod::PupdrW::pupdr11,
            PinNumber::Pin12 => gpiod::PupdrW::pupdr12,
            PinNumber::Pin13 => gpiod::PupdrW::pupdr13,
            PinNumber::Pin14 => gpiod::PupdrW::pupdr14,
            PinNumber::Pin15 => gpiod::PupdrW::pupdr15,
        };
        self.pupd.write(|r| set_pupd(r, resistor as u8));
    }

    fn set_out_type(&mut self, pin: &mut GpioPin<N>, out_type: Type) {
        let set_type = match pin.pin_number {
            PinNumber::Pin0 => gpiod::OtyperW::ot0,
            PinNumber::Pin1 => gpiod::OtyperW::ot1,
            PinNumber::Pin2 => gpiod::OtyperW::ot2,
            PinNumber::Pin3 => gpiod::OtyperW::ot3,
            PinNumber::Pin4 => gpiod::OtyperW::ot4,
            PinNumber::Pin5 => gpiod::OtyperW::ot5,
            PinNumber::Pin6 => gpiod::OtyperW::ot6,
            PinNumber::Pin7 => gpiod::OtyperW::ot7,
            PinNumber::Pin8 => gpiod::OtyperW::ot8,
            PinNumber::Pin9 => gpiod::OtyperW::ot9,
            PinNumber::Pin10 => gpiod::OtyperW::ot10,
            PinNumber::Pin11 => gpiod::OtyperW::ot11,
            PinNumber::Pin12 => gpiod::OtyperW::ot12,
            PinNumber::Pin13 => gpiod::OtyperW::ot13,
            PinNumber::Pin14 => gpiod::OtyperW::ot14,
            PinNumber::Pin15 => gpiod::OtyperW::ot15,
        };
        let value = match out_type {
            Type::PushPull => false,
            Type::OpenDrain => true,
        };
        self.out_type.write(|r| set_type(r, value));
    }

    fn set_out_speed(&mut self, pin: &mut GpioPin<N>, out_speed: Speed) {
        let set_speed = match pin.pin_number {
            PinNumber::Pin0 => gpiod::OspeedrW::ospeedr0,
            PinNumber::Pin1 => gpiod::OspeedrW::ospeedr1,
            PinNumber::Pin2 => gpiod::OspeedrW::ospeedr2,
            PinNumber::Pin3 => gpiod::OspeedrW::ospeedr3,
            PinNumber::Pin4 => gpiod::OspeedrW::ospeedr4,
            PinNumber::Pin5 => gpiod::OspeedrW::ospeedr5,
            PinNumber::Pin6 => gpiod::OspeedrW::ospeedr6,
            PinNumber::Pin7 => gpiod::OspeedrW::ospeedr7,
            PinNumber::Pin8 => gpiod::OspeedrW::ospeedr8,
            PinNumber::Pin9 => gpiod::OspeedrW::ospeedr9,
            PinNumber::Pin10 => gpiod::OspeedrW::ospeedr10,
            PinNumber::Pin11 => gpiod::OspeedrW::ospeedr11,
            PinNumber::Pin12 => gpiod::OspeedrW::ospeedr12,
            PinNumber::Pin13 => gpiod::OspeedrW::ospeedr13,
            PinNumber::Pin14 => gpiod::OspeedrW::ospeedr14,
            PinNumber::Pin15 => gpiod::OspeedrW::ospeedr15,
        };
        self.out_speed.write(|r| set_speed(r, out_speed as u8));
    }

    fn set_alternate_function(&mut self, pin: &mut GpioPin<N>, alternate_fn: AlternateFunction) {
        match pin.pin_number as u8 {
            0...7 => {
                let set_alternate_fn = match pin.pin_number {
                    PinNumber::Pin0 => gpiod::AfrlW::afrl0,
                    PinNumber::Pin1 => gpiod::AfrlW::afrl1,
                    PinNumber::Pin2 => gpiod::AfrlW::afrl2,
                    PinNumber::Pin3 => gpiod::AfrlW::afrl3,
                    PinNumber::Pin4 => gpiod::AfrlW::afrl4,
                    PinNumber::Pin5 => gpiod::AfrlW::afrl5,
                    PinNumber::Pin6 => gpiod::AfrlW::afrl6,
                    PinNumber::Pin7 => gpiod::AfrlW::afrl7,
                    _ => unreachable!(),
                };
                self.afrl.write(|r| set_alternate_fn(r, alternate_fn as u8));
            }
            8...15 => {
                let set_alternate_fn = match pin.pin_number {
                    PinNumber::Pin8 => gpiod::AfrhW::afrh8,
                    PinNumber::Pin9 => gpiod::AfrhW::afrh9,
                    PinNumber::Pin10 => gpiod::AfrhW::afrh10,
                    PinNumber::Pin11 => gpiod::AfrhW::afrh11,
                    PinNumber::Pin12 => gpiod::AfrhW::afrh12,
                    PinNumber::Pin13 => gpiod::AfrhW::afrh13,
                    PinNumber::Pin14 => gpiod::AfrhW::afrh14,
                    PinNumber::Pin15 => gpiod::AfrhW::afrh15,
                    _ => unreachable!(),
                };
                self.afrh.write(|r| set_alternate_fn(r, alternate_fn as u8));
            }
            _ => unreachable!(),
        }
    }
}

pub struct GpioPins<N: GpioPortNumber> {
    pub pin_0: GpioPin<N>,
    pub pin_1: GpioPin<N>,
    pub pin_2: GpioPin<N>,
    pub pin_3: GpioPin<N>,
    pub pin_4: GpioPin<N>,
    pub pin_5: GpioPin<N>,
    pub pin_6: GpioPin<N>,
    pub pin_7: GpioPin<N>,
    pub pin_8: GpioPin<N>,
    pub pin_9: GpioPin<N>,
    pub pin_10: GpioPin<N>,
    pub pin_11: GpioPin<N>,
    pub pin_12: GpioPin<N>,
    pub pin_13: GpioPin<N>,
    pub pin_14: GpioPin<N>,
    pub pin_15: GpioPin<N>,
}

pub struct GpioPin<N: GpioPortNumber> {
    pin_number: PinNumber,
    _phantom: PhantomData<N>,
}

impl<N> GpioPin<N>
    where N: GpioPortNumber
{
    fn new(pin_number: PinNumber) -> GpioPin<N> {
        GpioPin {
            pin_number: pin_number,
            _phantom: PhantomData,
        }
    }
}

pub struct GpioRead {
    idr: &'static gpiod::Idr,
    pin: PinNumber,
}

impl GpioRead {
    pub fn read(&self) -> bool {
        let values = self.idr.read();
        match self.pin {
            PinNumber::Pin0 => values.idr0(),
            PinNumber::Pin1 => values.idr1(),
            PinNumber::Pin2 => values.idr2(),
            PinNumber::Pin3 => values.idr3(),
            PinNumber::Pin4 => values.idr4(),
            PinNumber::Pin5 => values.idr5(),
            PinNumber::Pin6 => values.idr6(),
            PinNumber::Pin7 => values.idr7(),
            PinNumber::Pin8 => values.idr8(),
            PinNumber::Pin9 => values.idr9(),
            PinNumber::Pin10 => values.idr10(),
            PinNumber::Pin11 => values.idr11(),
            PinNumber::Pin12 => values.idr12(),
            PinNumber::Pin13 => values.idr13(),
            PinNumber::Pin14 => values.idr14(),
            PinNumber::Pin15 => values.idr15(),
        }
    }
}

pub struct GpioWrite {
    bsrr: BsrrRef,
    pin: PinNumber,
}

impl GpioWrite {
    pub fn set(&mut self, high: bool) {
        self.bsrr.set(self.pin, high);
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

pub trait GpioPortNumber {}

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

impl GpioPortNumber for PortA {}
impl GpioPortNumber for PortB {}
impl GpioPortNumber for PortC {}
impl GpioPortNumber for PortD {}
impl GpioPortNumber for PortE {}
impl GpioPortNumber for PortF {}
impl GpioPortNumber for PortG {}
impl GpioPortNumber for PortH {}
impl GpioPortNumber for PortI {}
impl GpioPortNumber for PortJ {}
impl GpioPortNumber for PortK {}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Resistor {
    NoPull = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    PushPull = 0,
    OpenDrain = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Speed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    VeryHigh = 0b11,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AlternateFunction {
    AF0 = 0b0000,
    AF1 = 0b0001,
    AF2 = 0b0010,
    AF3 = 0b0011,
    AF4 = 0b0100,
    AF5 = 0b0101,
    AF6 = 0b0110,
    AF7 = 0b0111,
    AF8 = 0b1000,
    AF9 = 0b1001,
    AF10 = 0b1010,
    AF11 = 0b1011,
    AF12 = 0b1100,
    AF13 = 0b1101,
    AF14 = 0b1110,
    AF15 = 0b1111,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Mode {
    Input = 0b00,
    Output = 0b01,
    AlternateFunction = 0b10,
    Analog = 0b11,
}

#[derive(Debug, Clone)]
struct BsrrRef(*mut gpiod::Bsrr);

impl BsrrRef {
    fn set(&self, pin_number: PinNumber, value: bool) {
        let f = match value {
            true => {
                match pin_number {
                    PinNumber::Pin0 => gpiod::BsrrW::bs0,
                    PinNumber::Pin1 => gpiod::BsrrW::bs1,
                    PinNumber::Pin2 => gpiod::BsrrW::bs2,
                    PinNumber::Pin3 => gpiod::BsrrW::bs3,
                    PinNumber::Pin4 => gpiod::BsrrW::bs4,
                    PinNumber::Pin5 => gpiod::BsrrW::bs5,
                    PinNumber::Pin6 => gpiod::BsrrW::bs6,
                    PinNumber::Pin7 => gpiod::BsrrW::bs7,
                    PinNumber::Pin8 => gpiod::BsrrW::bs8,
                    PinNumber::Pin9 => gpiod::BsrrW::bs9,
                    PinNumber::Pin10 => gpiod::BsrrW::bs10,
                    PinNumber::Pin11 => gpiod::BsrrW::bs11,
                    PinNumber::Pin12 => gpiod::BsrrW::bs12,
                    PinNumber::Pin13 => gpiod::BsrrW::bs13,
                    PinNumber::Pin14 => gpiod::BsrrW::bs14,
                    PinNumber::Pin15 => gpiod::BsrrW::bs15,
                }
            }
            false => {
                match pin_number {
                    PinNumber::Pin0 => gpiod::BsrrW::br0,
                    PinNumber::Pin1 => gpiod::BsrrW::br1,
                    PinNumber::Pin2 => gpiod::BsrrW::br2,
                    PinNumber::Pin3 => gpiod::BsrrW::br3,
                    PinNumber::Pin4 => gpiod::BsrrW::br4,
                    PinNumber::Pin5 => gpiod::BsrrW::br5,
                    PinNumber::Pin6 => gpiod::BsrrW::br6,
                    PinNumber::Pin7 => gpiod::BsrrW::br7,
                    PinNumber::Pin8 => gpiod::BsrrW::br8,
                    PinNumber::Pin9 => gpiod::BsrrW::br9,
                    PinNumber::Pin10 => gpiod::BsrrW::br10,
                    PinNumber::Pin11 => gpiod::BsrrW::br11,
                    PinNumber::Pin12 => gpiod::BsrrW::br12,
                    PinNumber::Pin13 => gpiod::BsrrW::br13,
                    PinNumber::Pin14 => gpiod::BsrrW::br14,
                    PinNumber::Pin15 => gpiod::BsrrW::br15,
                }
            }
        };
        let bsrr = unsafe { &mut *self.0 };
        bsrr.write(|r| f(r, true));
    }
}

mod into_gpiod {
    use svd_board::gpioa::Gpioa;
    use svd_board::gpiob::Gpiob;
    use svd_board::gpiod::Gpiod;
    use core::mem;

    pub trait IntoGpiod {
        fn into(self) -> Gpiod;
    }

    impl IntoGpiod for Gpiod {
        fn into(self) -> Gpiod {
            self
        }
    }

    impl IntoGpiod for Gpioa {
        fn into(self) -> Gpiod {
            unsafe { mem::transmute(self) }
        }
    }

    impl IntoGpiod for Gpiob {
        fn into(self) -> Gpiod {
            unsafe { mem::transmute(self) }
        }
    }
}
