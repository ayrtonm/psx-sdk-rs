//! Gamepad input

use crate::sys::{critical_section, kernel};
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::read_volatile;
use core::slice;
use strum_macros::IntoStaticStr;

// This is the real minimum buffer size
const BUFFER_BYTES: usize = 0x22;

// This wrapper takes slightly larger buffers to ensure the buffer can be a
// &[u32].
const BUFFER_SIZE: usize = (BUFFER_BYTES + 2) / size_of::<u32>();

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PadType {
    Mouse = 0x12,
    NegCon = 0x23,
    Konami = 0x31,
    Digital = 0x41,
    AnalogStick = 0x53,
    NamcoLightgun = 0x63,
    AnalogPad = 0x73,
    Multitap = 0x80,
    Jogcon = 0xE3,
    ConfigMode = 0xF3,
    Disconnected = 0xFF,
    Unknown = 0,
}

impl From<u8> for PadType {
    fn from(val: u8) -> PadType {
        match val {
            0x12 => PadType::Mouse,
            0x23 => PadType::NegCon,
            0x31 => PadType::Konami,
            0x41 => PadType::Digital,
            0x53 => PadType::AnalogStick,
            0x63 => PadType::NamcoLightgun,
            0x73 => PadType::AnalogPad,
            0x80 => PadType::Multitap,
            0xE3 => PadType::Jogcon,
            0xF3 => PadType::ConfigMode,
            0xFF => PadType::Disconnected,
            _ => PadType::Unknown,
        }
    }
}

#[derive(Debug, IntoStaticStr)]
pub enum Error {
    AlreadyInitialized,
}

pub struct Gamepad<'a, 'b> {
    buf0_ptr: *mut u8,
    buf1_ptr: *mut u8,
    _buf0: PhantomData<&'a mut [u32; BUFFER_SIZE]>,
    _buf1: PhantomData<&'b mut [u32; BUFFER_SIZE]>,
    center_p0_left: u16,
    center_p0_right: u16,
    center_p1_left: u16,
    center_p1_right: u16,
}

// TODO: Make a custom PartialEq/Eq using a threshold
#[derive(Copy, Clone)]
pub struct Stick {
    pub vertical: i8,
    pub horizontal: i8,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PolledButtons(u16);

impl<'a, 'b> Gamepad<'a, 'b> {
    pub const BUFFER_SIZE: usize = BUFFER_SIZE;
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
            kernel::init_pad(buf0_ptr, BUFFER_BYTES, buf1_ptr, BUFFER_BYTES);
            buf0[0] = 0xFF;
            buf0[1] = 0xFF;
            buf1[0] = 0xFF;
            buf1[1] = 0xFF;
            kernel::start_pad();
            kernel::change_clear_pad(1);
        }
        Ok(Gamepad {
            buf0_ptr,
            buf1_ptr,
            _buf0: PhantomData,
            _buf1: PhantomData,
            center_p0_left: 0x8080,
            center_p0_right: 0x8080,
            center_p1_left: 0x8080,
            center_p1_right: 0x8080,
        })
    }

    pub fn calibrate(&mut self) {
        //crate::println!("{:x?}", self);
        //unsafe {
        //    kernel::change_clear_pad(1);
        //}
        let mut x = 0;
        let mut y = 0;
        let n = 2;
        let samples = 1 << n;
        for _ in 0..samples {
            let sample = unsafe { read_volatile(self.buf0_ptr.cast::<u16>().add(3)) };
            x += (sample as u8) as u32;
            y += ((sample >> 8) as u8) as u32;
        }
        x >>= n;
        y >>= n;
        let val = x | (y << 8);
        self.center_p0_left = val as u16;
    }

    pub fn poll(&self) -> PolledButtons {
        PolledButtons(unsafe { read_volatile(self.buf0_ptr.cast::<u16>().add(1)) })
    }

    const RIGHT_STICK_OFFSET: usize = 2;
    const LEFT_STICK_OFFSET: usize = 3;

    fn read_stick(&self, offset: usize) -> Stick {
        let center = if offset == Self::LEFT_STICK_OFFSET {
            self.center_p0_left
        } else {
            self.center_p0_right
        };
        let val = unsafe { read_volatile(self.buf0_ptr.cast::<u16>().add(offset)) - center };
        let horizontal = (val as u8) as i8;
        let vertical = ((val >> 8) as u8) as i8;
        Stick {
            vertical,
            horizontal,
        }
    }

    pub fn left_stick(&self) -> Stick {
        self.read_stick(Self::LEFT_STICK_OFFSET)
    }

    pub fn right_stick(&self) -> Stick {
        self.read_stick(Self::RIGHT_STICK_OFFSET)
    }

    pub fn info(&self) -> PadType {
        let val = unsafe { read_volatile(self.buf0_ptr.add(1)) };
        PadType::from(val)
    }
}
impl PolledButtons {
    pub fn released(&self, button: Button) -> bool {
        self.0 & (1 << (button as u16)) != 0
    }

    pub fn pressed(&self, button: Button) -> bool {
        !self.released(button)
    }
}

impl<'a, 'b> Debug for Gamepad<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let player_0 = unsafe { slice::from_raw_parts(self.buf0_ptr as *const u8, BUFFER_BYTES) };
        let player_1 = unsafe { slice::from_raw_parts(self.buf1_ptr as *const u8, BUFFER_BYTES) };
        f.debug_struct("sys::Gamepad")
            .field("player_0", &player_0)
            .field("player_1", &player_1)
            .finish()
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
