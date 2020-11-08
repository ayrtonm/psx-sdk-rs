use crate::bios;
use crate::constrain;

pub mod color;
pub mod position;
pub mod polygon;
pub mod line;
pub mod vram;
pub mod framebuffer;

pub struct Ctxt {
    pub draw_env: DrawEnv,
    pub display_env: DisplayEnv,
}

pub struct DrawEnv;
pub struct DisplayEnv;

pub struct Res {
    pub h: Hres,
    pub v: Vres,
}
pub enum Hres { H256, H320, H368, H512, H640 }
pub enum Vres { V240, V480 }
pub enum Vmode { NTSC, PAL }
pub enum Depth { Lo, Hi }

impl From<Hres> for u32 {
    fn from(h: Hres) -> u32 {
        match h {
            Hres::H256 => 256,
            Hres::H320 => 320,
            Hres::H368 => 368,
            Hres::H512 => 512,
            Hres::H640 => 640,
        }
    }
}

impl From<Vres> for u32 {
    fn from(v: Vres) -> u32 {
        match v {
            Vres::V240 => 240,
            Vres::V480 => 480,
        }
    }
}

impl Ctxt {
    pub fn new() -> Self {
        let draw_env = DrawEnv::new();
        let display_env = DisplayEnv::new();
        Ctxt { draw_env, display_env }
    }
    // Calls GP1(00h)
    pub fn reset_gpu(&self) -> &Self {
        bios::gpu_gp1_command_word(0x0000_0000);
        self
    }
    // Calls GP1(01h)
    pub fn reset_buffer(&self) -> &Self {
        bios::gpu_gp1_command_word(0x0100_0000);
        self
    }
}

impl DisplayEnv {
    pub fn new() -> Self {
        DisplayEnv { }
    }
    // Calls GP1(03h)
    pub fn on(&self) -> &Self {
        bios::gpu_gp1_command_word(0x0300_0000);
        self
    }
    pub fn off(&self) -> &Self {
        bios::gpu_gp1_command_word(0x0300_0001);
        self
    }
    // Calls GP1(05h)
    pub fn start(&self, x: u32, y: u32) -> &Self {
        self.gpu_gp1_cmd::<0x05, 10, 9, 10>(x, y)
    }
    // Calls GP1(06h)
    pub fn horizontal(&self, x1: u32, x2: u32) -> &Self {
        self.gpu_gp1_cmd::<0x06, 12, 12, 12>(x1, x2)
    }
    // Calls GP1(07h)
    pub fn vertical(&self, y1: u32, y2: u32) -> &Self {
        self.gpu_gp1_cmd::<0x07, 10, 10, 10>(y1, y2)
    }
    // Calls GP1(08h)
    pub fn mode(&self, hres: Hres, vres: Vres, vmode: Vmode, depth: Depth, interlace: bool) -> &Self {
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
        let interlace = if interlace {
            1 << 5
        } else {
            0
        };
        bios::gpu_gp1_command_word(cmd | hres | vres | vmode | depth | interlace);
        self
    }
    fn gpu_gp1_cmd<const CMD: usize, const XMASK: u32, const YMASK: u32, const YSHIFT: u32>(&self, mut x: u32, mut y: u32) -> &Self {
        constrain!(CMD < 0x100);
        let cmd = (CMD as u32) << 24;
        x &= (1 << XMASK) - 1;
        y &= (1 << YMASK) - 1;
        bios::gpu_gp1_command_word(cmd | x | y << YSHIFT);
        self
    }
}

impl DrawEnv {
    pub fn new() -> Self {
        DrawEnv { }
    }
    // Calls GP0(E3h)
    pub fn start(&self, x: u32, y: u32) -> &Self {
        self.gpu_cmd::<0xe3, 10, 9, 10>(x, y)
    }
    // Calls GP0(E4h)
    pub fn end(&self, x: u32, y: u32) -> &Self {
        self.gpu_cmd::<0xe4, 10, 9, 10>(x, y)
    }
    // Calls GP0(E5h)
    pub fn offset(&self, x: u32, y: u32) -> &Self {
        self.gpu_cmd::<0xe5, 11, 11, 11>(x, y)
    }
    fn gpu_cmd<const CMD: usize, const XMASK: u32, const YMASK: u32, const YSHIFT: u32>(&self, mut x: u32, mut y: u32) -> &Self {
        constrain!(CMD < 0x100);
        let cmd = (CMD as u32) << 24;
        x &= (1 << XMASK) - 1;
        y &= (1 << YMASK) - 1;
        bios::gpu_command_word(cmd | x | y << YSHIFT);
        self
    }
}
