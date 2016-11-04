// provisional runtime
// apparently, even with panic="abort" you need to have a panic_fmt
// implementation
#[cfg(feature = "panic-fmt")]
use core::fmt;

extern "C" {
    fn main(hw: ::Hardware) -> !;
}

#[no_mangle]
pub extern "C" fn start() {
    unsafe { main(::hw()) }
}

#[cfg(feature = "panic-fmt")]
#[lang = "panic_fmt"]
extern "C" fn panic_impl(_: fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[cfg(feature = "unwind-cpp")]
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

// end runtime
