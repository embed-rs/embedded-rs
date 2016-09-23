use ::base::volatile::VolatileStruct;
use ::components::*;
use ::InterruptHandler;

#[repr(C)]
pub struct VectorTable {
    pub msp: &'static (),
    pub reset: Option<InterruptHandler>,
    pub nmi: Option<InterruptHandler>,
    pub hard_fault: Option<InterruptHandler>,
}

#[allow(unused)]
pub const INITIAL_CPU_FREQ: usize = 0;

pub struct Hardware {
    foo: usize,
}

pub const VECTOR_TABLE: VectorTable = VectorTable {
    msp: &(),
    reset: None,
    nmi: None,
    hard_fault: None,
};


pub unsafe fn hw() -> Hardware {
    Hardware { foo: 0 }
}
