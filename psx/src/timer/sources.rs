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

pub trait Sources {
    fn bits(self) -> u32;
}

impl Sources for Source0 {
    #[inline(always)]
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Sources for Source1 {
    #[inline(always)]
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Sources for Source2 {
    #[inline(always)]
    fn bits(self) -> u32 {
        self as u32
    }
}
