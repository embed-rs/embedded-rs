//! RCC AHB1 peripheral reset register (RCC_AHB1RSTR)

bitflags! {
    pub flags Register: u32 {
        const GPIO_A_RESET = 1 << 0,
        const GPIO_B_RESET = 1 << 1,
        const GPIO_C_RESET = 1 << 2,
        const GPIO_D_RESET = 1 << 3,
        const GPIO_E_RESET = 1 << 4,
        const GPIO_F_RESET = 1 << 5,
        const GPIO_G_RESET = 1 << 6,
        const GPIO_H_RESET = 1 << 7,
        const GPIO_I_RESET = 1 << 8,
        const GPIO_J_RESET = 1 << 9,
        const GPIO_K_RESET = 1 << 10,

        // etc
    }
}
