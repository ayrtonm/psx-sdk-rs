pub enum Source {
    System = 0,
    Alternate,
}

pub enum SyncMode {
    Pause = 0,
    Reset,
    Count,
    FreeRun,
}

pub fn delay(n: usize) {
    for _ in 0..n {
        unsafe {
            core::ptr::read_volatile(0 as *const u32);
        }
    }
}
