//! Gamepad data processing
//!
//! This module contains methods to process the BIOS's gamepad buffer data.

use super::kernel;
use core::pin::Pin;

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

#[derive(Debug)]
pub struct Buffer([u8; 0x22]);

impl Buffer {
    pub fn new() -> Self {
        Self([0; 0x22])
    }
}

impl !Unpin for Buffer {}

/// Processes buffer data for the gamepads.
#[derive(Debug)]
pub struct Controller<'a, 'b> {
    buf0: &'a mut Buffer,
    buf1: &'b mut Buffer,
}

impl<'a, 'b> Controller<'a, 'b> {
    pub fn new(buf0: &'a mut Buffer, buf1: &'b mut Buffer) -> Self {
        Self { buf0, buf1 }
    }

    pub fn init(self: &Pin<&mut Self>) {
        unsafe {
            kernel::init_pad(
                self.buf0.0.as_ptr() as *mut u8,
                0x22,
                self.buf1.0.as_ptr() as *mut u8,
                0x22,
            );
            kernel::start_pad()
        }
    }

    pub fn pressed(self: &Pin<&mut Self>, player: Player, button: Button) -> bool {
        let buffer = self.get_buffer(player);
        u16::from_le_bytes([buffer.0[2], buffer.0[3]]) & (1 << button as u16) == 0
    }

    pub fn released(self: &Pin<&mut Self>, player: Player, button: Button) -> bool {
        !self.pressed(player, button)
    }

    pub fn stop(self) {
        // impl Drop takes care of calling `stop_pad`
        //(self.buf0, self.buf1)
    }

    fn get_buffer<'c>(self: &'c Pin<&mut Self>, player: Player) -> &'c Buffer {
        match player {
            Player::P1 => self.buf0,
            Player::P2 => self.buf1,
        }
    }
}

impl Drop for Controller<'_, '_> {
    fn drop(&mut self) {
        unsafe { kernel::stop_pad() }
    }
}
