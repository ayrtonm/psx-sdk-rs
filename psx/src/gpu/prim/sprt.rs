use super::{Buffer, DoubleBuffer, DoublePacket, Init, Packet};
use crate::gpu::Color;
use crate::gpu::Vertex;
use crate::gpu::{Clut, TexCoord};

#[repr(C)]
pub struct Sprt {
    //pub tag: u32,
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub size: Vertex,
}

#[repr(C)]
pub struct Sprt8 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}

#[repr(C)]
pub struct Sprt16 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}

impl Init for Sprt {
    fn init(&mut self) {
        self.cmd();
    }
}

#[allow(non_snake_case)]
impl<const N: usize> Buffer<N> {
    pub fn Sprt(&self) -> Option<&mut Packet<Sprt>> {
        self.alloc::<Sprt>()
    }
}

#[allow(non_snake_case)]
impl<const N: usize> DoubleBuffer<N> {
    pub fn Sprt(&self) -> Option<DoublePacket<Sprt>> {
        self.alloc::<Sprt>()
    }
}

impl Sprt {
    pub fn cmd(&mut self) -> &mut Self {
        self.cmd = 0x64;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn offset<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        self.offset = offset.into();
        self
    }

    pub fn t0<T>(&mut self, t0: T) -> &mut Self
    where TexCoord: From<T> {
        self.t0 = t0.into();
        self
    }

    pub fn clut<T>(&mut self, clut: T) -> &mut Self
    where Clut: From<T> {
        self.clut = clut.into();
        self
    }

    pub fn size<T>(&mut self, size: T) -> &mut Self
    where Vertex: From<T> {
        self.size = size.into();
        self
    }
}
