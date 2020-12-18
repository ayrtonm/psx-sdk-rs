#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source0 {
    System = 0,
    DotClock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source1 {
    System = 0,
    Hblank,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source2 {
    System = 0,
    SlowSystem = 2,
}

pub trait Source {
    fn bits(self) -> u32;
}

impl Source for Source0 {
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Source for Source1 {
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Source for Source2 {
    fn bits(self) -> u32 {
        self as u32
    }
}
