use std::mem::size_of;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U16LSB([u8; size_of::<u16>()]);

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U32LSB([u8; size_of::<u32>()]);

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U16MSB([u8; size_of::<u16>()]);

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U32MSB([u8; size_of::<u32>()]);

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U16([u8; size_of::<u16>() * 2]);

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct U32([u8; size_of::<u32>() * 2]);

impl From<u16> for U16LSB {
    fn from(x: u16) -> Self {
        Self(x.to_le_bytes())
    }
}
impl From<u32> for U32LSB {
    fn from(x: u32) -> Self {
        Self(x.to_le_bytes())
    }
}
impl From<u16> for U16MSB {
    fn from(x: u16) -> Self {
        Self(x.to_be_bytes())
    }
}
impl From<u32> for U32MSB {
    fn from(x: u32) -> Self {
        Self(x.to_be_bytes())
    }
}
impl From<u16> for U16 {
    fn from(x: u16) -> Self {
        let mut u = [0; 4];
        u[0..2].copy_from_slice(&x.to_le_bytes());
        u[2..4].copy_from_slice(&x.to_be_bytes());
        Self(u)
    }
}
impl From<u32> for U32 {
    fn from(x: u32) -> Self {
        let mut u = [0; 8];
        u[0..4].copy_from_slice(&x.to_le_bytes());
        u[4..8].copy_from_slice(&x.to_be_bytes());
        Self(u)
    }
}
