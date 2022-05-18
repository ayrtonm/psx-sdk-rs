//! GPU registers
use crate::hw::MemRegister;
use core::mem::size_of;
use core::slice;

mod gp0;
mod gp1;
mod status;
mod tests;

/// A port used to send GP0 commands.
pub type GP0 = MemRegister<u32, 0x1F80_1810>;
/// A port used to send GP1 commands.
pub type GP1 = MemRegister<u32, 0x1F80_1814>;
/// The register that receives GPU responses.
pub type Response = MemRegister<u32, 0x1F80_1810>;
// This is a struct rather than a type to allow overriding the derived Debug
// impl.
/// The GPU status register.
pub struct Status(MemRegister<u32, 0x1F80_1814>);

/// A struct whose memory layout is a valid GP0 command.
pub trait GP0Command: Sized {
    /// Gets `self`'s memory as a slice of 32-bit words.
    fn data(&self) -> &[u32] {
        let ptr = self as *const Self as *const u32;
        let len = size_of::<Self>() / size_of::<u32>();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}
