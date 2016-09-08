#[inline(always)]
pub fn keep() {
    // FIXME: gate on arm architecture
    unsafe {
        asm!("");
    }
}

#[inline(always)]
pub fn nop() {
    // FIXME: gate using arm architecture
    unsafe {
        asm!("NOP" : : : : "volatile");
    }
}

/// Delay for roughly n instructions
///
/// Note: This function usually compiles down to a 2-instruction loop + some
/// minor overhead. `n` is therefore halved.
///
/// Compiling without --release will cause this function to take between 10 to
/// 30 times as long, making it quite unuseable.
pub fn delay(n: usize) {
    for _ in 0..(n/2) {
        // example loop disassembly:
        //   8010080:    3801          subs    r0, #1
        //   8010082:    d1fd          bne.n    8010080
        keep();
    }
}
