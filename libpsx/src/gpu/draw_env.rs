use crate::gpu::color::{Color, Palette};
use crate::gpu::vertex::{Component, Line, PolyLine, Quad, Triangle, Vertex};
use crate::gpu::DrawEnv;
use crate::macros::RegisterWrite;

type ShadedPolyLine<'a, 'b, 'c> = &'a mut dyn Iterator<Item = (&'a Color, &'b Vertex)>;

impl DrawEnv {
    const TERMINATION_CODE: u32 = 0x5555_5555;

    fn serialize((x, y): (u16, u16)) -> u32 {
        (x as u32) | (y as u32) << 16
    }

    pub fn copy_rect(&mut self, src: (u16, u16), dst: (u16, u16), size: (u16, u16)) {
        let cmd = 0xC0 << 24;
        let cmd_params = [
            cmd,
            DrawEnv::serialize(src),
            DrawEnv::serialize(dst),
            DrawEnv::serialize(size),
        ];
        self.write_slice(&cmd_params);
    }

    pub fn rect_to_vram(&mut self, dest: (u16, u16), size: (u16, u16), data: &[u32]) {
        let cmd = 0xA0 << 24;
        let cmd_params = [cmd, DrawEnv::serialize(dest), DrawEnv::serialize(size)];
        self.write_slice(&cmd_params);
        self.write_slice(data);
    }

    // Calls DrawEnv(E3h)
    pub fn start(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE3, 10, 9, 10>(x, y)
    }

    // Calls DrawEnv(E4h)
    pub fn end(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE4, 10, 9, 10>(x, y)
    }

    // Calls DrawEnv(E5h)
    pub fn offset(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE5, 11, 11, 11>(x, y)
    }

    fn generic_cmd<
        const CMD: u8,
        const XMASK: Component,
        const YMASK: Component,
        const SHIFT: Component,
    >(
        &mut self, mut x: Component, mut y: Component,
    ) {
        if cfg!(debug_assertions) {
            x &= (1 << XMASK) - 1;
            y &= (1 << YMASK) - 1;
        }
        let cmd = (CMD as u32) << 24;
        let x = x as u32;
        let y = (y as u32) << (SHIFT as u32);
        self.write(cmd | x | y);
    }

    pub fn draw_triangle(&mut self, p: &Triangle, c: &Color) {
        self.draw::<0x20>(p, c);
    }

    pub fn draw_triangle_transparent(&mut self, p: &Triangle, c: &Color) {
        self.draw::<0x22>(p, c);
    }

    pub fn draw_quad(&mut self, p: &Quad, c: &Color) {
        self.draw::<0x28>(p, c);
    }

    pub fn draw_quad_transparent(&mut self, p: &Quad, c: &Color) {
        self.draw::<0x2A>(p, c);
    }

    pub fn draw_shaded_triangle(&mut self, p: &Triangle, c: Palette<3>) {
        self.draw_shaded::<0x30>(&mut c.iter().zip(p.iter()));
    }

    pub fn draw_shaded_triangle_transparent(&mut self, p: &Triangle, c: Palette<3>) {
        self.draw_shaded::<0x32>(&mut c.iter().zip(p.iter()));
    }

    pub fn draw_shaded_quad(&mut self, p: &Quad, c: Palette<4>) {
        self.draw_shaded::<0x38>(&mut c.iter().zip(p.iter()));
    }

    pub fn draw_shaded_quad_transparent(&mut self, p: &Quad, c: Palette<4>) {
        self.draw_shaded::<0x3A>(&mut c.iter().zip(p.iter()));
    }

    pub fn draw_line(&mut self, l: Line, c: &Color) {
        self.draw::<0x40>(&l[..], c);
    }

    pub fn draw_line_transparent(&mut self, l: Line, c: &Color) {
        self.draw::<0x42>(&l[..], c);
    }

    pub fn draw_polyline(&mut self, l: PolyLine, c: &Color) {
        self.draw::<0x48>(l, c);
        self.write(DrawEnv::TERMINATION_CODE);
    }

    pub fn draw_polyline_transparent(&mut self, l: PolyLine, c: &Color) {
        self.draw::<0x4A>(l, c);
        self.write(DrawEnv::TERMINATION_CODE);
    }

    pub fn draw_shaded_line(&mut self, l: Line, c: Palette<2>) {
        self.draw_shaded::<0x50>(&mut c.iter().zip(l.iter()));
    }

    pub fn draw_shaded_line_transparent(&mut self, l: Line, c: Palette<2>) {
        self.draw_shaded::<0x52>(&mut c.iter().zip(l.iter()));
    }

    pub fn draw_shaded_polyline(&mut self, l: ShadedPolyLine) {
        self.draw_shaded::<0x58>(l);
        self.write(DrawEnv::TERMINATION_CODE);
    }

    pub fn draw_shaded_polyline_transparent(&mut self, l: ShadedPolyLine) {
        self.draw_shaded::<0x5A>(l);
        self.write(DrawEnv::TERMINATION_CODE);
    }

    pub fn draw_rect(&mut self, offset: &Vertex, w: Component, h: Component, c: &Color) {
        self.write((0x60 << 24) | u32::from(c));
        self.write(offset.into());
        self.write((h as u32) << 16 | (w as u32));
    }

    // TODO: `tex` in the fn sig is temporary, come up with something sensible
    pub fn draw_rect_textured(&mut self, offset: &Vertex, w: Component, h: Component, tex: u32) {
        self.write(0x65 << 24);
        self.write(offset.into());
        self.write(tex);
        self.write((h as u32) << 16 | (w as u32));
    }

    pub fn draw_square(&mut self, offset: &Vertex, l: Component, c: &Color) {
        self.draw_rect(offset, l, l, c)
    }

    fn draw<const CMD: u32>(&mut self, l: PolyLine, c: &Color) {
        self.write((CMD << 24) | u32::from(c));
        for v in l {
            self.write(v.into());
        }
    }

    fn draw_shaded<const CMD: u32>(&mut self, l: ShadedPolyLine) {
        let mut iter = l.map(|(c, v)| (c.into(), v.into()));
        if let Some((c, v)) = iter.next() {
            self.write((CMD << 24) | c);
            self.write(v);
        }
        for (c, v) in iter {
            self.write(c);
            self.write(v);
        }
    }
}
