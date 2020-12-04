use crate::gpu::prim::{Buffer, OT};
use crate::gpu::Color;
use crate::gpu::Vertex;
use crate::gpu::{Clut, TexPage};
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
        let mut font = unzip_now!("../../font.tim.zip");
        let tim = TIM::new(&mut font);
        // TODO: wtf is a 2? use an enum here
        gp1.dma_direction(2);
        let transfer = gpu_dma.load_tim(&tim);
        let next_transfer = transfer.wait();
        let (tpage, clut) = next_transfer(gpu_dma, &tim).wait();
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
        self.cursor = self.cursor.shift((0, self.font_size.y()));
    }

    // TODO: make sure we don't overrun the buffer
    pub fn print<'a, M>(
        &mut self, msg: M, gp0: &mut gpu::GP0, gp1: &mut gpu::GP1, gpu_dma: &mut dma::gpu::Channel,
    ) where M: Iterator<Item = u8> {
        self.set_texpage(gp0);
        let w = self.font_size.x() as u8;
        let h = self.font_size.y() as u8;
        // Assuming only one texture page is used
        let ascii_per_row = 128 / w;
        for ascii in msg {
            if ascii == b'\n' {
                self.newline();
            } else {
                let xoffset = (ascii % ascii_per_row) * w;
                let yoffset = (ascii / ascii_per_row) * h;
                let letter = self
                    .buffer
                    .Sprt()
                    .unwrap()
                    .color(self.color.unwrap_or(Color::WHITE))
                    .offset(self.cursor.shift(self.box_offset))
                    .t0((xoffset, yoffset))
                    .clut(self.clut)
                    .size(self.font_size);
                self.ot.add_prim(0, letter);
                if self.cursor.x() + self.font_size.x() >= self.box_offset.x() + self.box_size.x() {
                    self.cursor = self
                        .cursor
                        .shift((-self.box_size.x() - self.font_size.x(), self.font_size.y()));
                } else {
                    self.cursor = self.cursor.shift((self.font_size.x(), 0));
                }
            }
        }
        gpu_dma.prepare_ot(gp1).send(&self.ot).wait();
    }
}
