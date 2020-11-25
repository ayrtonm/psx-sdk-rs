use crate::gpu::vertex::{Pixel, Vertex};
use crate::gpu::{Depth, DispPort, DmaSource, Vmode};
use crate::registers::Write;

impl DispPort {
    // Calls DispPort(00h)
    pub fn reset_gpu(&mut self) -> &mut Self {
        self.write(0);
        self
    }

    // Calls DispPort(01h)
    pub fn reset_buffer(&mut self) -> &mut Self {
        self.write(1 << 24);
        self
    }

    // Calls DispPort(03h)
    pub fn on(&mut self) -> &mut Self {
        self.write(0x0300_0000);
        self
    }

    // Calls DispPort(03h)
    pub fn off(&mut self) -> &mut Self {
        self.write(0x0300_0001);
        self
    }

    // Calls DispPort(04h)
    pub fn dma(&mut self, dir: DmaSource) -> &mut Self {
        let source = match dir {
            DmaSource::Off => 0,
            DmaSource::FIFO => 1,
            DmaSource::CPU => 2,
            DmaSource::GPU => 3,
        };
        self.write((0x04 << 24) | source);
        self
    }

    // Calls DispPort(05h)
    pub fn start<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.generic_cmd::<0x05, 10, 9, 10>(v.x(), v.y())
    }

    // Calls DispPort(06h)
    pub fn horizontal(&mut self, x1: Pixel, x2: Pixel) -> &mut Self {
        self.generic_cmd::<0x06, 12, 12, 12>(x1, x2)
    }

    // Calls DispPort(07h)
    pub fn vertical(&mut self, y1: Pixel, y2: Pixel) -> &mut Self {
        self.generic_cmd::<0x07, 10, 10, 10>(y1, y2)
    }

    // Calls DispPort(08h)
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
        self.write(cmd | hres | vres | vmode | depth | interlace);
        self
    }

    fn generic_cmd<const CMD: u8, const XMASK: Pixel, const YMASK: Pixel, const SHIFT: Pixel>(
        &mut self, mut x: Pixel, mut y: Pixel,
    ) -> &mut Self {
        if cfg!(debug_assertions) {
            x &= (1 << XMASK) - 1;
            y &= (1 << YMASK) - 1;
        }
        let cmd = (CMD as u32) << 24;
        let x = x as u32;
        let y = (y as u32) << (SHIFT as u32);
        self.write(cmd | x | y);
        self
    }
}
