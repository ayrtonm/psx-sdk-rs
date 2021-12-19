//! Gamepad input
use super::kernel;

const BUFFER_SIZE: usize = 0x22;

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

#[derive(Debug)]
pub struct GamePad {
    pub buf0: *mut u8,
    buf1: *mut u8,
}

impl GamePad {
    pub fn buffer() -> [u8; BUFFER_SIZE] {
        [0; BUFFER_SIZE]
    }

    pub fn new(buf0: &mut [u8; BUFFER_SIZE], buf1: &mut [u8; BUFFER_SIZE]) -> Self {
        unsafe {
            kernel::start_pad();
            kernel::init_pad(
                buf0.as_mut_ptr(),
                buf0.len(),
                buf1.as_mut_ptr(),
                buf1.len(),
            );
        }
        GamePad {
            buf0: buf0.as_mut_ptr(),
            buf1: buf1.as_mut_ptr(),
        }
    }

    pub fn released(&self, button: Button) -> bool {
        let val = unsafe { *self.buf0.add(2).cast::<u16>() };
        val & (1 << (button as u16)) != 0
    }

    pub fn pressed(&self, button: Button) -> bool {
        !self.released(button)
    }
}

impl Drop for GamePad {
    fn drop(&mut self) {
        unsafe {
            kernel::stop_pad();
        }
    }
}
