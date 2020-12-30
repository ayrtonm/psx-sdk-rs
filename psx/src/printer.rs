#![allow(dead_code)]
use core::mem::size_of;

use crate::dma;
use crate::dma::{BaseAddress, BlockControl, TransferMode};
use crate::gpu::{Clut, Color, TexPage, Vertex};
use crate::graphics::buffer::Buffer;
use crate::graphics::ot::OT;
use crate::graphics::packet::Packet;
use crate::graphics::primitive::Sprt8;
use crate::tim::TIM;
use crate::value::LoadMut;
use crate::workarounds::UnwrapUnchecked;

/// A font stored in the framebuffer
pub struct Font {
    tpage: TexPage,
    clut: Option<Clut>,
}

/// Screen printer with an `N` word primitive buffer for character sprites.
pub struct Printer<const N: usize> {
    font: Option<Font>,
    buffer: Buffer<N>,
    ot: OT<1>,
    cursor: Vertex,
    //font_size: FontSize,
    box_offset: Vertex,
    box_size: Vertex,
    color: Color,
}

/// The minimum buffer size for a printer.
pub const MIN_SIZE: usize = size_of::<Packet<Sprt8>>() / 4;

impl Printer<MIN_SIZE> {
    /// Creates a new printer with the minimum buffer size.
    pub fn new<T, U, V>(cursor: T, box_offset: U, box_size: V, color: Option<Color>) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        Printer::with_buffer(cursor, box_offset, box_size, color)
    }
}
impl<const N: usize> Printer<N> {
    /// Creates a new printer.
    pub fn with_buffer<T, U, V>(
        cursor: T, box_offset: U, box_size: V, color: Option<Color>,
    ) -> Self
    where Vertex: From<T> + From<U> + From<V> {
        Printer {
            font: None,
            buffer: Buffer::new(),
            ot: OT::default(),
            cursor: cursor.into(),
            box_offset: box_offset.into(),
            box_size: box_size.into(),
            color: color.unwrap_or(Color::WHITE),
        }
    }

    /// Loads the default font into VRAM.
    pub fn load_font(&mut self, gpu_dma: &mut dma::gpu::CHCR) {
        let current = gpu_dma.load_mut();
        let old_transfer_mode = current.value.transfer_mode();
        current.transfer_mode(TransferMode::Immediate).store();

        // Use a zipped font to save ~2 KB
        let mut font = unzip!("../font.tim.zip");
        let tim = TIM::new(&mut font);
        let font = Font {
            tpage: tim.tex_page(),
            clut: tim.clut(),
        };

        let bmp = tim.bitmap.data();
        dma::gpu::MADR.set(bmp.first().unwrap_unchecked());
        dma::gpu::BCR.set(bmp.len());
        gpu_dma.load_mut().start(()).wait();

        tim.clut_bitmap.map(|clut_bitmap| {
            let clut = clut_bitmap.data();
            dma::gpu::MADR.set(clut.first().unwrap_unchecked());
            dma::gpu::BCR.set(clut.len());
            gpu_dma.load_mut().start(()).wait();
        });

        old_transfer_mode.map(|old_transfer_mode| {
            gpu_dma.load_mut().transfer_mode(old_transfer_mode).store();
        });

        self.font = Some(font);
    }

    /// Moves the cursor to the initial position on the next line.
    pub fn newline(&mut self) {
        self.cursor = (0, self.cursor.y + 8).into();
    }

    /// Resets the cursor's position to the box's offset.
    pub fn reset(&mut self) {
        self.cursor = self.box_offset;
    }

    fn print_char(&mut self, ascii: u8, gpu_dma: &mut dma::gpu::CHCR) {
        let (w, h) = (8, 8);
        let ascii_per_row = 128 / w;
        // Offset ascii values to work with font subset stored in VRAM.
        let ascii = ascii - (2 * ascii_per_row);
        let xoffset = (ascii % ascii_per_row) * w;
        let yoffset = (ascii / ascii_per_row) * h;
        let mut letter = match self.buffer.sprt8() {
            Some(sprt) => sprt,
            None => {
                gpu_dma.send_list(&self.ot).wait();
                self.ot.empty();
                self.buffer.empty().sprt8().unwrap_unchecked()
            },
        };
        letter
            .set_color(self.color)
            .set_offset(self.cursor.shift(self.box_offset))
            .set_tex_coord((xoffset, yoffset))
            .set_clut(self.font.as_ref().map(|f| f.clut).flatten());
        self.ot.insert(&mut letter, 0);
        if self.cursor.x + 8 >= self.box_offset.x + self.box_size.x {
            self.newline();
        } else {
            self.cursor = self.cursor.shift((8, 0));
        };
    }

    /// Prints a message with the given formatted arguments.
    pub fn print<'a, M, const A: usize>(
        &mut self, msg: M, args: [u32; A], gpu_dma: &mut dma::gpu::CHCR,
    ) where M: IntoIterator<Item = &'a u8> {
        let mut fmt_arg = false;
        let mut leading_zeros = false;
        let mut args = args.iter();
        for &ascii in msg {
            match ascii {
                b'\n' => self.newline(),
                b'\0' => break,
                b'0' if fmt_arg => leading_zeros = true,
                b'{' if !fmt_arg => fmt_arg = true,
                b'{' if fmt_arg => {
                    fmt_arg = false;
                    self.print_char(b'{', gpu_dma);
                },
                b'}' if fmt_arg => {
                    fmt_arg = false;
                    let arg = args.next().unwrap_unchecked();
                    let formatted = Self::format_u32(*arg, leading_zeros);
                    leading_zeros = false;
                    for &c in &formatted {
                        if c != b'\0' {
                            self.print_char(c, gpu_dma);
                        }
                    }
                },
                _ => {
                    if !fmt_arg {
                        self.print_char(ascii, gpu_dma);
                    }
                },
            }
        }
        gpu_dma.send_list(&self.ot).wait();
    }

    fn format_u32(x: u32, leading_zeros: bool) -> [u8; 9] {
        let mut leading = !leading_zeros;
        let mut ar = [0; 9];
        let mut j = 0;
        for i in 0..8 {
            let nibble = (x >> ((7 - i) * 4)) & 0xF;
            if nibble != 0 || i == 7 {
                leading = false;
            };
            if !leading {
                let as_char = core::char::from_digit(nibble, 16)
                    .unwrap_unchecked()
                    .to_ascii_uppercase();
                unsafe { *ar.get_unchecked_mut(j) = as_char as u8 };
                j += 1;
            }
        }
        unsafe { *ar.get_unchecked_mut(j) = b'h' };
        ar
    }
}
