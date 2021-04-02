use crate::hal::timer::{Current, Mode, Target};
use crate::hal::Mutable;
use crate::hal::{T0_CNT, T0_MODE, T0_TGT};
use crate::hal::{T1_CNT, T1_MODE, T1_TGT};
use crate::hal::{T2_CNT, T2_MODE, T2_TGT};

pub use crate::hal::timer::ty::{Name, Source, SyncMode};

mod counter;

pub struct Counter<CNT: Current, MODE: Mode, TGT: Target, const NAME: Name> {
    cnt: CNT,
    mode: MODE,
    tgt: TGT,
}

pub type DotClock = Counter<T0_CNT<Mutable>, T0_MODE<Mutable>, T0_TGT<Mutable>, { Name::DotClock }>;
pub type Hblank = Counter<T1_CNT<Mutable>, T1_MODE<Mutable>, T1_TGT<Mutable>, { Name::Hblank }>;
pub type Fractional =
    Counter<T2_CNT<Mutable>, T2_MODE<Mutable>, T2_TGT<Mutable>, { Name::Fractional }>;

pub fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
