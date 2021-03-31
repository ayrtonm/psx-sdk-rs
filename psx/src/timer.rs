#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Name {
    DotClock = 0,
    Hblank,
    Fractional,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    System = 0,
    Alternate,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
