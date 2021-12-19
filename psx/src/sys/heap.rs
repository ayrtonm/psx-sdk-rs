//! Dynamic memory allocation
//!
//! This module provides dynamic memory allocation backed by the BIOS's
//! `malloc`, `heap_init` and `free`.

use crate::sys::{critical_section, kernel};
use core::ops::{Deref, DerefMut};
use core::slice;

/// A heap backed by the BIOS's [`kernel::malloc`] and [`kernel::free`].
#[derive(Debug)]
pub struct Heap(());

/// A mutable reference to a heap-allocated buffer.
///
/// This buffer can be used as a `u8` [`prim@slice`].
#[derive(Debug)]
pub struct Buffer<'b>(&'b mut [u8]);

impl<'b> Deref for Buffer<'b> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'b> DerefMut for Buffer<'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'b> Drop for Buffer<'b> {
    fn drop(&mut self) {
        let ptr = self.as_mut_ptr().cast();
        unsafe { kernel::free(ptr) }
    }
}

/// Heap initialization errors.
pub enum InitError {
    /// The heap has already been initialized.
    AlreadyInitialized,
}

/// Memory allocation errors.
pub enum AllocError {
    /// Requested buffer size is not a multiple of 4.
    InvalidSize,
    /// Heap is out of memory.
    OutOfMemory,
}

impl Heap {
    /// Initializes a new heap.
    fn init(heap: &mut [u32]) -> Result<Self, InitError> {
        // SAFETY: The start and end of the heap do not overlap with any sections
        // defined in the linker script and are aligned and sized correctly.
        unsafe {
            kernel::init_heap(heap.as_ptr() as usize, heap.len());
        }
        Ok(Heap(()))
    }

    /// Creates a new heap if one has not already been created.
    pub fn new(heap: &mut [u32]) -> Result<Self, InitError> {
        static mut HEAP_INITIALIZED: bool = false;
        critical_section(||
            // SAFETY: Interrupts are disabled within a critical section allowing safe access to
            // HEAP_INITIALIZED.
            unsafe {
                if HEAP_INITIALIZED {
                    Err(InitError::AlreadyInitialized)
                } else {
                    HEAP_INITIALIZED = true;
                    Ok(())
                }
            })?;
        Self::init(heap)
    }

    /// Moves the heap to a new region of memory, dropping all existing buffers.
    pub fn reinit(self, heap: &mut [u32]) -> Result<Self, InitError> {
        Self::init(heap)
    }

    /// Allocates a buffer. The requested size must be a multiple of 4.
    pub fn malloc(&self, bytes: usize) -> Result<Buffer, AllocError> {
        let invalid_size = (bytes & 3) != 0;
        if invalid_size {
            return Err(AllocError::InvalidSize)
        }
        let ptr = unsafe { kernel::malloc(bytes) };
        if ptr.is_null() {
            return Err(AllocError::OutOfMemory)
        }
        let slice = unsafe { slice::from_raw_parts_mut(ptr, bytes) };
        Ok(Buffer(slice))
    }

    /// Explicitly frees a buffer.
    pub fn free(&self, _buffer: Buffer) {
        // Buffer's Drop impl frees its memory
    }
}
