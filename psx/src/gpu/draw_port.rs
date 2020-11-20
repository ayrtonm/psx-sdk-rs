use crate::gpu::color::{Color, Palette};
use crate::gpu::vertex::{Line, Pixel, Polygon, Quad, Triangle, Vertex};
use crate::gpu::DrawPort;
use crate::registers::Write;

impl DrawPort {
    const TERMINATION_CODE: u32 = 0x5555_5555;

    fn render_polygon<const CMD: u32, const N: usize>(
        &mut self, p: &Polygon<N>, c: &Color,
    ) -> &mut Self {
        self.write((CMD << 24) | u32::from(c));
        for vertex in p {
            self.write(vertex.into());
        }
        self
    }

    fn render_shaded<const CMD: u32, const N: usize>(
        &mut self, poly: &Polygon<N>, pal: &Palette<N>,
    ) -> &mut Self {
        self.write((CMD << 24) | u32::from(&pal[0]));
        self.write((&poly[0]).into());
        for i in 1..N {
            self.write((&pal[i]).into());
            self.write((&poly[i]).into());
        }
        self
    }

    // GPU Render Polygon Commands
    pub fn draw_triangle(&mut self, t: &Triangle, c: &Color) -> &mut Self {
        self.render_polygon::<0x20, 3>(t, c)
    }

    pub fn draw_triangle_transparent(&mut self, t: &Triangle, c: &Color) -> &mut Self {
        self.render_polygon::<0x22, 3>(t, c)
    }

    pub fn draw_quad(&mut self, q: &Quad, c: &Color) -> &mut Self {
        self.render_polygon::<0x28, 4>(q, c)
    }

    pub fn draw_quad_transparent(&mut self, q: &Quad, c: &Color) -> &mut Self {
        self.render_polygon::<0x2A, 4>(q, c)
    }

    pub fn draw_shaded_triangle(&mut self, t: &Triangle, pal: &Palette<3>) -> &mut Self {
        self.render_shaded::<0x30, 3>(t, pal)
    }

    pub fn draw_shaded_triangle_transparent(
        &mut self, t: &Triangle, pal: &Palette<3>,
    ) -> &mut Self {
        self.render_shaded::<0x32, 3>(t, pal)
    }

    pub fn draw_shaded_quad(&mut self, q: &Quad, pal: &Palette<4>) -> &mut Self {
        self.render_shaded::<0x38, 4>(q, pal)
    }

    pub fn draw_shaded_quad_transparent(&mut self, q: &Quad, pal: &Palette<4>) -> &mut Self {
        self.render_shaded::<0x3A, 4>(q, pal)
    }

    // GPU Render Line Commands
    pub fn draw_line(&mut self, l: &Line, c: &Color) -> &mut Self {
        self.render_polygon::<0x40, 2>(l, c)
    }

    pub fn draw_line_transparent(&mut self, l: &Line, c: &Color) -> &mut Self {
        self.render_polygon::<0x42, 2>(l, c)
    }

    pub fn draw_curve<const N: usize>(&mut self, p: &Polygon<N>, c: &Color) -> &mut Self {
        self.render_polygon::<0x48, N>(p, c);
        self.write(DrawPort::TERMINATION_CODE);
        self
    }

    pub fn draw_curve_transparent<const N: usize>(
        &mut self, p: &Polygon<N>, c: &Color,
    ) -> &mut Self {
        self.render_polygon::<0x4A, N>(p, c);
        self.write(DrawPort::TERMINATION_CODE);
        self
    }

    pub fn draw_shaded_line(&mut self, l: &Line, p: &Palette<2>) -> &mut Self {
        self.render_shaded::<0x50, 2>(l, p)
    }

    pub fn draw_shaded_line_transparent(&mut self, l: &Line, p: &Palette<2>) -> &mut Self {
        self.render_shaded::<0x52, 2>(l, p)
    }

    pub fn draw_shaded_curve<const N: usize>(
        &mut self, poly: &Polygon<N>, pal: &Palette<N>,
    ) -> &mut Self {
        self.render_shaded::<0x58, N>(poly, pal);
        self.write(DrawPort::TERMINATION_CODE);
        self
    }

    pub fn draw_shaded_curve_transparent<const N: usize>(
        &mut self, poly: &Polygon<N>, pal: &Palette<N>,
    ) -> &mut Self {
        self.render_shaded::<0x5A, N>(poly, pal);
        self.write(DrawPort::TERMINATION_CODE);
        self
    }

    // GPU Render Rectangle Commands
    pub fn draw_rect<T, U>(&mut self, offset: T, size: U, c: &Color) -> &mut Self
    where Vertex: From<T> + From<U> {
        self.write((0x60 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self.write(Vertex::from(size).into());
        self
    }

    pub fn draw_square<T>(&mut self, offset: T, size: Pixel, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x60 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self.write(Vertex::new(size, size).into());
        self
    }

    pub fn draw_rect_transparent<T, U>(&mut self, offset: T, size: U, c: &Color) -> &mut Self
    where Vertex: From<T> + From<U> {
        self.write((0x62 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self.write(Vertex::from(size).into());
        self
    }

    pub fn draw_square_transparent<T>(&mut self, offset: T, size: Pixel, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x62 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self.write(Vertex::new(size, size).into());
        self
    }

    pub fn draw_pixel<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x68 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    pub fn draw_pixel_transparent<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x6A << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    pub fn draw_small_rect<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x70 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    pub fn draw_small_rect_transparent<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x72 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    pub fn draw_medium_rect<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x78 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    pub fn draw_medium_rect_transparent<T>(&mut self, offset: T, c: &Color) -> &mut Self
    where Vertex: From<T> {
        self.write((0x7A << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self
    }

    // GPU Memory Transfer Commands
    pub fn fill_vram<T, U>(&mut self, offset: T, size: U, c: &Color) -> &mut Self
    where Vertex: From<T> + From<U> {
        self.write((0x02 << 24) | u32::from(c));
        self.write(Vertex::from(offset).into());
        self.write(Vertex::from(size).into());
        self
    }

    pub fn copy_vram<T, U, V>(&mut self, src: T, dest: U, size: V) -> &mut Self
    where Vertex: From<T> + From<U> + From<V> {
        self.write(0x80 << 24);
        self.write(Vertex::from(src).into());
        self.write(Vertex::from(dest).into());
        self.write(Vertex::from(size).into());
        self
    }

    pub fn rect_to_vram<T, U>(&mut self, dest: T, size: U, data: &[u32]) -> &mut Self
    where Vertex: From<T> + From<U> {
        self.write(0xA0 << 24);
        self.write(Vertex::from(dest).into());
        self.write(Vertex::from(size).into());
        for &d in data {
            self.write(d);
        }
        self
    }

    // GPU Rendering Attributes
    pub fn start<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE3 << 24 | v.x() as u32 | ((v.y() as u32) << 10));
        self
    }

    pub fn end<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE4 << 24 | v.x() as u32 | ((v.y() as u32) << 10));
        self
    }

    pub fn offset<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE5 << 24 | v.x() as u32 | ((v.y() as u32) << 11));
        self
    }
}
