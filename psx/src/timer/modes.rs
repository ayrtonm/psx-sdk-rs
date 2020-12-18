#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode0 {
    A = 0,
    B,
    C,
    D,
}

pub enum Mode1 {}
pub enum Mode2 {}

pub trait Mode {
    fn bits(self) -> u32;
}

impl Mode for Mode0 {
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Mode for Mode1 {
    fn bits(self) -> u32 {
        self as u32
    }
}

impl Mode for Mode2 {
    fn bits(self) -> u32 {
        self as u32
    }
}
