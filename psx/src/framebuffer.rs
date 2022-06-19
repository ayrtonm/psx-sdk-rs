use crate::gpu::colors::WHITE;
use crate::gpu::primitives::Sprt8;
use crate::gpu::{Clut, Color, DMAMode, Depth, DispEnv, DrawEnv, Packet, TexColor, TexCoord,
                 TexPage, Vertex, VertexError, VideoMode, GPU_BUFFER_SIZE};
use crate::hw::gpu::{GP0Command, GP0, GP1};
use crate::hw::{gpu, irq, Register};
use crate::irq::IRQ;
use crate::format::tim::TIM;
use crate::{dma, include_words};
use core::fmt;
use core::mem::size_of;

fn draw_sync() {
    let mut gpu_stat = gpu::Status::new();
    while !gpu_stat.cmd_ready() || !gpu_stat.dma_ready() {
        gpu_stat.load();
    }
}

/// A double-buffered framebuffer configuration
///
/// Maintains the framebuffer's configuration and state. Also provides acess to
/// the GPU registers `GP0`, `GP1` and `GPU_STATUS`.
pub struct Framebuffer {
    /// The write-only GPU I/O port for GP0 commands and packets
    pub gp0: GP0,
    /// The write-only GPU I/O port for GP1 commands
    pub gp1: GP1,
    /// The read-only GPU status register
    pub gpu_status: gpu::Status,
    /// The IRQ status register
    pub irq_mask: irq::Mask,
    /// The IRQ mask register
    pub irq_status: irq::Status,
    disp_envs: [DispEnv; 2],
    draw_envs: [Packet<DrawEnv>; 2],
    swapped: bool,
}

impl Default for Framebuffer {
    fn default() -> Self {
        // SAFETY: The framebuffer parameters are valid.
        unsafe { Self::new((0, 0), (0, 240), (320, 240), None).unwrap_unchecked() }
    }
}

impl Framebuffer {
    /// Creates a new framebuffer.
    ///
    /// Places one buffer at `buf0` and the other at `buf1` and uses the
    /// specified resolution and background color (or black if `bg_color` is
    /// `None`). Also resets the GPU, enables DMA to GP0 on the GPU-side and
    /// enables the display.
    pub fn new(
        buf0: (i16, i16), buf1: (i16, i16), res: (i16, i16), bg_color: Option<Color>,
    ) -> Result<Self, VertexError> {
        let mut fb = Framebuffer {
            // These registers are read-only
            gp0: GP0::skip_load(),
            gp1: GP1::skip_load(),
            gpu_status: gpu::Status::new(),
            // wait_vblank will reload this anyway
            irq_status: irq::Status::skip_load(),
            irq_mask: irq::Mask::new(),
            disp_envs: [DispEnv::new(buf0, res)?, DispEnv::new(buf1, res)?],
            draw_envs: [
                Packet::new(DrawEnv::new(buf1, res, bg_color)?),
                Packet::new(DrawEnv::new(buf0, res, bg_color)?),
            ],
            swapped: false,
        };
        GP1::skip_load()
            .reset_gpu()
            .dma_mode(Some(DMAMode::GP0))
            .display_mode(res, VideoMode::NTSC, Depth::Bits15, false)?
            .enable_display(true);
        fb.irq_mask.enable_irq(IRQ::Vblank).store();
        //fb.wait_vblank();
        //fb.swap();
        Ok(fb)
    }

    /// Changes the framebuffer's background color.
    pub fn set_bg_color(&mut self, color: Color) {
        for packet_env in &mut self.draw_envs {
            packet_env.contents.bg_color = color;
        }
    }

    /// Swaps the framebuffers using only GPU I/O ports.
    pub fn swap(&mut self) {
        self.swapped = !self.swapped;
        let idx = self.swapped as usize;
        self.gp1.set_display_env(&self.disp_envs[idx]);
        self.gp0.send_command(&self.draw_envs[idx].contents);
    }

    /// Swaps the framebuffers using GPU I/O ports and the DMA channel
    pub fn dma_swap(&mut self, gpu_dma: &mut dma::GPU) {
        self.swapped = !self.swapped;
        let idx = self.swapped as usize;
        self.gp1.set_display_env(&self.disp_envs[idx]);
        gpu_dma.send_list(&self.draw_envs[idx]);
    }

    /// Loads a `TIM` file into VRAM.
    ///
    /// After loading a TIM into VRAM, the copy in memory isn't necessary so the
    /// lifetimes of the `TIM` and `LoadedTIM` are completely disconnected.
    pub fn load_tim(&mut self, tim: TIM) -> LoadedTIM {
        // Used to avoid implementing GP0Command for any &[u32]
        // TIM::new ensures that the bitmap data is a valid GP0 command
        struct CopyToVRAM<'a>(&'a [u32]);

