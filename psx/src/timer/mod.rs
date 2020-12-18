use crate::mmio::register::{Read, Update, Write};

pub(crate) mod modes;
pub(crate) mod sources;

use modes::Mode;
use sources::Source;

pub use modes::{Mode0, Mode1, Mode2};
pub use sources::{Source0, Source1, Source2};

pub trait Timer<M: Mode, S: Source>: Update<u32> {
    fn enable_sync(&mut self, sync: bool) -> &mut Self {
        unsafe {
            self.update(|val| if sync { val | 1 } else { val & !1 });
        }
        self
    }
    fn sync_mode(&mut self, mode: M) -> &mut Self {
        unsafe {
            self.update(|mut val| {
                val &= !(0b11 << 1);
                val | mode.bits()
            });
        }
        self
    }
    fn source(&mut self, source: S) -> &mut Self {
        unsafe {
            self.update(|mut val| {
                val &= !(0b11 << 8);
                val | source.bits()
            });
        }
        self
    }
}

macro_rules! impl_timer {
    ($offset:expr) => {
        paste::paste! {
            impl crate::mmio::[<timer $offset>]::Timer {
                pub fn get_current(&self) -> u16 {
                    unsafe {
                        self.current.read() as u16
                    }
                }
                pub fn set_current(&mut self, value: u16) {
                    unsafe {
                        self.current.write(value.into())
                    }
                }
                pub fn get_target(&self) -> u16 {
                    unsafe {
                        self.target.read() as u16
                    }
                }
                pub fn set_target(&mut self, value: u16) {
                    unsafe {
                        self.target.write(value.into())
                    }
                }
            }
        }
    };
}

impl_timer!(0);
impl_timer!(1);
impl_timer!(2);
