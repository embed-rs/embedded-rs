use svd_board::gpiod::{self, Gpiod};
use core::marker::PhantomData;
use volatile::{ReadOnly, WriteOnly, ReadWrite};

pub struct Gpio<N: GpioPortNumber> {
    registers: &'static mut Gpiod,
    phantom: PhantomData<N>,
}

impl<N: GpioPortNumber> Gpio<N> {
    /// Safety: It's unsafe to create two Gpios with the same port number.
    pub unsafe fn new(registers: &'static mut Gpiod) -> Gpio<N> {
        Gpio {
            registers: registers,
            phantom: PhantomData,
        }
    }

    pub fn split(self) -> (GpioPort<N>, GpioPins<N>) {
        let &mut Gpiod { ref mut moder,
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
    mode: &'static mut ReadWrite<gpiod::Moder>,
    out_type: &'static mut ReadWrite<gpiod::Otyper>,
    out_speed: &'static mut ReadWrite<gpiod::Ospeedr>,
    pupd: &'static mut ReadWrite<gpiod::Pupdr>,
    afrl: &'static mut ReadWrite<gpiod::Afrl>,
    afrh: &'static mut ReadWrite<gpiod::Afrh>,
    idr: &'static ReadOnly<gpiod::Idr>,
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
            PinNumber::Pin0 => gpiod::Moder::set_moder0,
            PinNumber::Pin1 => gpiod::Moder::set_moder1,
            PinNumber::Pin2 => gpiod::Moder::set_moder2,
            PinNumber::Pin3 => gpiod::Moder::set_moder3,
            PinNumber::Pin4 => gpiod::Moder::set_moder4,
            PinNumber::Pin5 => gpiod::Moder::set_moder5,
            PinNumber::Pin6 => gpiod::Moder::set_moder6,
            PinNumber::Pin7 => gpiod::Moder::set_moder7,
            PinNumber::Pin8 => gpiod::Moder::set_moder8,
            PinNumber::Pin9 => gpiod::Moder::set_moder9,
            PinNumber::Pin10 => gpiod::Moder::set_moder10,
            PinNumber::Pin11 => gpiod::Moder::set_moder11,
            PinNumber::Pin12 => gpiod::Moder::set_moder12,
            PinNumber::Pin13 => gpiod::Moder::set_moder13,
            PinNumber::Pin14 => gpiod::Moder::set_moder14,
            PinNumber::Pin15 => gpiod::Moder::set_moder15,
        };
        self.mode.update(|r| {
            set_mode(r, mode as u8);
        });
    }

    fn set_resistor(&mut self, pin: &mut GpioPin<N>, resistor: Resistor) {
        let set_pupd = match pin.pin_number {
            PinNumber::Pin0 => gpiod::Pupdr::set_pupdr0,
            PinNumber::Pin1 => gpiod::Pupdr::set_pupdr1,
            PinNumber::Pin2 => gpiod::Pupdr::set_pupdr2,
            PinNumber::Pin3 => gpiod::Pupdr::set_pupdr3,
            PinNumber::Pin4 => gpiod::Pupdr::set_pupdr4,
            PinNumber::Pin5 => gpiod::Pupdr::set_pupdr5,
            PinNumber::Pin6 => gpiod::Pupdr::set_pupdr6,
            PinNumber::Pin7 => gpiod::Pupdr::set_pupdr7,
            PinNumber::Pin8 => gpiod::Pupdr::set_pupdr8,
            PinNumber::Pin9 => gpiod::Pupdr::set_pupdr9,
            PinNumber::Pin10 => gpiod::Pupdr::set_pupdr10,
            PinNumber::Pin11 => gpiod::Pupdr::set_pupdr11,
            PinNumber::Pin12 => gpiod::Pupdr::set_pupdr12,
            PinNumber::Pin13 => gpiod::Pupdr::set_pupdr13,
            PinNumber::Pin14 => gpiod::Pupdr::set_pupdr14,
            PinNumber::Pin15 => gpiod::Pupdr::set_pupdr15,
        };
        self.pupd.update(|r| {
            set_pupd(r, resistor as u8);
        });
    }

