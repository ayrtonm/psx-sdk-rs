use super::vertex::{Pixel, Vertex};
use super::{Depth, Vmode};
use crate::mmio::gpu;
use crate::mmio::register::Write;

impl gpu::GP1 {
    pub fn on(&mut self) -> &mut Self {
        unsafe { self.write(0x0300_0000) };
        self
    }

    pub fn dma_direction(&mut self, dest: u32) -> &mut Self {
        unsafe { self.write(0x04 << 24 | dest) };
        self
    }

    pub fn start<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(offset);
        if cfg!(debug_assertions) {
            if v.x() > (1 << 10) - 1 || v.y() > (1 << 9) - 1 {
                panic!("");
            }
        }
        unsafe { self.write(0x05 << 24 | v.x() as u32 | ((v.y() as u32) << 10)) };
        self
    }

    pub fn horizontal(&mut self, x0: Pixel, x1: Pixel) -> &mut Self {
        if cfg!(debug_assertions) {
            if x0 > (1 << 12) - 1 || x1 > (1 << 12) - 1 {
                panic!("");
            }
        }
        unsafe { self.write(0x06 << 24 | x0 as u32 | ((x1 as u32) << 12)) };
        self
    }

    pub fn vertical(&mut self, y0: Pixel, y1: Pixel) -> &mut Self {
        if cfg!(debug_assertions) {
            if y0 > (1 << 10) - 1 || y1 > (1 << 10) - 1 {
                panic!("");
            }
        }
        unsafe { self.write(0x07 << 24 | y0 as u32 | ((y1 as u32) << 10)) };
        self
    }

    pub fn mode(
        &mut self, hres: Pixel, vres: Pixel, vmode: Vmode, depth: Depth, interlace: bool,
    ) -> &mut Self {
        let cmd = 0x08 << 24;
        let hres = match hres {
            256 => 0,
            320 => 1,
            512 => 2,
            640 => 3,
            368 => 1 << 6,
            _ => unreachable!("Invalid hres"),
        };
        // Should this only equal 1 << 2 when interlace is true or is that
        // restriction handled in hardware?
        let vres = match vres {
            240 => 0,
            480 => 1 << 2,
            _ => unreachable!("Invalid vres"),
        };
        let vmode = match vmode {
            Vmode::NTSC => 0,
            Vmode::PAL => 1 << 3,
        };
        let depth = match depth {
            Depth::Lo => 0,
            Depth::Hi => 1 << 4,
        };
        let interlace = if interlace { 1 << 5 } else { 0 };
        unsafe { self.write(cmd | hres | vres | vmode | depth | interlace) };
        self
    }
}
