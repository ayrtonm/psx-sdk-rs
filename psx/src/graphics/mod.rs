use crate::num_words;
use core::slice::from_raw_parts;

mod buffer;
mod packet;
pub mod primitive;

const TERMINATION: u32 = 0x00FF_FFFF;

pub use buffer::{Buffer, DoubleBuffer};
pub use packet::{DoubleRef, Packet, Ref};

pub trait Initialize {
    fn init(&mut self);
}

pub trait AsSlice: Sized {
    fn as_slice(&self) -> &[u32] {
        unsafe { from_raw_parts(self as *const Self as *const u32, num_words::<Self>()) }
    }
}

impl<T: Initialize> AsSlice for T {}
