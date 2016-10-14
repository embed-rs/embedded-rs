pub use self::inner::{Hardware, hw};

#[cfg(feature="stm32f7")]
mod inner {
    pub use svd_board::{Hardware, hw};
}
