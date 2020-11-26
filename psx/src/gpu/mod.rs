use crate::{ro_register, wo_register};

pub mod color;
pub mod disp_port;
pub mod draw_port;
pub mod framebuffer;
pub mod gpu_stat;
pub mod primitives;
pub mod texture;
pub mod vertex;
pub mod vram;

ro_register!(GpuRead, 0x1F80_1810);
ro_register!(GpuStat, 0x1F80_1814);
wo_register!(DrawPort, 0x1F80_1810);
wo_register!(DispPort, 0x1F80_1814);

pub trait AsU32 {
    fn as_u32(&self) -> u32;
}

pub trait Packet<const N: usize> {
    fn packet(&self) -> [u32; N];
}

impl<const N: usize> Packet<N> for [u32; N] {
    fn packet(&self) -> [u32; N] {
        *self
    }
}

pub enum Vmode {
    NTSC,
    PAL,
}
pub enum Depth {
    Lo,
    Hi,
}
pub enum DmaSource {
    Off,
    FIFO,
    CPU,
    GPU,
}
