Rust on the STM32-Discovery board family
========================================

The `stm32` crates strives to make development on the [STM32-Discovery](www.st.com/stm32-discovery) uncomplicated and fun without sacrificing suitability for serious application development. It consists of a minimal "runtime" needed to compile Rust programs for these boards and an optional small layer of hardware abstraction.


Getting started
---------------

The following are requisites for `stm32` development:

1. A working Rust nightly, [https://www.rustup.rs](rustup.rs) is highly recommeded.
2. A linker that can link for the target platform, like `arm-none-eabi-gcc`.
3. [Xargo](https://github.com/japaric/xargo), which can conveniently be installed through `cargo install xargo`
