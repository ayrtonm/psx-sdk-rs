pub use crate::hal::timer::ty::{Name, Source, SyncMode};

pub fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
