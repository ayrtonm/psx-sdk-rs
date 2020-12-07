use super::{Buffer, DoubleBuffer, DoublePacket, Init, Packet};
use crate::gpu::{Clut, Color, TexCoord, TexPage, Vertex};

#[repr(C)]
pub struct PolyF3 {
    pub color: Color,
    cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

#[repr(C)]
pub struct PolyF4 {
    pub color: Color,
    cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}

#[repr(C)]
pub struct PolyFT3 {
    pub color: Color,
    cmd: u8,
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
    pub color: Color,
    cmd: u8,
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
    pub color0: Color,
    cmd: u8,
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
    pub color0: Color,
    cmd: u8,
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
    pub color0: Color,
    cmd: u8,
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
    pub color0: Color,
    cmd: u8,
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
    pub color: Color,
    cmd: u8,
    pub v0: Vertex,
    pub v1: Vertex,
}

#[repr(C)]
pub struct LineF<const N: usize> {
    pub color: Color,
    cmd: u8,
    pub vertices: [Vertex; N],
    term: u32,
}

#[repr(C)]
pub struct LineG2 {
    pub color0: Color,
    cmd: u8,
    pub v0: Vertex,
    pub color1: Color,
    pub _pad: u8,
    pub v1: Vertex,
}

#[repr(C)]
pub struct ColoredVertex {
    pub c: Color,
    pub _pad: u8,
    pub v: Vertex,
}

#[repr(C)]
pub struct LineG<const N: usize> {
    pub colored_vertices: [ColoredVertex; N],
    term: u32,
}

#[repr(C)]
pub struct Tile {
    pub color: Color,
    pub cmd: u8,
    pub offset: Vertex,
    pub size: Vertex,
}

#[repr(C)]
pub struct Tile1 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile8 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}

#[repr(C)]
pub struct Tile16 {
    pub color: Color,
    cmd: u8,
    pub offset: Vertex,
}

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

macro_rules! impl_prim {
    ($name:ident, $cmd:expr) => {
        impl Init for $name {
            fn init(&mut self) {
                self.cmd();
            }
        }

        #[allow(non_snake_case)]
        impl<const N: usize> Buffer<N> {
            pub fn $name(&self) -> Option<&mut Packet<$name>> {
                self.alloc::<$name>()
            }
        }

        #[allow(non_snake_case)]
        impl<const N: usize> DoubleBuffer<N> {
            pub fn $name(&self) -> Option<DoublePacket<$name>> {
                self.alloc::<$name>()
            }
        }

        impl $name {
            pub(self) fn cmd(&mut self) -> &mut Self {
                self.cmd = $cmd;
                self
            }
        }
    };
}

macro_rules! impl_vertices {
    (1, $name:path) => {
        impl $name {
            pub fn offset<T>(&mut self, offset: T) -> &mut Self
            where Vertex: From<T> {
                self.offset = Vertex::from(offset);
                self
            }
        }
    };
    (2, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 2]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self
            }
        }
    };
    (3, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 3]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self.v2 = vertices[2];
                self
            }
        }
    };
    (4, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 4]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self.v2 = vertices[2];
                self.v3 = vertices[3];
                self
            }
        }
    };
}

macro_rules! impl_color {
    ($name:path) => {
        impl $name {
            pub fn color(&mut self, color: Color) -> &mut Self {
                self.color = color;
                self
            }
        }
    };
}

macro_rules! impl_gouraud {
    (2, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 2]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self
            }
        }
    };
    (3, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 3]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self.color2 = palette[2];
                self
            }
        }
    };
    (4, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 4]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self.color2 = palette[2];
                self.color3 = palette[3];
                self
            }
        }
    };
}

impl_prim!(PolyF3, 0x20);
impl_prim!(PolyF4, 0x28);
impl_prim!(PolyFT3, 0x24);
impl_prim!(PolyFT4, 0x2C);

impl_prim!(PolyG3, 0x30);
impl_prim!(PolyG4, 0x38);
impl_prim!(PolyGT3, 0x34);
impl_prim!(PolyGT4, 0x3C);

impl_prim!(LineF2, 0x40);
// TODO: LineF<N>
impl_prim!(LineG2, 0x50);
// TODO: LineG<N>
impl_prim!(Tile, 0x60);
impl_prim!(Tile1, 0x68);
impl_prim!(Tile8, 0x70);
impl_prim!(Tile16, 0x78);
impl_prim!(Sprt, 0x64);
impl_prim!(Sprt8, 0x74);
impl_prim!(Sprt16, 0x7C);

mod vertices {
    use super::*;
    impl_vertices!(3, PolyF3);
    impl_vertices!(4, PolyF4);
    impl_vertices!(3, PolyFT3);
    impl_vertices!(4, PolyFT4);

    impl_vertices!(3, PolyG3);
    impl_vertices!(4, PolyG4);
    impl_vertices!(3, PolyGT3);
    impl_vertices!(4, PolyGT4);

    impl_vertices!(2, LineF2);
    impl<const N: usize> LineF<N> {
        pub fn vertices<T>(&mut self, vertices: [T; N]) -> &mut Self
        where Vertex: From<T> {
            self.vertices = vertices.map(|t| Vertex::from(t));
            self
        }
    }
    // TODO: LineG2
    // TODO: LineG<N>
    impl_vertices!(1, Tile);
    impl_vertices!(1, Tile1);
    impl_vertices!(1, Tile8);
    impl_vertices!(1, Tile16);
    impl_vertices!(1, Sprt);
    impl_vertices!(1, Sprt8);
    impl_vertices!(1, Sprt16);
}

mod color {
    use super::*;
    impl_color!(PolyF3);
    impl_color!(PolyF4);
    impl_color!(PolyFT3);
    impl_color!(PolyFT4);

    impl_gouraud!(3, PolyG3);
    impl_gouraud!(4, PolyG4);
    impl_gouraud!(3, PolyGT3);
    impl_gouraud!(4, PolyGT4);

    impl_color!(LineF2);
    impl<const N: usize> LineF<N> {
        pub fn color(&mut self, color: Color) -> &mut Self {
            self.color = color;
            self
        }
    }
    impl_gouraud!(2, LineG2);
    // TODO: LineG<N>

    impl_color!(Tile);
    impl_color!(Tile1);
    impl_color!(Tile8);
    impl_color!(Tile16);
    impl_color!(Sprt);
    impl_color!(Sprt8);
    impl_color!(Sprt16);
}

// TODO: make this into a macro
impl Sprt {
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
