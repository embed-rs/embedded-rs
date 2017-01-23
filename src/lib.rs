//! STM32-Discovery boards minimal runtime.
//!
//! See the `README.md` for a detailed introduction.

#![feature(asm)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins_snapshot;
#[macro_use]
extern crate bitflags;
extern crate bit_field;
extern crate volatile;
extern crate arrayvec;

pub mod components;
pub mod interfaces;
pub mod irq;
pub mod util;
pub mod runtime;

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
        use $crate::boards::$board::Hardware;

        #[allow(improper_ctypes)]
        extern {
            static _STACK_TOP: ();
        }

        extern "C" fn _rust_start() {
            ::main(unsafe { $crate::boards::$board::hw() })
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