    fn set_out_type(&mut self, pin: &mut GpioPin<N>, out_type: Type) {
        let set_type = match pin.pin_number {
            PinNumber::Pin0 => gpiod::Otyper::set_ot0,
            PinNumber::Pin1 => gpiod::Otyper::set_ot1,
            PinNumber::Pin2 => gpiod::Otyper::set_ot2,
            PinNumber::Pin3 => gpiod::Otyper::set_ot3,
            PinNumber::Pin4 => gpiod::Otyper::set_ot4,
            PinNumber::Pin5 => gpiod::Otyper::set_ot5,
            PinNumber::Pin6 => gpiod::Otyper::set_ot6,
            PinNumber::Pin7 => gpiod::Otyper::set_ot7,
            PinNumber::Pin8 => gpiod::Otyper::set_ot8,
            PinNumber::Pin9 => gpiod::Otyper::set_ot9,
            PinNumber::Pin10 => gpiod::Otyper::set_ot10,
            PinNumber::Pin11 => gpiod::Otyper::set_ot11,
            PinNumber::Pin12 => gpiod::Otyper::set_ot12,
            PinNumber::Pin13 => gpiod::Otyper::set_ot13,
            PinNumber::Pin14 => gpiod::Otyper::set_ot14,
            PinNumber::Pin15 => gpiod::Otyper::set_ot15,
        };
        let value = match out_type {
            Type::PushPull => false,
            Type::OpenDrain => true,
        };
        self.out_type.update(|r| {
            set_type(r, value);
        });
    }

    fn set_out_speed(&mut self, pin: &mut GpioPin<N>, out_speed: Speed) {
        let set_speed = match pin.pin_number {
            PinNumber::Pin0 => gpiod::Ospeedr::set_ospeedr0,
            PinNumber::Pin1 => gpiod::Ospeedr::set_ospeedr1,
            PinNumber::Pin2 => gpiod::Ospeedr::set_ospeedr2,
            PinNumber::Pin3 => gpiod::Ospeedr::set_ospeedr3,
            PinNumber::Pin4 => gpiod::Ospeedr::set_ospeedr4,
            PinNumber::Pin5 => gpiod::Ospeedr::set_ospeedr5,
            PinNumber::Pin6 => gpiod::Ospeedr::set_ospeedr6,
            PinNumber::Pin7 => gpiod::Ospeedr::set_ospeedr7,
            PinNumber::Pin8 => gpiod::Ospeedr::set_ospeedr8,
            PinNumber::Pin9 => gpiod::Ospeedr::set_ospeedr9,
            PinNumber::Pin10 => gpiod::Ospeedr::set_ospeedr10,
            PinNumber::Pin11 => gpiod::Ospeedr::set_ospeedr11,
            PinNumber::Pin12 => gpiod::Ospeedr::set_ospeedr12,
            PinNumber::Pin13 => gpiod::Ospeedr::set_ospeedr13,
            PinNumber::Pin14 => gpiod::Ospeedr::set_ospeedr14,
            PinNumber::Pin15 => gpiod::Ospeedr::set_ospeedr15,
        };
        self.out_speed.update(|r| {
            set_speed(r, out_speed as u8);
        });
    }

