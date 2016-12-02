pub use volatile::Volatile;

pub trait VolatileStruct: Sized {
    #[inline(always)]
    unsafe fn from_ptr(addr: *mut Self) -> &'static mut Self {
        let item: &'static mut Self = &mut *addr;
        item
    }

    #[inline(always)]
    unsafe fn from_addr(addr: usize) -> &'static mut Self {
        // we need the Sized trait here, otherwise the cast doesn't work
        Self::from_ptr(addr as *mut Self)
    }
}
