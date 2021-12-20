//! BIOS function wrappers.
//!
//! This modules contains wrappers for functions provided by the BIOS.

use crate::KSEG0;
use core::mem::size_of;

pub mod fs;
pub mod gamepad;
pub mod heap;
pub mod kernel;
pub mod tty;

fn table_of_tables() -> *const u32 {
    let addr = KSEG0 + (0x100 / size_of::<u32>());
    addr as *const u32
}

/// Runs the given function in an interrupt-free critical section using BIOS
/// syscalls.
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    unsafe {
        kernel::enter_critical_section();
        let res = f();
        kernel::exit_critical_section();
        res
    }
}

/// A random number generator
///
/// Each call to [`Self::rand`] advances the generator state to `x = x *
/// 0x41C6_4E6D + 0x3039` and returns the lower **15 bits** of `x /
/// 0x1_0000`. Note that all `Rng` instances share a global state.
pub struct Rng(());

impl Rng {
    /// Creates a new random number generator initialized with `seed`.
    pub fn new(seed: u32) -> Self {
        // SAFETY: srand has no safety requirements.
        unsafe { kernel::srand(seed) }
        Rng(())
    }
    /// Reseeds the random number generator.
    pub fn reseed(&mut self, seed: u32) {
        // SAFETY: srand has no safety requirements.
        unsafe { kernel::srand(seed) }
    }
    /// Returns a random **15 bit** number.
    pub fn rand(&mut self) -> u16 {
        // SAFETY: The BIOS rng was seeded in `Rng::new`.
        unsafe { kernel::rand() }
    }
}

#[test_case]
fn rng() {
    fuzz!(|seed: u32, num: u8| {
        let mut rng = Rng::new(seed);
        let mut state = seed;
        for _ in 0..num {
            let x = rng.rand() as u32;
            state = state * 0x41C6_4E6D + 0x3039;
            assert!(x == (state / 0x1_0000) & 0x7F_FF);
        }
    });
}
