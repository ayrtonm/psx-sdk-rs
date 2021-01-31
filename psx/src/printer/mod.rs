//! Text printing and formatting routines

#![allow(dead_code)]
use crate::gpu::{draw_sync, Clut, Color, Pixel, TexPage, Vertex, WHITE};
use crate::graphics::primitive::Sprt8;
use crate::hal::GP0;
use crate::tim::TIM;
use fmt::format_u32;

mod fmt;

pub struct Printer {
    font: Option<Font>,
    cursor: Vertex,
    box_offset: Vertex,
    box_size: Vertex,
    color: Color,
}

struct Font {
    tpage: TexPage,
    clut: Option<Clut>,
}

const FONT_SIZE: u8 = 8;

impl Printer {
    pub const fn new(
        cursor: (Pixel, Pixel), box_offset: (Pixel, Pixel), box_size: (Pixel, Pixel),
        color: Option<Color>,
    ) -> Self {
        let cursor = Vertex::new(cursor);
        let box_offset = Vertex::new(box_offset);
        let box_size = Vertex::new(box_size);
        let color = match color {
            Some(color) => color,
            None => WHITE,
        };
        Printer {
            font: None,
            cursor,
            box_offset,
            box_size,
            color,
        }
    }

    pub fn load_font(&mut self) {
        let mut font = unzip!("../../font.tim.zip");
        let tim = TIM::new(&mut font);
        self.font = Some(Font {
            tpage: tim.tex_page(),
            clut: tim.clut(),
        });
        GP0.write_slice(tim.bmp());
        tim.clut_bmp().map(|clut_bmp| {
            GP0.write_slice(clut_bmp);
        });
    }

    pub fn newline(&mut self) {
        self.cursor = (0, self.cursor.y + 8).into();
    }

    pub fn reset(&mut self) {
        self.cursor = self.box_offset;
    }

    pub fn println<'m, M, const A: usize>(&mut self, msg: M, args: [u32; A])
    where M: AsRef<[u8]> {
        self.print(msg, args);
        self.newline();
    }

    pub fn print<'m, M, const A: usize>(&mut self, msg: M, args: [u32; A])
    where M: AsRef<[u8]> {
        let mut fmt_arg = false;
        let mut leading_zeros = false;
        let mut hexdecimal = false;
        let mut whitespace = false;
        let mut args_iter = args.iter();
        for &ascii in msg.as_ref() {
            match ascii {
                b'\n' => self.newline(),
                b'\0' => break,
                b'0' if fmt_arg => leading_zeros = true,
                b'x' if fmt_arg => hexdecimal = true,
                b' ' if fmt_arg => whitespace = true,
                b'{' if !fmt_arg => fmt_arg = true,
                b'{' if fmt_arg => {
                    fmt_arg = false;
                    self.print_char(b'{');
                },
                b'}' if fmt_arg => {
                    fmt_arg = false;
                    let arg = *args_iter.next().unwrap();
                    let formatted = format_u32(arg, leading_zeros, hexdecimal);
                    leading_zeros = false;
                    hexdecimal = false;
                    for &c in &formatted {
                        if c != b'\0' {
                            if whitespace {
                                self.print_char(b' ');
                            } else {
                                self.print_char(c);
                            }
                        }
                    }
                },
                _ => {
                    if !fmt_arg {
                        self.print_char(ascii);
                    }
                },
            }
        }
    }

    fn print_char(&mut self, ascii: u8) {
        let ascii_per_row = 128 / FONT_SIZE;
        // Font texture doesn't contain the first 2 rows (32 ascii characters) to save
        // VRAM
        let ascii = ascii - (2 * ascii_per_row);
        let xoffset = (ascii % ascii_per_row) * FONT_SIZE;
        let yoffset = (ascii / ascii_per_row) * FONT_SIZE;
        let mut letter = Sprt8::new();
        letter
            .set_color(self.color)
            .set_clut(self.font.as_ref().map(|f| f.clut).flatten())
            .set_offset(self.cursor.shift(self.box_offset))
            .set_tex_coord((xoffset, yoffset));
        GP0.draw(&letter);
        draw_sync();
        if self.cursor.x + FONT_SIZE as Pixel >= self.box_offset.x + self.box_size.x {
            self.newline();
        } else {
            self.cursor = self.cursor.shift((FONT_SIZE as Pixel, 0).into());
        }
    }
}
