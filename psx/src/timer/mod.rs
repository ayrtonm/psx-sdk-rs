use crate::mmio::register::{Read, Write};

pub(crate) mod modes;
pub(crate) mod sources;

pub use modes::{Mode0, Mode1, Mode2};
pub use sources::{Source0, Source1, Source2};

macro_rules! impl_timer {
    ($offset:expr) => {
        paste::paste! {
            impl crate::mmio::[<timer $offset>]::Current {
                pub fn get(&self) -> u16 {
                    unsafe {
                        self.read() as u16
                    }
                }

                pub fn set(&mut self, value: u16) {
                    unsafe {
                        self.write(value.into())
                    }
                }
            }

            impl crate::mmio::[<timer $offset>]::Target {
                pub fn get(&self) -> u16 {
                    unsafe {
                        self.read() as u16
                    }
                }

                pub fn set(&mut self, value: u16) {
                    unsafe {
                        self.write(value.into())
                    }
                }
            }

            pub mod [<timer $offset>] {
                use super::modes::Modes;
                use super::sources::Sources;
                use crate::mmio::register::{Read, Write};

                impl_mut_value!(crate::mmio::[<timer $offset>]::Mode);

                impl Value {
                    #[inline(always)]
                    pub fn sync_enabled(&self) -> bool {
                        self.bits & 1 != 0
                    }
                }

                impl<'a> MutValue<'a> {
                    #[inline(always)]
                    pub fn enable_sync(mut self, sync: bool) -> Self {
                        if sync {
                            self.value.bits |= 1;
                        } else {
                            self.value.bits &= !1;
                        }
                        self
                    }

                    #[inline(always)]
                    pub fn sync_mode(mut self, mode: super::[<Mode $offset>]) -> Self {
                        self.value.bits &= !(0b11 << 1);
                        self.value.bits |= mode.bits();
                        self
                    }

                    #[inline(always)]
                    pub fn target_reset(mut self, enable: bool) -> Self {
                        if enable {
                            self.value.bits |= 1 << 3;
                        } else {
                            self.value.bits &= !(1 << 3);
                        }
                        self
                    }

                    #[inline(always)]
                    pub fn source(mut self, source: super::[<Source $offset>]) -> Self {
                        self.value.bits &= !(0b11 << 8);
                        self.value.bits |= source.bits();
                        self
                    }
                }
            }
        }
    };
}

impl_timer!(0);
impl_timer!(1);
impl_timer!(2);
