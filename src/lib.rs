//! STM32-Discovery boards minimal runtime.
//!
//! See the `README.md` for a detailed introduction.

#![feature(asm)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![no_std]

extern crate rustc_builtins;
#[macro_use]
extern crate bitflags;
extern crate bit_field;

pub mod base;
pub mod boards;
pub mod components;
pub mod irq;
pub mod util;


// runtime; should be moved to embedded-rs?
// apparently, even with panic="abort" you need to have a panic_fmt
// implementation
use core::fmt;
#[lang = "panic_fmt"]
#[unwind]
extern "C" fn panic_impl(_: fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

// end runtime

pub type InterruptHandler = extern "C" fn() -> ();

#[allow(improper_ctypes)]
extern "C" {
    static _STACK_TOP: ();
}

#[macro_export]
macro_rules! board {
    ($board:ident,
     {
         $( $fname:ident : $fval:expr),*
     }
    )  =>
    (
        #[allow(improper_ctypes)]
        extern {
            static _STACK_TOP: ();
        }

        extern "C" fn _rust_start() {
            ::main()
        }

        #[link_section="vectors"]
        #[no_mangle]
        pub static VECTORS: $crate::boards::$board::VectorTable =
        $crate::boards::$board::VectorTable {
            msp: unsafe { &_STACK_TOP },
            reset: Some(_rust_start),
            $( $fname: $fval, )*
            ..$crate::boards::$board::VECTOR_TABLE
        };

        use $crate::boards::$board::INITIAL_CPU_FREQ;
    )
}
