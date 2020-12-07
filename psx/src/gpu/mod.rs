mod color;
pub mod gp0;
pub mod gp1;
pub mod prim;
pub mod stat;
mod texture;
mod vertex;

pub use vertex::Pixel;
pub use vertex::Vertex;

pub use color::Color;

pub use texture::Bpp;
pub use texture::Clut;
pub use texture::TexCoord;
pub use texture::TexPage;

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
