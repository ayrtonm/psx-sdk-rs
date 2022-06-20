//! Random number generator
use crate::gpu::Color;
use crate::sys::kernel;
use core::mem::size_of;
use core::slice;

/// A random number generator
///
/// Use [`Self::rand`] to generate a random integer or float. Note that each
/// call to [`Self::step`] advances the generator state to
/// `x = x * 0x41C6_4E6D + 0x3039` and returns the lower **15 bits** of
/// `x / 0x1_0000`. Also all `Rng` instances share a global state.
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
    /// Steps the rng state and returns a random **15 bit** number.
    pub fn step(&self) -> u16 {
        // SAFETY: The BIOS rng was seeded in `Rng::new`.
        unsafe { kernel::rand() }
    }

    /// Generates a random number with multiple calls to the BIOS.
    pub fn rand<T: From<u8>>(&self) -> T {
        let mut res = T::from(0);
        let ptr = &mut res as *mut T as *mut u8;
        let slice = unsafe { slice::from_raw_parts_mut(ptr, size_of::<T>()) };
        for n in 0..slice.len() {
            slice[n] = self.step() as u8;
        }
        res
    }

    /// Generates a random color with two calls to the BIOS.
    pub fn rand_color(&self) -> Color {
        let x = ((self.step() as u32) << 15) | (self.step() as u32);
        Color::new(x as u8, (x >> 8) as u8, (x >> 16) as u8)
    }
}

/// Checks that a single rng step produces a 15-bit number.
#[test_case]
fn rng_size() {
    fuzz!(|seed: u32| {
        let mut rng = Rng::new(seed);
        let rand = rng.step();
        assert!(rand < (1 << 15));
    });
}

#[test_case]
fn rng_state() {
    fuzz!(|seed: u32, steps: u8| {
        let mut rng = Rng::new(seed);
        let mut state = seed;
        // The fuzzer steps the global rng state an unspecified number of times
        // between cases so we iterate the rng within a single fuzz case
        for _ in 0..steps {
            let x = rng.step() as u32;
            state = state * 0x41C6_4E6D + 0x3039;
            let expected = (state / 0x1_0000) & 0x7F_FF;
            assert!(x == expected);
        }
    });
}
