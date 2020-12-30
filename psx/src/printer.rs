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

/// A font stored in the framebuffer
pub struct Font {
    tpage: TexPage,
    clut: Option<Clut>,
}

/// Screen printer with an `N` word primitive buffer for character sprites.
pub struct Printer<const N: usize> {
    font: Option<Font>,
    buffer: Buffer<N>,
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

        let mut font = include_u32!("../font.tim");
        let tim = TIM::new(&mut font);
        let font = Font {
            tpage: tim.tex_page(),
            clut: tim.clut(),
        };

        let bmp = tim.bitmap.data();
        dma::gpu::MADR.set(bmp.first().unwrap());
        dma::gpu::BCR.set(bmp.len());
        gpu_dma.load_mut().start(()).wait();

        tim.clut_bitmap.map(|clut_bitmap| {
            let clut = clut_bitmap.data();
            dma::gpu::MADR.set(clut.first().unwrap());
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

    /// Prints a message with the given formatted arguments.
    pub fn print<'a, M, const A: usize>(
        &mut self, msg: M, args: [u32; A], gpu_dma: &mut dma::gpu::CHCR,
    ) where M: IntoIterator<Item = &'a u8> {
        let (w, h) = (8, 8);
        let ascii_per_row = 128 / w;
        let mut ot = OT::default();
        let mut print_char = |printer: &mut Self, ascii| {
            // Offset ascii values to work with font subset stored in VRAM.
            let ascii = ascii - (2 * ascii_per_row);
            let xoffset = (ascii % ascii_per_row) * w;
            let yoffset = (ascii / ascii_per_row) * h;
            let mut letter = match printer.buffer.sprt8() {
                Some(sprt) => sprt,
                None => {
                    gpu_dma.send_list(&ot).wait();
                    ot.empty();
                    printer.buffer.empty().sprt8().unwrap()
                },
            };
            letter
                .set_color(printer.color)
                .set_offset(printer.cursor.shift(printer.box_offset))
                .set_tex_coord((xoffset, yoffset))
                .set_clut(printer.font.as_ref().map(|f| f.clut).flatten());
            ot.insert(&mut letter, 0);
            if printer.cursor.x + 8 >= printer.box_offset.x + printer.box_size.x {
                printer.newline();
            } else {
                printer.cursor = printer.cursor.shift((8, 0));
            };
        };
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
                    print_char(self, b'{');
                },
                b'}' if fmt_arg => {
                    fmt_arg = false;
                    let arg = args.next().unwrap();
                    let formatted = Self::format_u32(*arg, leading_zeros);
                    leading_zeros = false;
                    for &c in &formatted {
                        if c != b'\0' {
                            print_char(self, c);
                        }
                    }
                },
                _ => {
                    if !fmt_arg {
                        print_char(self, ascii);
                    }
                },
            }
        }
        gpu_dma.send_list(&ot).wait();
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
                    .unwrap()
                    .to_ascii_uppercase();
                ar[j] = as_char as u8;
                j += 1;
            }
        }
        ar[j] = b'h';
        ar
    }
}
