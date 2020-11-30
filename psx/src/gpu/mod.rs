pub mod color;
pub mod gp0;
pub mod gp1;
pub mod primitive;
pub mod stat;
pub mod texture;
pub mod vertex;

pub enum Vmode {
    NTSC,
    PAL,
}
pub enum Depth {
    Lo,
    Hi,
}

pub mod dma {
    pub enum Source {
        Off = 0,
        FIFO,
        CPU,
        GPU,
    }
}
