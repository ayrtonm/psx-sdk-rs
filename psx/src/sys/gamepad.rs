//! Gamepad input

use crate::sys::{critical_section, kernel};
use core::marker::PhantomData;
use core::ptr::read_volatile;
use strum_macros::IntoStaticStr;

pub const BUFFER_SIZE: usize = 0x22;

pub type Buffer = [u8; BUFFER_SIZE];

static mut PAD_INITIALIZED: bool = false;

#[derive(Debug)]
pub enum Button {
    Select = 0,
    L3,
    R3,
    Start,
    Up,
    Right,
    Down,
    Left,
    L2,
    R2,
    L1,
    R1,
    Triangle,
    Circle,
    Cross,
    Square,
}

#[derive(Debug, IntoStaticStr)]
pub enum Error {
    AlreadyInitialized,
}

#[derive(Debug)]
pub struct Gamepad<'a, 'b> {
    buf0_ptr: *mut u8,
    buf1_ptr: *mut u8,
    _buf0: PhantomData<&'a ()>,
    _buf1: PhantomData<&'b ()>,
}

impl<'a, 'b> Gamepad<'a, 'b> {
    pub fn new(
        buf0: &'a mut [u8; BUFFER_SIZE], buf1: &'b mut [u8; BUFFER_SIZE],
    ) -> Result<Self, Error> {
        let buf0_ptr = buf0.as_mut_ptr();
        let buf1_ptr = buf1.as_mut_ptr();
        critical_section(|| unsafe {
            if PAD_INITIALIZED {
                return Err(Error::AlreadyInitialized)
            }
            PAD_INITIALIZED = true;
            Ok(())
        })?;
        unsafe {
            kernel::start_pad();
            kernel::init_pad(buf0_ptr, BUFFER_SIZE, buf1_ptr, BUFFER_SIZE);
        }
        Ok(Gamepad {
            buf0_ptr,
            buf1_ptr,
            _buf0: PhantomData,
            _buf1: PhantomData,
        })
    }

    pub fn released(&self, button: Button) -> bool {
        let val = unsafe { read_volatile(self.buf0_ptr.add(2).cast::<u16>()) };
        val & (1 << (button as u16)) != 0
    }

    pub fn pressed(&self, button: Button) -> bool {
        !self.released(button)
    }
}

impl<'a, 'b> Drop for Gamepad<'a, 'b> {
    // If Gamepad is forgotten without running drop, PAD_INITIALIZED will remain
    // true so init_pad and start_pad won't be able to run again.
    fn drop(&mut self) {
        critical_section(|| unsafe {
            PAD_INITIALIZED = false;
        });
        unsafe {
            kernel::stop_pad();
        }
    }
}
