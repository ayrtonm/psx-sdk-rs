use crate::global::Global;

static SEED: Global<u32> = Global::new(0);

pub fn srand(seed: u32) -> u32 {
    // This is a random number generator so I don't really care if seed isn't set
    // correctly
    unsafe {
        *SEED.as_ptr() = seed;
    }
    0
}

pub fn rand() -> u32 {
    // This is a random number generator so I don't really care if seed isn't
    // updated correctly
    let seed = SEED.as_ptr();
    unsafe {
        *seed *= 0x41C6_4E6D;
        *seed += 0x3039;
        (*seed >> 16) & 0x7FFF
    }
}
