use core::{ops, ptr};

// consider:
// http://stackoverflow.com/questions/35009015/how-do-i-write-to-a-memory-mapped-address-in-rust
//           also, implement operators for Volatile?

// mem::swap: requires immovable trait or volatile stuff ("future work")

// many thanks to talchas from rust irc

#[repr(C)]
pub struct Volatile<T>(T);

impl<T> Volatile<T> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0 as *const T) }
    }

    #[inline(always)]
    pub fn write(&mut self, src: T) {
        unsafe { ptr::write_volatile(&mut self.0, src) }
    }

    /// Updates the register value
    #[inline(always)]
    pub fn update<F>(&mut self, f: F)
        where F: FnOnce(&mut T)
    {
        let mut value = self.read();
        f(&mut value);
        self.write(value);
    }
}

impl<T> ops::BitOrAssign<T> for Volatile<T>
    where T: ops::BitOr<T, Output = T>
{
    #[inline(always)]
    fn bitor_assign(&mut self, val: T) {
        let tmp = self.read();
        let new_val = tmp | val;
        self.write(new_val);
    }
}

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