        impl GP0Command for CopyToVRAM<'_> {
            fn data(&self) -> &[u32] {
                self.0
            }
        }

        self.draw_sync();
        self.gp0.send_command(&CopyToVRAM(tim.bmp.data));
        if let Some(clut) = &tim.clut_bmp {
            self.draw_sync();
            self.gp0.send_command(&CopyToVRAM(clut.data));
        };

        let clut = tim.clut_bmp.map(|clut| clut.offset);
        LoadedTIM {
            tex_page: tim.bmp.offset,
            clut,
        }
    }

    /// Loads the default font TIM into VRAM.
    ///
    /// This returns a `LoadedTIM` which can then be used to create `TextBox`s
    /// using `LoadedTIM::new_text_box`. Note that `LoadedTIM` does not track
    /// lifetimes so it's the user's responsibility to ensure that the font
    /// remains in VRAM while it's needed.
    pub fn load_default_font(&mut self) -> LoadedTIM {
        let tim = include_words!("../font.tim");

        // SAFETY: The default font TIM contains valid data.
        let font = unsafe { TIM::new(tim).unwrap_unchecked() };
        self.load_tim(font)
    }

    /// Spins until the GPU is ready to draw.
    pub fn draw_sync(&mut self) {
        self.gpu_status.load();
        while !self.gpu_status.cmd_ready() || !self.gpu_status.dma_ready() {
            self.gpu_status.load();
        }
    }

    /// Spins until vblank.
    pub fn wait_vblank(&mut self) {
        self.irq_status.ack(IRQ::Vblank).store().wait(IRQ::Vblank);
    }
}

impl fmt::Write for TextBox {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        for c in msg.chars() {
            if c.is_ascii() {
                self.print_char(c as u8);
            } else {
                // Print '?' for non-ascii UTF-8
                self.print_char(b'?');
            }
        }
        Ok(())
    }
}

/// The properties of a TIM file that has been loaded into VRAM.
///
/// This does not track lifetimes, so it's the user's responsibility to ensure
/// that the TIM remains in VRAM while it's needed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoadedTIM {
    /// The loaded TIM's texture page attribute.
    pub tex_page: TexPage,
    /// The loaded TIM's color loookup table attribute.
    pub clut: Option<Clut>,
}

// Up to 5 `Sprt8`s fit in the GPU buffer at one time.
const TEXT_BOX_BUFFER: usize = GPU_BUFFER_SIZE / size_of::<Sprt8>();

/// A text box configuration and in-memory buffer.
pub struct TextBox {
    color: TexColor,
    initial: Vertex,
    cursor: Vertex,
    size: Vertex,
    idx: usize,
    buffer: [Sprt8; TEXT_BOX_BUFFER],
}

impl LoadedTIM {
    /// Creates a new text box using the loaded TIM as the font.
    pub fn new_text_box(&self, offset: (i16, i16), size: (i16, i16)) -> TextBox {
        let offset = Vertex::new(offset);
        let size = Vertex::new(size);
        let color = TexColor::from(WHITE);
        let mut buffer = [Sprt8::new(); TEXT_BOX_BUFFER];
        for letter in &mut buffer {
            if let Some(clut) = self.clut {
                letter.set_clut(clut);
            }
            letter.set_color(color);
        }
        TextBox {
            color,
            initial: offset,
            cursor: offset,
            size,
            idx: 0,
            buffer,
        }
    }
}

const FONT_SIZE: u8 = 8;

impl TextBox {
    /// Move the cursor to the beginning of the next line.
    pub fn newline(&mut self) {
        self.cursor = Vertex(self.initial.0, self.cursor.1 + FONT_SIZE as i16);
    }
    /// Move the cursor to its initial position.
    pub fn reset(&mut self) {
        self.cursor = self.initial;
    }
    /// Change the font color.
    pub fn change_color(&mut self, color: Color) {
        let color = TexColor::from(color);
        if color != self.color {
            self.color = color;
            for letter in &mut self.buffer {
                letter.set_color(color);
            }
        }
    }
    /// Prints a single character.
    pub fn print_char(&mut self, ascii: u8) {
        if ascii == b'\n' {
            self.newline();
            self.cursor.0 = self.initial.0;
        } else {
            let ascii_per_row = 128 / FONT_SIZE;
            // The default font omits the first 32 characters to save on VRAM. These
            // characters are printed as '?'
            let ascii = if ascii < (2 * ascii_per_row) {
                b'?'
            } else {
                ascii - (2 * ascii_per_row)
            };
            let x = (ascii % ascii_per_row) * FONT_SIZE;
            let y = (ascii / ascii_per_row) * FONT_SIZE;
            if self.idx == 0 {
                draw_sync();
            }
            self.buffer[self.idx]
                .set_offset(self.cursor)
                .set_tex_coord(TexCoord { x, y });
            GP0::skip_load().send_command(&self.buffer[self.idx]);
            self.idx += 1;
            if self.idx == TEXT_BOX_BUFFER {
                self.idx = 0;
            }
            self.cursor.0 += FONT_SIZE as i16;
            if self.cursor.0 == self.initial.0 + self.size.0 {
                self.newline();
                self.cursor.0 = self.initial.0;
            }
            if self.cursor.1 == self.initial.1 + self.size.1 {
                self.cursor = self.initial;
            }
        }
    }
}

/// Print a rust-style format string and args using the `&mut TextBox` specified
/// by `$box`.
#[macro_export]
macro_rules! dprint {
    ($box:expr, $($args:tt)*) => {
        {
            use core::fmt::Write;
            $box.write_fmt(format_args!($($args)*)).ok()
        }
    };
}

/// Print a rust-style format string and args using the `&mut TextBox` specified
/// by `$box`.
#[macro_export]
macro_rules! dprintln {
    ($box:expr, $($args:tt)*) => {
        $crate::dprint!($box, $($args)*);
        $box.print_char(b'\n');
    };
}
