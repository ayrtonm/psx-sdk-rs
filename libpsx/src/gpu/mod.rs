use core::marker::PhantomData;
use crate::bios;
use crate::constrain;
use crate::util::zero_extend;

pub mod color;
pub mod position;
pub mod polygon;
pub mod line;
pub mod vram;

// TODO: Think about whether it makes sense to keep a copy of the GPUSTAT settings in a field
// or whether this should only provide an interface to read/write the GPU settings
// TODO: Think about whether this should be must_use or not
//#[must_use]
#[derive(Clone, Copy)]
pub struct Ctxt<S: Screen> {
    display: PhantomData<S>,
}

#[derive(Clone, Copy)]
pub struct Disabled;
#[derive(Clone, Copy)]
pub struct Enabled;

pub trait Screen {
    type Not: Screen;
    const CMD: u32;
    fn toggle() -> PhantomData::<Self::Not> {
        bios::gpu_gp1_command_word(Self::CMD);
        PhantomData::<Self::Not>
    }
}
impl Screen for Disabled {
    type Not = Enabled;
    const CMD: u32 = 0x0300_0000;
}
impl Screen for Enabled {
    type Not = Disabled;
    const CMD: u32 = 0x0300_0001;
}

impl<S: Screen> Ctxt<S> {
    pub fn toggle_display(self) -> Ctxt<S::Not> {
        Ctxt::<S::Not> {
            display: S::toggle(),
        }
    }
    pub fn reset_gpu(self) -> Ctxt<Disabled> {
        bios::gpu_gp1_command_word(0x0000_0000);
        Ctxt::new()
    }
    pub fn reset_buffer(self) -> Self {
        bios::gpu_gp1_command_word(0x0100_0000);
        self
    }
    fn xy_cmd<const CMD: usize, const XMASK: u16, const YMASK: u16, const YSHIFT: u32>(self, mut x: u16, mut y: u16) -> Self {
        constrain!(CMD < 0x100);
        let cmd = (CMD as u32) << 24;
        x &= (1 << XMASK) - 1;
        y &= (1 << YMASK) - 1;
        bios::gpu_command_word(cmd | x as u32 | (y as u32) << YSHIFT);
        self
    }
    fn xy_gp1_cmd<const CMD: usize, const XMASK: u16, const YMASK: u16, const YSHIFT: u32>(self, mut x: u16, mut y: u16) -> Self {
        constrain!(CMD < 0x100);
        let cmd = (CMD as u32) << 24;
        x &= (1 << XMASK) - 1;
        y &= (1 << YMASK) - 1;
        bios::gpu_gp1_command_word(cmd | x as u32 | (y as u32) << YSHIFT);
        self
    }
    // Calls GP0(E3h)
    pub fn draw_start(self, x: u16, y: u16) -> Self {
        self.xy_cmd::<0xe3, 10, 9, 10>(x, y)
    }
    // Calls GP0(E4h)
    pub fn draw_end(self, x: u16, y: u16) -> Self {
        self.xy_cmd::<0xe4, 10, 9, 10>(x, y)
    }
    // Calls GP0(E5h)
    pub fn draw_offset(self, x: u16, y: u16) -> Self {
        self.xy_cmd::<0xe5, 11, 11, 11>(x, y)
    }
    // Calls GP1(05h)
    pub fn display_start(self, mut x: u16, mut y: u16) -> Self {
        self.xy_gp1_cmd::<0x05, 10, 9, 10>(x, y)
    }
}

impl Ctxt<Disabled> {
    pub fn new() -> Self {
        //const DEFAULT_GPUSTAT: u32 = 0x1480_2000;
        Ctxt::<Disabled> {
            display: PhantomData::<Disabled>,
        }
    }
    pub fn display_on(self) -> Ctxt<Enabled> {
        self.toggle_display()
    }
}

impl Ctxt<Enabled> {
    pub fn display_off(self) -> Ctxt<Disabled> {
        self.toggle_display()
    }
}
