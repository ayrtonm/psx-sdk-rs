//! Gamepad data processing
//!
//! This module contains methods to process the BIOS's gamepad buffer data.

use super::kernel;
use core::pin::Pin;

type Buffer = [u8; 0x22];

pub enum Player {
    P1 = 0,
    P2,
}

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
    X,
    Square,
}

/// Processes buffer data for the gamepads.
#[derive(Debug)]
pub struct Controller<'a, 'b> {
    buf0: Pin<&'a mut Buffer>,
    buf1: Pin<&'b mut Buffer>,
}

impl<'a, 'b> Controller<'a, 'b> {
    pub fn new(buf0: &'a mut Buffer, buf1: &'b mut Buffer) -> Self {
        let mut gp = Self {
            buf0: Pin::new(buf0),
            buf1: Pin::new(buf1),
        };
        unsafe {
            kernel::init_pad(
                gp.buf0.as_mut_ptr(),
                gp.buf0.len(),
                gp.buf1.as_mut_ptr(),
                gp.buf1.len(),
            );
            kernel::start_pad()
        }
        gp
    }

    pub fn pressed(&self, player: Player, button: Button) -> bool {
        let buffer = self.get_buffer(player);
        u16::from_le_bytes([buffer[2], buffer[3]]) & (1 << button as u16) == 0
    }

    pub fn released(&self, player: Player, button: Button) -> bool {
        !self.pressed(player, button)
    }

    pub fn stop(self) -> (Buffer, Buffer) {
        // impl Drop takes care of calling `stop_pad`
        (*self.buf0, *self.buf1)
    }

    fn get_buffer(&self, player: Player) -> &Pin<&mut Buffer> {
        match player {
            Player::P1 => &self.buf0,
            Player::P2 => &self.buf1,
        }
    }
}

impl Drop for Controller<'_, '_> {
    fn drop(&mut self) {
        unsafe { kernel::stop_pad() }
    }
}
