#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    PauseHblank = 0,
    ResetHblank,
    TimeHblank,
    FreeRunHblank,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    System = 0,
    DotClock,
}

impl_timer!(0);