    fn set_alternate_function(&mut self, pin: &mut GpioPin<N>, alternate_fn: AlternateFunction) {
        match pin.pin_number as u8 {
            0...7 => {
                let set_alternate_fn = match pin.pin_number {
                    PinNumber::Pin0 => gpiod::Afrl::set_afrl0,
                    PinNumber::Pin1 => gpiod::Afrl::set_afrl1,
                    PinNumber::Pin2 => gpiod::Afrl::set_afrl2,
                    PinNumber::Pin3 => gpiod::Afrl::set_afrl3,
                    PinNumber::Pin4 => gpiod::Afrl::set_afrl4,
                    PinNumber::Pin5 => gpiod::Afrl::set_afrl5,
                    PinNumber::Pin6 => gpiod::Afrl::set_afrl6,
                    PinNumber::Pin7 => gpiod::Afrl::set_afrl7,
                    _ => unreachable!(),
                };
                self.afrl.update(|r| {
                    set_alternate_fn(r, alternate_fn as u8);
                });
            }
            8...15 => {
                let set_alternate_fn = match pin.pin_number {
                    PinNumber::Pin8 => gpiod::Afrh::set_afrh8,
                    PinNumber::Pin9 => gpiod::Afrh::set_afrh9,
                    PinNumber::Pin10 => gpiod::Afrh::set_afrh10,
                    PinNumber::Pin11 => gpiod::Afrh::set_afrh11,
                    PinNumber::Pin12 => gpiod::Afrh::set_afrh12,
                    PinNumber::Pin13 => gpiod::Afrh::set_afrh13,
                    PinNumber::Pin14 => gpiod::Afrh::set_afrh14,
                    PinNumber::Pin15 => gpiod::Afrh::set_afrh15,
                    _ => unreachable!(),
                };
                self.afrh.update(|r| {
                    set_alternate_fn(r, alternate_fn as u8);
                });
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
    idr: &'static ReadOnly<gpiod::Idr>,
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
struct BsrrRef(*mut WriteOnly<gpiod::Bsrr>);

impl BsrrRef {
    fn set(&self, pin_number: PinNumber, value: bool) {
        let f = match value {
            true => {
                match pin_number {
                    PinNumber::Pin0 => gpiod::Bsrr::set_bs0,
                    PinNumber::Pin1 => gpiod::Bsrr::set_bs1,
                    PinNumber::Pin2 => gpiod::Bsrr::set_bs2,
                    PinNumber::Pin3 => gpiod::Bsrr::set_bs3,
                    PinNumber::Pin4 => gpiod::Bsrr::set_bs4,
                    PinNumber::Pin5 => gpiod::Bsrr::set_bs5,
                    PinNumber::Pin6 => gpiod::Bsrr::set_bs6,
                    PinNumber::Pin7 => gpiod::Bsrr::set_bs7,
                    PinNumber::Pin8 => gpiod::Bsrr::set_bs8,
                    PinNumber::Pin9 => gpiod::Bsrr::set_bs9,
                    PinNumber::Pin10 => gpiod::Bsrr::set_bs10,
                    PinNumber::Pin11 => gpiod::Bsrr::set_bs11,
                    PinNumber::Pin12 => gpiod::Bsrr::set_bs12,
                    PinNumber::Pin13 => gpiod::Bsrr::set_bs13,
                    PinNumber::Pin14 => gpiod::Bsrr::set_bs14,
                    PinNumber::Pin15 => gpiod::Bsrr::set_bs15,
                }
            }
            false => {
                match pin_number {
                    PinNumber::Pin0 => gpiod::Bsrr::set_br0,
                    PinNumber::Pin1 => gpiod::Bsrr::set_br1,
                    PinNumber::Pin2 => gpiod::Bsrr::set_br2,
                    PinNumber::Pin3 => gpiod::Bsrr::set_br3,
                    PinNumber::Pin4 => gpiod::Bsrr::set_br4,
                    PinNumber::Pin5 => gpiod::Bsrr::set_br5,
                    PinNumber::Pin6 => gpiod::Bsrr::set_br6,
                    PinNumber::Pin7 => gpiod::Bsrr::set_br7,
                    PinNumber::Pin8 => gpiod::Bsrr::set_br8,
                    PinNumber::Pin9 => gpiod::Bsrr::set_br9,
                    PinNumber::Pin10 => gpiod::Bsrr::set_br10,
                    PinNumber::Pin11 => gpiod::Bsrr::set_br11,
                    PinNumber::Pin12 => gpiod::Bsrr::set_br12,
                    PinNumber::Pin13 => gpiod::Bsrr::set_br13,
                    PinNumber::Pin14 => gpiod::Bsrr::set_br14,
                    PinNumber::Pin15 => gpiod::Bsrr::set_br15,
                }
            }
        };
        let mut new_value = gpiod::Bsrr::reset_value();
        f(&mut new_value, true);

        let bsrr = unsafe { &mut *self.0 };
        bsrr.write(new_value);
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
