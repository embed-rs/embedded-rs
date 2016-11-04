pub use self::inner::{Hardware, hw};

#[cfg(feature="stm32f7")]
mod inner {
    use svd_board;
    use components::gpio::{self, Gpio};

    pub struct Hardware {
        pub rcc: &'static mut svd_board::rcc::Rcc,
        pub gpio_a: Gpio<gpio::PortA>,
        pub gpio_b: Gpio<gpio::PortB>,
        pub gpio_c: Gpio<gpio::PortC>,
        pub gpio_d: Gpio<gpio::PortD>,
        pub gpio_e: Gpio<gpio::PortE>,
        pub gpio_f: Gpio<gpio::PortF>,
        pub gpio_g: Gpio<gpio::PortG>,
        pub gpio_h: Gpio<gpio::PortH>,
        pub gpio_i: Gpio<gpio::PortI>,
        pub gpio_j: Gpio<gpio::PortJ>,
        pub gpio_k: Gpio<gpio::PortK>,
    }

    pub fn hw() -> Hardware {
        let svd_board::Hardware { rcc,
                                  gpioa,
                                  gpiob,
                                  gpioc,
                                  gpiod,
                                  gpioe,
                                  gpiof,
                                  gpiog,
                                  gpioh,
                                  gpioi,
                                  gpioj,
                                  gpiok,
                                  .. } = svd_board::hw();

        Hardware {
            rcc: rcc,
            gpio_a: unsafe { Gpio::new(&mut *(gpioa as *mut _ as *mut _)) },
            gpio_b: unsafe { Gpio::new(&mut *(gpiob as *mut _ as *mut _)) },
            gpio_c: unsafe { Gpio::new(gpioc) },
            gpio_d: unsafe { Gpio::new(gpiod) },
            gpio_e: unsafe { Gpio::new(gpioe) },
            gpio_f: unsafe { Gpio::new(gpiof) },
            gpio_g: unsafe { Gpio::new(gpiog) },
            gpio_h: unsafe { Gpio::new(gpioh) },
            gpio_i: unsafe { Gpio::new(gpioi) },
            gpio_j: unsafe { Gpio::new(gpioj) },
            gpio_k: unsafe { Gpio::new(gpiok) },
        }
    }
}
