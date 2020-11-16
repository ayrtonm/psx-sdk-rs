use crate::gpu::vertex::Component;
use crate::gpu::res::{Depth, DmaSource, Hres, Vmode, Vres};
use crate::gpu::DisplayEnv;

impl DisplayEnv {
    // Calls DisplayEnv(00h)
    pub fn reset_gpu(&mut self) {
        self.write(0);
    }

    // Calls DisplayEnv(01h)
    pub fn reset_buffer(&mut self) {
        self.write(1);
    }

    // Calls DisplayEnv(03h)
    pub fn on(&mut self) {
        self.write(0x0300_0000);
    }

    // Calls DisplayEnv(03h)
    pub fn off(&mut self) {
        self.write(0x0300_0001);
    }

    // Calls DisplayEnv(04h)
    pub fn dma(&mut self, dir: DmaSource) {
        let source = match dir {
            DmaSource::Off => 0,
            DmaSource::FIFO => 1,
            DmaSource::CPU => 2,
            DmaSource::GPU => 3,
        };
        self.write((0x04 << 24) | source);
    }

    // Calls DisplayEnv(05h)
    pub fn start(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0x05, 10, 9, 10>(x, y);
    }

    // Calls DisplayEnv(06h)
    pub fn horizontal(&mut self, x1: Component, x2: Component) {
        self.generic_cmd::<0x06, 12, 12, 12>(x1, x2);
    }

    // Calls DisplayEnv(07h)
    pub fn vertical(&mut self, y1: Component, y2: Component) {
        self.generic_cmd::<0x07, 10, 10, 10>(y1, y2);
    }

    // Calls DisplayEnv(08h)
    pub fn mode(&mut self, hres: &Hres, vres: &Vres, vmode: Vmode, depth: Depth, interlace: bool) {
        let cmd = 0x08 << 24;
        let hres = match hres {
            Hres::H256 => 0,
            Hres::H320 => 1,
            Hres::H512 => 2,
            Hres::H640 => 3,
            Hres::H368 => 1 << 6,
        };
        // Should this only equal 1 << 2 when interlace is true or is that
        // restriction handled in hardware?
        let vres = match vres {
            Vres::V240 => 0,
            Vres::V480 => 1 << 2,
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
    }

    fn generic_cmd<const CMD: u8, const XMASK: Component, const YMASK: Component, const SHIFT: Component>(
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
}
