static mut SEED: u32 = 0;

pub fn srand(seed: u32) -> u32 {
    // This is a random number generator so I don't really care if seed isn't set
    // correctly
    unsafe {
        SEED = seed;
    }
    0
}

pub fn rand() -> u32 {
    // This is a random number generator so I don't really care if seed isn't
    // updated correctly
    let seed = unsafe { &mut SEED };
    *seed *= 0x41C6_4E6D;
    *seed += 0x3039;
    (*seed >> 16) & 0x7FFF
}
