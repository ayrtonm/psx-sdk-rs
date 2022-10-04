static mut SEED: u32 = 0;

pub fn srand(seed: u32) {
    unsafe {
        SEED = seed;
    }
}

pub fn rand() -> u32 {
    unsafe {
        SEED = SEED * 0x41C6_4E6D + 0x3039;
        (SEED >> 16) & 0x7FFF
    }
}
