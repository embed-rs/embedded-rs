use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};

/// Enable IRQs (disables PRIMASK)
///
/// Implemented using a single instruction. Does not affect NMI and
/// HardFault.
#[cfg(target_arch = "arm")]
unsafe fn enable_irq() {
    asm!("CPSIE i" : : : : "volatile");
}

/// Disables IRQs (enables PRIMASK)
///
/// Implemented using a single instruction.
#[cfg(target_arch = "arm")]
unsafe fn disable_irq() {
    asm!("CPSID i" : : : : "volatile");
}

/// Enable IRQs (disables FAULTMASK)
///
/// Implemented using a single instruction. Does not affect NMI.
#[cfg(target_arch = "arm")]
unsafe fn enable_fault_irq() {
    asm!("CPSIE f" : : : : "volatile");
}

/// Disables IRQs (enables FAULTMASK)
///
/// Implemented using a single instruction.
#[cfg(target_arch = "arm")]
unsafe fn disable_fault_irq() {
    asm!("CPSID f" : : : : "volatile");
}

pub trait MaskRegister {
    fn get_mask(&self) -> bool;
    fn set_mask(&mut self, enabled: bool);
}

pub struct MaskMutex<R: MaskRegister, T> {
    // FIXME: Rust's MaskMutex uses Box - but we do not always have a heap.
    //        For now, take ownership of the object, which might be a pointer
    //        anyway.
    data: UnsafeCell<T>,
    reg: UnsafeCell<R>,
}

pub struct MaskMutexGuard<'a, R: MaskRegister + 'a, T: 'a> {
    _data: &'a mut T,
    reg: &'a mut R,
    prev: bool,
}

impl<'a, R: MaskRegister, T> MaskMutex<R, T> {
    pub fn new(p: R, t: T) -> MaskMutex<R, T> {
        MaskMutex{
            data: UnsafeCell::new(t),
            reg: UnsafeCell::new(p),
        }
    }

    pub fn lock(&'a self) -> MaskMutexGuard<'a, R, T> {
        let reg = unsafe { &mut *self.reg.get() };

        // Note: Locks can never be poisoned, as we don't have "real"
        // multithreading, so we always return a guard
        let prev = reg.get_mask();
        reg.set_mask(true);

        MaskMutexGuard {
            // UnsafeCell better here?
            _data: unsafe { &mut *self.data.get() },
            reg: reg,
            prev: prev,
        }
    }

    pub fn into_inner(self) -> T {
        unsafe { self.data.into_inner() }
    }
}

impl<'a, R: MaskRegister + 'a, T: 'a> Drop for MaskMutexGuard<'a, R, T> {
    fn drop(&mut self) {
        self.reg.set_mask(self.prev)
    }
}

impl<'a, R: MaskRegister + 'a, T: 'a> Deref for MaskMutexGuard<'a, R, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self._data
    }
}

impl<'a, R: MaskRegister + 'a, T: 'a> DerefMut for MaskMutexGuard<'a, R, T> {
    fn deref_mut(&mut self) -> &mut T {
        self._data
    }
}

// NOTE: Could add a less-complicated, non-reentrant version of the lock here
//       if we wanted.
