//! Gamepad input

use crate::sys::{critical_section, kernel};
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::read_volatile;
use strum_macros::IntoStaticStr;

pub const BUFFER_SIZE: usize = 0x22 / size_of::<u32>();

pub mod buttons {
    use super::Button;

    pub const SELECT: Button = Button::Select;
    pub const L3: Button = Button::L3;
    pub const R3: Button = Button::R3;
    pub const START: Button = Button::Start;
    pub const UP: Button = Button::Up;
    pub const RIGHT: Button = Button::Right;
    pub const DOWN: Button = Button::Down;
    pub const LEFT: Button = Button::Left;
    pub const L2: Button = Button::L2;
    pub const R2: Button = Button::R2;
    pub const L1: Button = Button::L1;
    pub const R1: Button = Button::R1;
    pub const TRIANGLE: Button = Button::Triangle;
    pub const CIRCLE: Button = Button::Circle;
    pub const CROSS: Button = Button::Cross;
    pub const SQUARE: Button = Button::Square;
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PolledValue(u16);

impl<'a, 'b> Gamepad<'a, 'b> {
    pub fn new(
        buf0: &'a mut [u32; BUFFER_SIZE], buf1: &'b mut [u32; BUFFER_SIZE],
    ) -> Result<Self, Error> {
        let buf0_ptr = buf0.as_mut_ptr() as *mut u8;
        let buf1_ptr = buf1.as_mut_ptr() as *mut u8;
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

    pub fn poll(&self) -> PolledValue {
        let val = unsafe { read_volatile(self.buf0_ptr.add(2).cast::<u16>()) };
        PolledValue(val)
    }
}

impl PolledValue {
    pub fn released(&self, button: Button) -> bool {
        self.0 & (1 << (button as u16)) != 0
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
