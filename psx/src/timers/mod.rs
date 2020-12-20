macro_rules! impl_timer {
    ($offset:expr) => {
        paste::paste! {
            use crate::mmio::register::{Read, Write};
            use crate::mmio::[<timer $offset>] as timer;

            impl timer::Current {
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

            impl timer::Target {
                #[inline(always)]
                pub fn get(&self) -> u16 {
                    unsafe {
                        self.read() as u16;
                        self.read() as u16
                    }
                }

                pub fn set(&mut self, value: u16) {
                    unsafe {
                        self.write(value.into())
                    }
                }
            }

            pub mod mode {
                use super::SyncMode;
                use super::Source;
                use super::timer;

                impl_mut_value!(timer::Mode);

                impl Value {
                    #[inline(always)]
                    pub fn sync_enabled(&self) -> bool {
                        self.bits & 1 != 0
                    }
                }

                impl<'a> MutValue<'a> {
                    #[inline(always)]
                    pub fn sync(mut self, sync: bool) -> Self {
                        if sync {
                            self.value.bits |= 1;
                        } else {
                            self.value.bits &= !1;
                        }
                        self
                    }

                    #[inline(always)]
                    pub fn sync_mode(mut self, sync_mode: SyncMode) -> Self {
                        self.value.bits &= !(0b11 << 1);
                        self.value.bits |= sync_mode as u32;
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
                    pub fn source(mut self, source: Source) -> Self {
                        self.value.bits &= !(0b11 << 8);
                        self.value.bits |= source as u32;
                        self
                    }
                }
            }
        }
    };
}

pub mod timer0;
pub mod timer1;
pub mod timer2;
