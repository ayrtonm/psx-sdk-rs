use core::intrinsics::{volatile_load, volatile_store};

use crate::rw_register;

pub mod vertex;
pub mod color;
pub mod draw;
pub mod vram;
pub mod env;

pub struct GP0;
pub struct GP1;

rw_register!(GP0, 0x1F80_1810);
rw_register!(GP1, 0x1F80_1814);
