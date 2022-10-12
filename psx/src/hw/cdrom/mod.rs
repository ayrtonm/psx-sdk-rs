//! CDROM controller registers

// This is a WIP so I don't care about docs rn
#![allow(missing_docs)]

use crate::hw::MemRegister;

mod controller;
mod status;

#[repr(u8)]
pub enum Port {
    Port0 = 0,
    Port1,
    Port2,
    Port3,
}

#[repr(u8)]
pub enum Command {
    GetStat = 0x1,
    Setloc = 0x2,
    Play = 0x3,
    Forward = 0x04,
    Backward = 0x05,
    ReadN = 0x06,
    MotorOn = 0x07,
    Stop = 0x08,
    Pause = 0x09,
    Init = 0x0A,
    Mute = 0x0B,
    Demute = 0x0C,
    Setfilter = 0x0D,
    Setmode = 0x0E,
    Getparam = 0x0F,
    GetlocL = 0x10,
    GetlocP = 0x11,
    SetSession = 0x12,
    GetTN = 0x13,
    GetTD = 0x14,
    SeekL = 0x15,
    SeekP = 0x16,
    Test = 0x19,
    GetID = 0x1A,
    ReadS = 0x1B,
    Reset = 0x1C,
    GetQ = 0x1D,
    ReadTOC = 0x1E,
    VideoCD = 0x1F,
    Secret1 = 0x50,
    Secret2 = 0x51,
    Secret3 = 0x52,
    Secret4 = 0x53,
    Secret5 = 0x54,
    Secret6 = 0x55,
    Secret7 = 0x56,
    SecretLock = 0x57,
}

pub type Status = MemRegister<u8, 0x1F80_1800>;
pub type Controller = MemRegister<u8, 0x1F80_1801>;
pub type Parameter = MemRegister<u8, 0x1F80_1802>;
pub type Data = MemRegister<u16, 0x1F80_1802>;
pub type Request = MemRegister<u8, 0x1F80_1803>;
pub type Interrupt = MemRegister<u8, 0x1F80_1803>;
