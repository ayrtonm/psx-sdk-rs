use crate::gpu::color::{Color, Palette};
use crate::gpu::texture::{Clut, Coord, Page};
use crate::gpu::vertex::{Polygon, Vertex};
use crate::gpu::{AsU32, Packet};

pub struct Primitive<const N: usize> {
    vertices: Polygon<N>,
    color: Color,
}

// This is to avoid conflicting impl of Packet
// TODO: combine this with Primitive when specialization becomes possible
pub struct Curve<const N: usize>(Primitive<N>);

pub struct ShadedPrimitive<const N: usize> {
    vertices: Polygon<N>,
    palette: Palette<N>,
}

pub struct ShadedCurve<const N: usize>(ShadedPrimitive<N>);

pub struct TexturedPrimitive<const N: usize> {
    primitive: Primitive<N>,
    tex_coords: [Coord; N],
    clut: Clut,
    tex_page: Page,
}

pub struct Rectangle {
    offset: Vertex,
    size: Vertex,
    color: Color,
}

pub struct Point {
    offset: Vertex,
    color: Color,
}

pub struct SmallSquare {
    offset: Vertex,
    color: Color,
}

pub struct MediumSquare {
    offset: Vertex,
    color: Color,
}

macro_rules! primitive_fn {
    ($name:ident, $N:expr) => {
        pub fn $name<T>(vertices: [T; $N], color: Color) -> Primitive<$N>
        where Vertex: From<T> {
            Primitive::<$N> {
                vertices: vertices.map(|v| Vertex::from(v)),
                color,
            }
        }
    };
}

macro_rules! textured_primitive_fn {
    ($name:ident, $N:expr) => {
        pub fn $name<T, U, V>(
            vertices: [T; $N], color: Color, tex_coords: [U; $N], clut: Clut, tex_page: V,
        ) -> TexturedPrimitive<$N>
        where
            Vertex: From<T>,
            Coord: From<U>,
            Page: From<V>, {
            TexturedPrimitive::<$N> {
                primitive: Primitive::<$N> {
                    vertices: vertices.map(|v| Vertex::from(v)),
                    color,
                },
                tex_coords: tex_coords.map(|t| Coord::from(t)),
                clut,
                tex_page: Page::from(tex_page),
            }
        }
    };
}

macro_rules! shaded_primitive_fn {
    ($name:ident, $N:expr) => {
        pub fn $name<T>(vertices: [T; $N], palette: Palette<$N>) -> ShadedPrimitive<$N>
        where Vertex: From<T> {
            ShadedPrimitive::<$N> {
                vertices: vertices.map(|v| Vertex::from(v)),
                palette,
            }
        }
    };
}

macro_rules! square_fn {
    ($name:ident, $struct:tt) => {
        pub fn $name<T>(offset: T, color: Color) -> $struct
        where Vertex: From<T> {
            $struct {
                offset: Vertex::from(offset),
                color,
            }
        }
    };
}

primitive_fn!(triangle, 3);
primitive_fn!(quad, 4);
primitive_fn!(line, 2);
shaded_primitive_fn!(shaded_triangle, 3);
shaded_primitive_fn!(shaded_quad, 4);
shaded_primitive_fn!(shaded_line, 2);
textured_primitive_fn!(textured_triangle, 3);
textured_primitive_fn!(textured_quad, 4);
pub fn curve<T, const N: usize>(vertices: [T; N], color: Color) -> Curve<N>
where Vertex: From<T> {
    Curve::<N>(Primitive::<N> {
        vertices: vertices.map(|v| Vertex::from(v)),
        color,
    })
}

pub fn shaded_curve<T, const N: usize>(vertices: [T; N], palette: Palette<N>) -> ShadedCurve<N>
where Vertex: From<T> {
    ShadedCurve::<N>(ShadedPrimitive::<N> {
        vertices: vertices.map(|v| Vertex::from(v)),
        palette,
    })
}

pub fn rectangle<T, U>(offset: T, size: U, color: Color) -> Rectangle
where Vertex: From<T> + From<U> {
    Rectangle {
        offset: Vertex::from(offset),
        size: Vertex::from(size),
        color,
    }
}
square_fn!(point, Point);
square_fn!(small_square, SmallSquare);
square_fn!(medium_square, MediumSquare);

impl<const N: usize> ShadedPrimitive<N> {
    pub fn map<F>(&mut self, f: F)
    where F: Fn(&mut Polygon<N>, &mut Palette<N>) {
        f(&mut self.vertices, &mut self.palette);
    }
}

impl<const N: usize> TexturedPrimitive<N> {
    pub fn map<F>(&mut self, f: F)
    where F: Fn(&mut Polygon<N>, &mut Color, &mut Page) {
        f(
            &mut self.primitive.vertices,
            &mut self.primitive.color,
            &mut self.tex_page,
        );
    }
}

