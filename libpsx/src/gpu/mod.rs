use crate::wo_register;

pub mod vertex;
pub mod color;
pub mod draw;
pub mod vram;
pub mod env;

pub struct GP0;
pub struct GP1;

wo_register!(GP0, 0x1F80_1810);
wo_register!(GP1, 0x1F80_1814);
