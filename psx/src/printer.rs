use crate::framebuffer::draw_sync;
use crate::gpu;
use crate::gpu::colors::*;
use crate::gpu::primitives::Sprt8;
use crate::gpu::{Color, TexCoord, Vertex};
use crate::hw::gpu::GP0;
use crate::tim::{TIMCoords, TIM};
use core::fmt;
use core::mem::size_of;
use core::slice;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Font {
    coords: TIMCoords,
}

// Up to 5 `Sprt8`s fit in the GPU buffer at one time.
const BUFFER_SIZE: usize = gpu::BUFFER_SIZE / size_of::<Sprt8>();

pub struct TextBox {
    font: Font,
    color: Color,
    initial: Vertex,
    cursor: Vertex,
    idx: usize,
    buffer: [Sprt8; BUFFER_SIZE],
}

impl Font {
    pub fn new(font: TIM) -> Self {
        let coords = font.load(None).expect("");
        Self { coords }
    }

    pub fn default() -> Self {
        // TODO: Make a better API for including a file as a [u32; N] since TIM takes
        // &mut [u32]
        let mut tim = *include_bytes!("../font.tim");
        let font = unsafe {
            TIM::new(slice::from_raw_parts_mut(
                tim.as_mut_ptr().cast(),
                tim.len() / size_of::<u32>(),
            ))
            .expect("The default font is a valid TIM file")
        };
        Self::new(font)
    }

    pub fn text_box(&self, offset: Vertex, color: Option<Color>) -> TextBox {
        let color = color.unwrap_or(WHITE);
        let mut buffer = [Sprt8::new(); BUFFER_SIZE];
        for letter in &mut buffer {
            if let Some(clut) = self.coords.clut {
                letter.set_clut(clut);
            }
            letter.set_color(color);
        }
        TextBox {
            font: *self,
            color,
            initial: offset,
            cursor: offset,
            idx: 0,
            buffer,
        }
    }
}

const FONT_SIZE: u8 = 8;

impl TextBox {
    pub fn print_char(&mut self, ascii: u8) {
        if ascii == b'\n' {
            self.newline();
            self.cursor.0 = self.initial.0;
        } else {
            let ascii_per_row = 128 / FONT_SIZE;
            let ascii = ascii - (2 * ascii_per_row);
            let x = (ascii % ascii_per_row) * FONT_SIZE;
            let y = (ascii / ascii_per_row) * FONT_SIZE;
            if self.idx == 0 {
                draw_sync();
            }
            self.buffer[self.idx]
                .set_offset(self.cursor)
                .set_tex_coord(TexCoord { x, y });
            GP0::new().send_command(&self.buffer[self.idx]);
            self.idx += 1;
            if self.idx == BUFFER_SIZE {
                self.idx = 0;
            }
            self.cursor.0 += FONT_SIZE as i16;
            // TODO: Fix assumption that resolution is 320x240
            if self.cursor.0 == 320 {
                self.newline();
                self.cursor.0 = self.initial.0;
            }
        }
    }
    pub fn newline(&mut self) {
        self.cursor = Vertex(self.initial.0, self.cursor.1 + FONT_SIZE as i16);
    }
    pub fn reset(&mut self) {
        self.cursor = self.initial;
    }
    pub fn change_color(&mut self, color: Color) {
        if color != self.color {
            self.color = color;
            for letter in &mut self.buffer {
                letter.set_color(color);
            }
        }
    }
}

impl fmt::Write for TextBox {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        for c in msg.chars() {
            if c.is_ascii() {
                self.print_char(c as u8);
            } else {
                self.print_char(b'?');
            }
        }
        Ok(())
    }
}

/// Print a rust-style format string and args using the `TextBox` specified by
/// `$box`.
#[macro_export]
macro_rules! dprint {
    ($box:expr, $($args:tt)*) => {
        core::fmt::Write::write_fmt(&mut $box, format_args!($($args)*)).ok()
    };
}

/// Print a rust-style format string and args using the `TextBox` specified by
/// `$box`.
#[macro_export]
macro_rules! dprintln {
    ($box:expr, $($args:tt)*) => {
        $crate::dprint!($box, $($args)*);
        $box.print_char(b'\n');
    };
}
