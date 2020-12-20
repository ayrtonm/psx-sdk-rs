use crate::gpu::Color;
use crate::gpu::Vertex;
use crate::gpu::{Clut, TexPage};
use crate::graphics::{Buffer, OT};
use crate::mmio::register::Write;
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

mod wrapper;

pub use wrapper::UnsafePrinter;

pub struct Printer<const N: usize> {
    // Where the font is stored
    tpage: Option<TexPage>,
    clut: Option<Clut>,

    buffer: Buffer<N>,
    ot: OT<1>,

    cursor: Vertex,
    font_size: Vertex,
    box_offset: Vertex,
    box_size: Vertex,
    color: Option<Color>,
}

impl<const N: usize> Printer<N> {
    pub fn new<T, U, V, S>(
        cursor: T, font_size: U, box_offset: V, box_size: S, color: Option<Color>,
        otc_dma: &mut dma::otc::Channel,
    ) -> Self
    where
        Vertex: From<T> + From<U> + From<V> + From<S>,
    {
        let cursor = Vertex::from(cursor);
        let font_size = Vertex::from(font_size);
        let box_offset = Vertex::from(box_offset);
        let box_size = Vertex::from(box_size);
        let buffer = Buffer::<N>::new();
        let ot = OT::<1>::new();
        // How unnecessary is this to not lock up the GPU? In case it is needed, I could
        // just write the single value straight to the ordering table
        otc_dma.clear(&ot).wait();
        Printer {
            tpage: None,
            clut: None,

            buffer,
            ot,

            cursor,
            font_size,
            box_offset,
            box_size,
            color,
        }
    }

    pub fn load_font(&mut self, gp1: &mut gpu::GP1, gpu_dma: &mut dma::gpu::Channel) {
        let mut font = unzip_now!("../../small_font_subset.tim.zip");
        let tim = TIM::new(&mut font);
        // TODO: wtf is a 2? use an enum here
        gp1.dma_direction(2);
        let transfer = gpu_dma.load_tim(&tim);
        let next_transfer = transfer.wait();
        let (tpage, clut) = next_transfer(gpu_dma, &tim).maybe_wait();
        self.tpage = Some(tpage);
        self.clut = clut;
    }

    // TODO: figure out at which point it makes the most sense to set the TexPage
    // (probably once per call to print)
    // TODO: I shouldn't be calling `write`, add a method to GP0 instead
    pub fn set_texpage(&self, gp0: &mut gpu::GP0) {
        unsafe {
            gp0.write(0xe1 << 24 | 0xa | (0 << 4));
        }
    }

    pub fn newline(&mut self) {
        self.cursor = self.cursor.shift((-self.cursor.x(), self.font_size.y()));
    }

    // TODO: make sure we don't overrun the buffer
    pub fn print<'a, M, const A: usize>(
        &mut self, msg: M, args: [u32; A], gp0: &mut gpu::GP0, gp1: &mut gpu::GP1,
        gpu_dma: &mut dma::gpu::Channel,
    ) where
        M: IntoIterator<Item = &'a u8>,
    {
        let w = self.font_size.x() as u8;
        let h = self.font_size.y() as u8;
        // Assuming only one texture page is used
        let ascii_per_row = 128 / w;
        let print_char = |printer: &mut Self, ascii| {
            let ascii = ascii - (2 * ascii_per_row);
            let xoffset = (ascii % ascii_per_row) * w;
            let yoffset = (ascii / ascii_per_row) * h;
            let letter = printer.buffer.sprt8().unwrap();
            letter
                .color(printer.color.unwrap_or(Color::WHITE))
                .offset(printer.cursor.shift(printer.box_offset))
                .t0((xoffset, yoffset))
                .clut(printer.clut);
            printer.ot.add_prim(letter, 0);
            if printer.cursor.x() + printer.font_size.x() >=
                printer.box_offset.x() + printer.box_size.x()
            {
                printer.cursor = printer.cursor.shift((
                    -printer.box_size.x() - printer.font_size.x(),
                    printer.font_size.y(),
                ));
            } else {
                printer.cursor = printer.cursor.shift((printer.font_size.x(), 0));
            }
        };
        self.set_texpage(gp0);
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
                    print_char(self, b'{')
                },
                b'}' if fmt_arg => {
                    fmt_arg = false;
                    let arg = args.next().unwrap();
                    let formatted = Self::format_u32(*arg, leading_zeros);
                    leading_zeros = false;
                    for &c in &formatted {
                        if c != b'\0' {
                            print_char(self, c)
                        };
                    }
                },
                _ => {
                    if !fmt_arg {
                        print_char(self, ascii)
                    }
                },
            }
        }
        gpu_dma.prepare_ot(gp1).send(&self.ot).wait();
    }

    pub fn format_u32(x: u32, leading_zeros: bool) -> [u8; 9] {
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
