use crate::{ro_register, wo_register};

pub mod attributes;
pub mod color;
pub mod display;
pub mod draw;
pub mod framebuffer;
pub mod res;
pub mod vertex;
pub mod vram;

ro_register!(GpuRead, 0x1F80_1810);
ro_register!(GpuStat, 0x1F80_1814);
wo_register!(DrawEnv, 0x1F80_1810);
wo_register!(DisplayEnv, 0x1F80_1814);
