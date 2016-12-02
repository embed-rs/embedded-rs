use components::gpio::stm32f7;
use volatile::{ReadOnly, WriteOnly, ReadWrite};
use arrayvec::ArrayVec;

pub use components::gpio::stm32f7::{Pin, OutputType, OutputSpeed, Resistor, AlternateFunction};

#[derive(Debug)]
pub enum Error {
    PinAlreadyInUse(Pin),
}

pub struct Gpio {
    port_a: GpioPort,
    port_b: GpioPort,
    port_c: GpioPort,
    port_d: GpioPort,
    port_e: GpioPort,
    port_f: GpioPort,
    port_g: GpioPort,
    port_h: GpioPort,
    port_i: GpioPort,
    port_j: GpioPort,
    port_k: GpioPort,
}

impl Gpio {
    pub fn new(gpio_a: &'static mut stm32f7::Gpio,
               gpio_b: &'static mut stm32f7::Gpio,
               gpio_c: &'static mut stm32f7::Gpio,
               gpio_d: &'static mut stm32f7::Gpio,
               gpio_e: &'static mut stm32f7::Gpio,
               gpio_f: &'static mut stm32f7::Gpio,
               gpio_g: &'static mut stm32f7::Gpio,
               gpio_h: &'static mut stm32f7::Gpio,
               gpio_i: &'static mut stm32f7::Gpio,
               gpio_j: &'static mut stm32f7::Gpio,
               gpio_k: &'static mut stm32f7::Gpio)
               -> Gpio {
        Gpio {
            port_a: GpioPort::new(gpio_a),
            port_b: GpioPort::new(gpio_b),
            port_c: GpioPort::new(gpio_c),
            port_d: GpioPort::new(gpio_d),
            port_e: GpioPort::new(gpio_e),
            port_f: GpioPort::new(gpio_f),
            port_g: GpioPort::new(gpio_g),
            port_h: GpioPort::new(gpio_h),
            port_i: GpioPort::new(gpio_i),
            port_j: GpioPort::new(gpio_j),
            port_k: GpioPort::new(gpio_k),
        }
    }

    pub fn to_input(&mut self, pin: (Port, Pin), resistor: Resistor) -> Result<InputPin, Error> {
        self.port(pin.0).to_input(pin.1, resistor)
    }

    pub fn to_output(&mut self,
                     pin: (Port, Pin),
                     out_type: OutputType,
                     out_speed: OutputSpeed,
                     resistor: Resistor)
                     -> Result<OutputPin, Error> {
        self.port(pin.0).to_output(pin.1, out_type, out_speed, resistor)
    }

    pub fn to_alternate_function(&mut self,
                                 pin: (Port, Pin),
                                 alternate_fn: AlternateFunction,
                                 typ: OutputType,
                                 speed: OutputSpeed,
                                 resistor: Resistor)
                                 -> Result<(), Error> {
        self.port(pin.0).to_alternate_function(pin.1, alternate_fn, typ, speed, resistor)
    }

    pub fn to_alternate_function_all(&mut self,
                                     pins: &[(Port, Pin)],
                                     alternate_fn: AlternateFunction,
                                     typ: OutputType,
                                     speed: OutputSpeed,
                                     resistor: Resistor)
                                     -> Result<(), Error> {

        // check that all pins are unused
        let mut pin_in_use = [self.port_a.pin_in_use,
                              self.port_b.pin_in_use,
                              self.port_c.pin_in_use,
                              self.port_d.pin_in_use,
                              self.port_e.pin_in_use,
                              self.port_f.pin_in_use,
                              self.port_g.pin_in_use,
                              self.port_h.pin_in_use,
                              self.port_i.pin_in_use,
                              self.port_j.pin_in_use,
                              self.port_k.pin_in_use];
        for &(port, pin) in pins {
            if pin_in_use[port as usize][pin as usize] {
                return Err(Error::PinAlreadyInUse(pin));
            } else {
                pin_in_use[port as usize][pin as usize] = true;
            }
        }

        // configure the pins for each port
        use self::Port::*;
        let ports = [PortA, PortB, PortC, PortD, PortE, PortF, PortG, PortH, PortI, PortJ, PortK];
        for &port in ports.iter() {
            // create a pin_vec that contains all pins belonging to the port
            let mut pin_vec = ArrayVec::<[_; 16]>::new();
            for pin in pins.iter().filter(|p| p.0 == port).map(|p| p.1) {
                // the array can't be too small since we check for duplicate pins
                assert!(pin_vec.push(pin).is_none());
            }

            // configure the pins as alternate function pins
            self.port(port)
                .to_alternate_function_all(pin_vec.as_slice(), alternate_fn, typ, speed, resistor)?;
        }
        Ok(())
    }

    pub fn port(&mut self, port: Port) -> &mut GpioPort {
        use self::Port::*;
        match port {
            PortA => &mut self.port_a,
            PortB => &mut self.port_b,
            PortC => &mut self.port_c,
            PortD => &mut self.port_d,
            PortE => &mut self.port_e,
            PortF => &mut self.port_f,
            PortG => &mut self.port_g,
            PortH => &mut self.port_h,
            PortI => &mut self.port_i,
            PortJ => &mut self.port_j,
            PortK => &mut self.port_k,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Port {
    PortA,
    PortB,
    PortC,
    PortD,
    PortE,
    PortF,
    PortG,
    PortH,
    PortI,
    PortJ,
    PortK,
}

pub struct GpioPort {
    pin_in_use: [bool; 16],
    mode: &'static mut ReadWrite<stm32f7::ModeRegister>,
    out_type: &'static mut ReadWrite<stm32f7::OutputTypeRegister>,
    out_speed: &'static mut ReadWrite<stm32f7::OutputSpeedRegister>,
    pupd: &'static mut ReadWrite<stm32f7::ResistorRegister>,
    input_data: &'static ReadOnly<stm32f7::InputDataRegister>,
    output_data: &'static ReadOnly<stm32f7::OutputDataRegister>,
    bit_set_reset: BsrrRef,
    alternate_fn: &'static mut ReadWrite<stm32f7::AlternateFunctionRegister>,
}

impl GpioPort {
    pub fn new(gpio: &'static mut stm32f7::Gpio) -> GpioPort {
        GpioPort {
            pin_in_use: [false; 16],
            mode: &mut gpio.mode,
            out_type: &mut gpio.out_type,
            out_speed: &mut gpio.out_speed,
            pupd: &mut gpio.pupd,
            input_data: &gpio.input_data,
            output_data: &gpio.output_data,
            bit_set_reset: BsrrRef(&mut gpio.bit_set_reset),
            alternate_fn: &mut gpio.alternate_fn,
        }
    }

    pub fn to_input(&mut self, pin: Pin, resistor: Resistor) -> Result<InputPin, Error> {
        self.use_pin(pin)?;

        self.mode.update(|r| r.set(pin, stm32f7::Mode::Input));
        self.pupd.update(|r| r.set(pin, resistor));

        Ok(InputPin {
            pin: pin,
            input_data: self.input_data,
        })
    }

    pub fn to_output(&mut self,
                     pin: Pin,
                     out_type: OutputType,
                     out_speed: OutputSpeed,
                     resistor: Resistor)
                     -> Result<OutputPin, Error> {
        self.use_pin(pin)?;

        self.mode.update(|r| r.set(pin, stm32f7::Mode::Output));
        self.out_type.update(|r| r.set(pin, out_type));
        self.out_speed.update(|r| r.set(pin, out_speed));
        self.pupd.update(|r| r.set(pin, resistor));

        Ok(OutputPin {
            pin: pin,
            output_data: self.output_data,
            bit_set_reset: self.bit_set_reset.clone(),
        })
    }

    pub fn to_alternate_function(&mut self,
                                 pin: Pin,
                                 alternate_fn: AlternateFunction,
                                 typ: OutputType,
                                 speed: OutputSpeed,
                                 resistor: Resistor)
                                 -> Result<(), Error> {
        self.to_alternate_function_all(&[pin], alternate_fn, typ, speed, resistor)
    }

    pub fn to_alternate_function_all(&mut self,
                                     pins: &[Pin],
                                     alternate_fn: AlternateFunction,
                                     typ: OutputType,
                                     speed: OutputSpeed,
                                     resistor: Resistor)
                                     -> Result<(), Error> {
        self.use_pins(pins)?;

        self.mode.update(|r| for &pin in pins {
            r.set(pin, stm32f7::Mode::AlternateFunction)
        });
        self.pupd.update(|r| for &pin in pins {
            r.set(pin, resistor)
        });
        self.out_type.update(|r| for &pin in pins {
            r.set(pin, typ)
        });
        self.out_speed.update(|r| for &pin in pins {
            r.set(pin, speed)
        });
        self.alternate_fn.update(|r| for &pin in pins {
            r.set(pin, alternate_fn)
        });

        Ok(())
    }

    fn use_pin(&mut self, pin: Pin) -> Result<(), Error> {
        if self.pin_in_use[pin as usize] {
            Err(Error::PinAlreadyInUse(pin))
        } else {
            self.pin_in_use[pin as usize] = true;
            Ok(())
        }
    }

    fn use_pins(&mut self, pins: &[Pin]) -> Result<(), Error> {
        // create a copy of the pin_in_use array since we only want to modify it in case of success
        let mut pin_in_use = self.pin_in_use;

        for &pin in pins {
            if pin_in_use[pin as usize] {
                return Err(Error::PinAlreadyInUse(pin));
            } else {
                pin_in_use[pin as usize] = true;
            }
        }

        // success => write back updated pin_in_use array
        self.pin_in_use = pin_in_use;

        Ok(())
    }
}

pub struct InputPin {
    pin: Pin,
    input_data: &'static ReadOnly<stm32f7::InputDataRegister>,
}

impl InputPin {
    pub fn get(&self) -> bool {
        self.input_data.read().get(self.pin)
    }
}

pub struct OutputPin {
    pin: Pin,
    output_data: &'static ReadOnly<stm32f7::OutputDataRegister>,
    bit_set_reset: BsrrRef,
}

impl OutputPin {
    pub fn get(&self) -> bool {
        self.output_data.read().get(self.pin)
    }

    pub fn set(&mut self, value: bool) {
        self.bit_set_reset.set(self.pin, value);
    }
}

#[derive(Debug, Clone)]
struct BsrrRef(*mut WriteOnly<stm32f7::BitSetResetRegister>);

impl BsrrRef {
    fn set(&self, pin: Pin, value: bool) {
        let mut bsrr = stm32f7::BitSetResetRegister::default();
        if value {
            bsrr.set(pin);
        } else {
            bsrr.reset(pin);
        }
        unsafe { (&mut *self.0).write(bsrr) };
    }
}
