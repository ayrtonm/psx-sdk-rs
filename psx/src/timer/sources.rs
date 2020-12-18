#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source0 {
    A = 0,
    B,
    C,
    D,
}

pub enum Source1 {}
pub enum Source2 {}

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

