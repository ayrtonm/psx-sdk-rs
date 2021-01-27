pub enum Source {
    System = 0,
    Alternate,
}

#[allow(non_upper_case_globals)]
pub const DotClock: Source = Source::Alternate;
#[allow(non_upper_case_globals)]
pub const Hblank: Source = Source::Alternate;
#[allow(non_upper_case_globals)]
pub const FracSys: Source = Source::Alternate;

pub enum SyncMode {
    Pause = 0,
    Reset,
    Count,
    FreeRun,
}
