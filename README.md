Rust embedded development
=========================

The `embed` crates strives to make development on the embedded board safe and fun. It provides the minimal required runtime and abstractions for common hardware concepts as well as a library for some boards.


Getting started
---------------

The following are requisites for embedded development:

1. A working Rust nightly, [https://www.rustup.rs](rustup.rs) is highly recommeded.
2. A linker that can link for the target platform, like `arm-none-eabi-gcc`.
3. [Xargo](https://github.com/japaric/xargo), which can conveniently be installed through `cargo install xargo`
