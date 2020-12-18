use crate::mmio::register::{Read, Update, Write};

pub(crate) mod modes;
pub(crate) mod sources;

use modes::Modes;
use sources::Sources;

pub use modes::{Mode0, Mode1, Mode2};
pub use sources::{Source0, Source1, Source2};

macro_rules! impl_timer {
    ($offset:expr) => {
        paste::paste! {
            impl crate::mmio::[<timer $offset>]::Current {
                pub fn get_current(&self) -> u16 {
                    unsafe {
                        self.read() as u16
                    }
                }

                pub fn set_current(&mut self, value: u16) {
                    unsafe {
                        self.write(value.into())
                    }
                }
            }

            impl crate::mmio::[<timer $offset>]::Mode {
                pub fn enable_sync(&mut self, sync: bool) -> &mut Self {
                    unsafe {
                        self.update(|val| if sync { val | 1 } else { val & !1 });
                    }
                    self
                }

                pub fn sync_mode(&mut self, mode: [<Mode $offset>]) -> &mut Self {
                    unsafe {
                        self.update(|mut val| {
                            val &= !(0b11 << 1);
                            val | mode.bits()
                        });
                    }
                    self
                }

                pub fn source(&mut self, source: [<Source $offset>]) -> &mut Self {
                    unsafe {
                        self.update(|mut val| {
                            val &= !(0b11 << 8);
                            val | source.bits()
                        });
                    }
                    self
                }
            }

            impl crate::mmio::[<timer $offset>]::Target {
                pub fn get_target(&self) -> u16 {
                    unsafe {
                        self.read() as u16
                    }
                }

                pub fn set_target(&mut self, value: u16) {
                    unsafe {
                        self.write(value.into())
                    }
                }
            }
        }
    };
}

impl_timer!(0);
impl_timer!(1);
impl_timer!(2);
