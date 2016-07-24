use core::fmt;

// FIXME: Check the signature is proper and add a description what
// eh_personality is actually used for.
#[lang = "eh_personality"]
extern fn eh_personality() {}

// DAMN. https://github.com/rust-lang/rust/blob/72ed7e78942e8d68f87cc7299625fb236f442ef1/src/librustc/middle/weak_lang_items.rs#L136
#[lang = "panic_fmt"]
#[unwind]
extern fn panic_impl(_: fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[lang = "start"]
fn start(_: *const u8, _: isize, _: *const *const u8) -> isize {
    -1
}

#[no_mangle]
pub extern fn abort() {}

// FIXME: what does this do?
// Note: They need to be public for the linker to see them when linking against
// users of stm_runtime
#[no_mangle]
pub extern fn __exidx_end() {}

// FIXME: what does this do?
#[no_mangle]
pub extern fn __exidx_start() {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
