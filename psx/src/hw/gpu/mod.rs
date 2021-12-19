//! GPU registers
use crate::hw::MemRegister;
use core::mem::size_of;
use core::slice;

mod gp0;
mod gp1;
mod response;
mod status;
mod tests;

pub struct GP0(MemRegister<u32, 0x1F80_1810>);
pub struct GP1(MemRegister<u32, 0x1F80_1814>);
pub struct Response(MemRegister<u32, 0x1F80_1810>);
pub struct Status(MemRegister<u32, 0x1F80_1814>);

/// A GP0 command which can be serialized into a slice of u32 words.
pub trait GP0Command: Sized {
    fn words(&self) -> &[u32] {
        let ptr = self as *const Self as *const u32;
        let len = size_of::<Self>() / size_of::<u32>();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}
