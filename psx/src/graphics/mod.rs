//! Graphics subsystem routines and primitive data types

use crate::gpu::Primitive;
use core::mem::size_of;
use core::slice::from_raw_parts;

mod buffer;
mod disp_env;
mod draw_env;
mod packet;
pub mod primitive;

const TERMINATION: u32 = 0x00FF_FFFF;

pub use buffer::{Buffer, DoubleBuffer};
pub use disp_env::DispEnv;
pub use draw_env::DrawEnv;
pub use packet::{DoubleRef, Packet, Ref};

pub trait Initialize {
    fn init(&mut self);
}

/// Gets a slice of the words in an entire repr(C) struct
pub trait AsSlice: Sized {
    fn as_slice(&self) -> &[u32] {
        unsafe { from_raw_parts(self as *const Self as *const u32, num_words::<Self>()) }
    }
}

impl<T: Initialize> AsSlice for T {}
impl<T: Initialize> Primitive for T {
    fn primitive(&self) -> &[u32] {
        self.as_slice()
    }
}

pub const fn num_words<T>() -> usize {
    size_of::<T>() / 4
}
