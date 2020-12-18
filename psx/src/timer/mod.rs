use crate::mmio::register::Update;

pub(crate) mod modes;
pub(crate) mod sources;

use modes::Mode;
use sources::Source;

pub use modes::{Mode0, Mode1, Mode2};
pub use sources::{Source0, Source1, Source2};

pub trait Timer<M: Mode, S: Source>: Update<u32> {
    fn sync(&mut self, sync: bool) -> &mut Self {
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
