use core::mem::size_of;
use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::gpu::{Clut, Color, TexCoord, Vertex};

#[macro_use]
mod macros;
mod buffer;
mod ot;
mod packet;
pub mod primitive;

pub use buffer::{Buffer, DoubleBuffer};
pub use ot::{DoubleOT, OT};
pub use packet::{DoublePacket, Packet};

pub trait Primitive: Sized {
    fn as_slice(&self) -> &[u32] {
        let size = packet_size::<Self>();
        unsafe { from_raw_parts(self as *const Self as *const u32, size) }
    }
    // Use this to unzip a file into a buffer-allocated prim
    fn as_mut_slice(&mut self) -> &mut [u32] {
        let size = packet_size::<Self>();
        unsafe { from_raw_parts_mut(self as *mut Self as *mut u32, size) }
    }
}

pub trait Init {
    fn init(&mut self);
}

impl<T> Primitive for T where T: Init {}

pub const fn packet_size<T>() -> usize {
    size_of::<Packet<T>>() / 4
}

use primitive::LineF;
use primitive::LineF2;
// TODO: impl this
//use primitive::LineG;
use primitive::LineG2;
use primitive::PolyF3;
use primitive::PolyF4;
use primitive::PolyFT3;
use primitive::PolyFT4;
use primitive::PolyG3;
use primitive::PolyG4;
use primitive::PolyGT3;
use primitive::PolyGT4;
use primitive::Sprt;
use primitive::Sprt16;
use primitive::Sprt8;
use primitive::Tile;
use primitive::Tile1;
use primitive::Tile16;
use primitive::Tile8;

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
    impl_vertices!(PolyF3, 3);
    impl_vertices!(PolyF4, 4);
    impl_vertices!(PolyFT3, 3);
    impl_vertices!(PolyFT4, 4);

    impl_vertices!(PolyG3, 3);
    impl_vertices!(PolyG4, 4);
    impl_vertices!(PolyGT3, 3);
    impl_vertices!(PolyGT4, 4);

    impl_vertices!(LineF2, 2);
    impl<const N: usize> LineF<N> {
        pub fn vertices<T>(&mut self, vertices: [T; N]) -> &mut Self
        where Vertex: From<T> {
            self.vertices = vertices.map(|t| Vertex::from(t));
            self
        }
    }
    // TODO: LineG2
    // TODO: LineG<N>
    impl_vertices!(Tile, 1);
    impl_vertices!(Tile1, 1);
    impl_vertices!(Tile8, 1);
    impl_vertices!(Tile16, 1);
    impl_vertices!(Sprt, 1);
    impl_vertices!(Sprt8, 1);
    impl_vertices!(Sprt16, 1);
}

mod color {
    use super::*;
    impl_color!(PolyF3);
    impl_color!(PolyF4);
    impl_color!(PolyFT3);
    impl_color!(PolyFT4);

    impl_gouraud!(PolyG3, 3);
    impl_gouraud!(PolyG4, 4);
    impl_gouraud!(PolyGT3, 3);
    impl_gouraud!(PolyGT4, 4);

    impl_color!(LineF2);
    impl<const N: usize> LineF<N> {
        pub fn color(&mut self, color: Color) -> &mut Self {
            self.color = color;
            self
        }
    }
    impl_gouraud!(LineG2, 2);
    // TODO: LineG<N>

    impl_color!(Tile);
    impl_color!(Tile1);
    impl_color!(Tile8);
    impl_color!(Tile16);
    impl_color!(Sprt);
    impl_color!(Sprt8);
    impl_color!(Sprt16);
}

// TODO: make these into macros
impl Sprt8 {
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
}

impl Sprt {
    pub fn size<T>(&mut self, size: T) -> &mut Self
    where Vertex: From<T> {
        self.size = size.into();
        self
    }
}
