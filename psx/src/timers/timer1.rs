#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    PauseVblank = 0,
    ResetVblank,
    TimeVblank,
    FreeRunVblank,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    System = 0,
    Hblank,
}

impl_timer!(1);
