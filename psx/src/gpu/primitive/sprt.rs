use super::{Buffer, Init, Packet};
use crate::gpu::color::Color;
use crate::gpu::texture::{Clut, TexCoord};
use crate::gpu::vertex::Vertex;

#[repr(C)]
pub struct Sprt {
    //pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub size: Vertex,
}

#[repr(C)]
pub struct Sprt8 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
}

#[repr(C)]
pub struct Sprt16 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
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
impl Packet<Sprt> {
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.packet.color(color);
        self
    }

    pub fn offset<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        self.packet.offset(offset);
        self
    }

    pub fn t0<T>(&mut self, t0: T) -> &mut Self
    where TexCoord: From<T> {
        self.packet.t0(t0);
        self
    }

    pub fn clut<T>(&mut self, clut: T) -> &mut Self
    where Clut: From<T> {
        self.packet.clut(clut);
        self
    }

    pub fn size<T>(&mut self, size: T) -> &mut Self
    where Vertex: From<T> {
        self.packet.size(size);
        self
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