impl Packet<4> for Primitive<3> {
    fn packet(&self) -> [u32; 4] {
        [
            0x20 << 24 | self.color.as_u32(),
            self.vertices[0].as_u32(),
            self.vertices[1].as_u32(),
            self.vertices[2].as_u32(),
        ]
    }
}

impl Packet<5> for Primitive<4> {
    fn packet(&self) -> [u32; 5] {
        [
            0x28 << 24 | self.color.as_u32(),
            self.vertices[0].as_u32(),
            self.vertices[1].as_u32(),
            self.vertices[2].as_u32(),
            self.vertices[3].as_u32(),
        ]
    }
}

impl Packet<6> for ShadedPrimitive<3> {
    fn packet(&self) -> [u32; 6] {
        [
            0x30 << 24 | self.palette[0].as_u32(),
            self.vertices[0].as_u32(),
            self.palette[1].as_u32(),
            self.vertices[1].as_u32(),
            self.palette[2].as_u32(),
            self.vertices[2].as_u32(),
        ]
    }
}

impl Packet<8> for ShadedPrimitive<4> {
    fn packet(&self) -> [u32; 8] {
        [
            0x38 << 24 | self.palette[0].as_u32(),
            self.vertices[0].as_u32(),
            self.palette[1].as_u32(),
            self.vertices[1].as_u32(),
            self.palette[2].as_u32(),
            self.vertices[2].as_u32(),
            self.palette[3].as_u32(),
            self.vertices[3].as_u32(),
        ]
    }
}

impl Packet<3> for Primitive<2> {
    fn packet(&self) -> [u32; 3] {
        [
            0x40 << 24 | self.color.as_u32(),
            self.vertices[0].as_u32(),
            self.vertices[1].as_u32(),
        ]
    }
}

impl Packet<4> for ShadedPrimitive<2> {
    fn packet(&self) -> [u32; 4] {
        [
            0x50 << 24 | self.palette[0].as_u32(),
            self.vertices[0].as_u32(),
            self.palette[1].as_u32(),
            self.vertices[1].as_u32(),
        ]
    }
}

// TODO: constrain M = N + 2
impl<const M: usize, const N: usize> Packet<M> for Curve<N> {
    fn packet(&self) -> [u32; M] {
        let mut ar = [0; M];
        ar[0] = 0x48 << 24 | self.0.color.as_u32();
        for i in 0..N {
            ar[i + 1] = self.0.vertices[i].as_u32();
        }
        ar[N + 1] = 0x5555_5555;
        ar
    }
}

// TODO: constrain M = (2 * N) + 1
impl<const M: usize, const N: usize> Packet<M> for ShadedCurve<N> {
    fn packet(&self) -> [u32; M] {
        let mut ar = [0; M];
        for i in 0..N {
            ar[2 * i] = self.0.palette[i].as_u32();
            ar[(2 * i) + 1] = self.0.vertices[i].as_u32();
        }
        ar[0] |= 0x58 << 24;
        ar[N + 1] = 0x5555_5555;
        ar
    }
}

impl Packet<3> for Rectangle {
    fn packet(&self) -> [u32; 3] {
        [
            0x60 << 24 | self.color.as_u32(),
            self.offset.as_u32(),
            self.size.as_u32(),
        ]
    }
}

macro_rules! impl_square {
    ($cmd:expr, $struct:ty) => {
        impl Packet<2> for $struct {
            fn packet(&self) -> [u32; 2] {
                [$cmd << 24 | self.color.as_u32(), self.offset.as_u32()]
            }
        }
    };
}

impl_square!(0x68, Point);
impl_square!(0x70, SmallSquare);
impl_square!(0x78, MediumSquare);

impl Packet<7> for TexturedPrimitive<3> {
    fn packet(&self) -> [u32; 7] {
        [
            0x24 << 24 | self.primitive.color.as_u32(),
            self.primitive.vertices[0].as_u32(),
            self.clut.as_u32() | self.tex_coords[0].as_u32(),
            self.primitive.vertices[1].as_u32(),
            self.tex_page.as_u32() | self.tex_coords[1].as_u32(),
            self.primitive.vertices[2].as_u32(),
            self.tex_coords[2].as_u32(),
        ]
    }
}

impl Packet<9> for TexturedPrimitive<4> {
    fn packet(&self) -> [u32; 9] {
        [
            0x2C << 24 | self.primitive.color.as_u32(),
            self.primitive.vertices[0].as_u32(),
            self.clut.as_u32() | self.tex_coords[0].as_u32(),
            self.primitive.vertices[1].as_u32(),
            self.tex_page.as_u32() | self.tex_coords[1].as_u32(),
            self.primitive.vertices[2].as_u32(),
            self.tex_coords[2].as_u32(),
            self.primitive.vertices[3].as_u32(),
            self.tex_coords[3].as_u32(),
        ]
    }
}
