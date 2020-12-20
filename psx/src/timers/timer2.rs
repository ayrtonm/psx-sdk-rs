#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    Stop = 0,
    Start,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    System = 0,
    SlowSystem = 2,
}

impl_timer!(2);
