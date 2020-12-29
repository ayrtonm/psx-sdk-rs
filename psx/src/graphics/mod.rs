/// Single- and Double-buffered primitive bump allocators.
pub mod buffer;

/// Single- and Double-buffered Ordering tables.
pub mod ot;

/// Single- and Double-buffered packets.
pub mod packet;

/// Primitive definitions.
pub mod primitive;

/// An interface for initializing graphics primitives.
pub trait InitPrimitive: Sized {
    /// Sets the GPU commands for the primitive data in a packet.
    fn init_primitive(&mut self);
}

/// An interface for accessing linked lists.
pub trait LinkedList {
    /// Gets the start address of the linked list.
    fn start_address(&self) -> &u32;
}

pub(self) const TERMINATION: u32 = 0x00FF_FFFF;
