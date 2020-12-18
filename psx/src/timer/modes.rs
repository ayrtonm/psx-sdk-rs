#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode0 {
    PauseHblank = 0,
    ResetHblank,
    TimeHblank,
    FreeRunHblank,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode1 {
    PauseVblank = 0,
    ResetVblank,
    TimeVblank,
    FreeRunVblank,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode2 {
    Stop = 0,
    Start,
}

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
