use crate::gpu::vertex::Pixel;
use crate::{ro_register, wo_register};

pub mod color;
pub mod disp_port;
pub mod draw_port;
pub mod framebuffer;
pub mod vertex;

ro_register!(GpuRead, 0x1F80_1810);
ro_register!(GpuStat, 0x1F80_1814);
wo_register!(DrawPort, 0x1F80_1810);
wo_register!(DispPort, 0x1F80_1814);

pub enum Hres {
    H256,
    H320,
    H368,
    H512,
    H640,
}
pub enum Vres {
    V240,
    V480,
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

pub type Res = (Hres, Vres);

impl From<&Hres> for Pixel {
    fn from(h: &Hres) -> Pixel {
        match h {
            Hres::H256 => 256,
            Hres::H320 => 320,
            Hres::H368 => 368,
            Hres::H512 => 512,
            Hres::H640 => 640,
        }
    }
}

impl From<&Vres> for Pixel {
    fn from(v: &Vres) -> Pixel {
        match v {
            Vres::V240 => 240,
            Vres::V480 => 480,
        }
    }
}
