use core::mem::size_of;

use super::color::Color;
use super::texture::{Clut, TexCoord, TexPage};
use super::vertex::Vertex;

pub struct Buffer<const N: usize> {
    pub data: [u32; N],
    pub nextpri: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        let data = [0; N];
        Buffer { data, nextpri: 0 }
    }

    pub fn get(&mut self, n: usize) -> &mut [u32] {
        self.nextpri += n;
        self.data.split_at_mut(self.nextpri).0
    }
}

pub struct Ptr<T>(*mut T);

impl<T> AsMut<T> for Ptr<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut().unwrap() }
    }
}

impl<T, const N: usize> AsMut<T> for Buffer<N> {
    fn as_mut(&mut self) -> &mut T {
        let ptr = self.get(size_of::<T>() / 4).as_mut_ptr().cast::<T>();
        unsafe { ptr.as_mut().unwrap() }
    }
}

macro_rules! impl_PolyF {
    ($n:expr, $name:ident, $cmd:expr) => {
        impl Ptr<$name> {
            pub fn as_ref(&self) -> &$name {
                unsafe { self.0.as_ref().unwrap() }
            }

            pub fn vertices<T>(&mut self, vertices: [T; $n]) -> &mut Self
            where Vertex: From<T> {
                self.as_mut().vertices = vertices.map(|t| Vertex::from(t));
                self
            }

            pub fn color(&mut self, color: Color) -> &mut Self {
                self.as_mut().color = color;
                self
            }

            pub fn tag(&mut self, tag: u32) -> &mut Self {
                self.as_mut().tag = tag;
                self
            }

            pub fn cmd(&mut self, cmd: u8) -> &mut Self {
                self.as_mut().cmd = cmd;
                self
            }
        }
        impl $name {
            pub fn from<const N: usize>(buffer: &mut Buffer<N>) -> Ptr<Self> {
                let mut prim = Ptr(AsMut::<Self>::as_mut(buffer) as *mut Self);
                prim.tag(0).cmd($cmd);
                prim
            }

            pub fn new<T, const N: usize>(
                buffer: &mut Buffer<N>, vertices: [T; $n], color: Color,
            ) -> Ptr<Self>
            where Vertex: From<T> {
                let mut prim = $name::from(buffer);
                prim.vertices(vertices).color(color);
                prim
            }
        }
    };
}

impl_PolyF!(3, PolyF3, 0x20);
impl_PolyF!(4, PolyF4, 0x28);

#[repr(C)]
pub struct PolyF3 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; 3],
}

#[repr(C)]
pub struct PolyF4 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; 4],
}

#[repr(C)]
pub struct PolyFT3 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad: u16,
}

#[repr(C)]
pub struct PolyFT4 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad0: u16,
    pub v3: Vertex,
    pub t3: TexCoord,
    pub _pad1: u16,
}

#[repr(C)]
pub struct PolyG3 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
}

#[repr(C)]
pub struct PolyG4 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub color3: Color,
    pub _pad2: u8,
    pub v3: Vertex,
}

#[repr(C)]
pub struct PolyGT3 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad2: u16,
}

#[repr(C)]
pub struct PolyGT4 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub t0: TexCoord,
    pub clut: Clut,
    pub color1: Color,
    pub _pad0: u8,
    pub v1: Vertex,
    pub t1: TexCoord,
    pub tpage: TexPage,
    pub color2: Color,
    pub _pad1: u8,
    pub v2: Vertex,
    pub t2: TexCoord,
    pub _pad2: u16,
    pub color3: Color,
    pub _pad3: u8,
    pub v3: Vertex,
    pub t3: TexCoord,
    pub _pad4: u16,
}

#[repr(C)]
pub struct LineF2 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineG2 {
    pub tag: u32,
    pub color0: Color,
    pub cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad: u8,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineF<const N: usize> {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub vertices: [Vertex; N],
    pub term: u32,
}

#[repr(C)]
pub struct ColoredVertex {
    pub c: Color,
    pub _pad: u8,
    pub v: Vertex,
}

#[repr(C)]
pub struct LineG<const N: usize> {
    pub tag: u32,
    pub colored_vertices: [ColoredVertex; N],
    pub term: u32,
}

#[repr(C)]
pub struct Tile {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub size: Vertex,
}

#[repr(C)]
pub struct Tile1 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile8 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile16 {
    pub tag: u32,
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Sprt {
    pub tag: u32,
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
