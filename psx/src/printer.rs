use crate::gpu::color::Color;
use crate::gpu::primitive::sprt::Sprt;
use crate::gpu::primitive::{Buffer, OT};
use crate::gpu::texture::{Clut, TexPage};
use crate::gpu::vertex::Vertex;
use crate::mmio::register::Write;
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

pub struct UnsafePrinter<const N: usize> {
    printer: Printer<N>,
    otc_dma: dma::otc::Channel,
    gpu_dma: dma::gpu::Channel,
    gp0: gpu::GP0,
    gp1: gpu::GP1,
}

impl<const N: usize> UnsafePrinter<N> {
    pub fn new<T, U, V, S>(
        cursor: T, font_size: U, box_offset: V, box_size: S, color: Color,
    ) -> Self
    where Vertex: From<T> + From<U> + From<V> + From<S> {
        unsafe {
            let mut otc_dma = dma::otc::Channel::new();
            UnsafePrinter {
                printer: Printer::<N>::new(
                    cursor,
                    font_size,
                    box_offset,
                    box_size,
                    color,
                    &mut otc_dma,
                ),
                otc_dma,
                gpu_dma: dma::gpu::Channel::new(),
                gp0: gpu::GP0::new(),
                gp1: gpu::GP1::new(),
            }
        }
    }

    pub fn load_font(&mut self) {
        self.printer.load_font(&mut self.gp1, &mut self.gpu_dma)
    }

    pub fn print<'a, M>(&mut self, msg: M)
    where M: Iterator<Item = u8> {
        self.printer
            .print(msg, &mut self.gp0, &mut self.gp1, &mut self.gpu_dma)
    }
}

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
    color: Color,
}

impl<const N: usize> Printer<N> {
    pub fn new<T, U, V, S>(
        cursor: T, font_size: U, box_offset: V, box_size: S, color: Color,
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
        let mut ot = OT::<1>::new();
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
        let mut font = unzip_now!("../font.tim.zip");
        let tim = TIM::new(&mut font);
        gp1.dma_direction(2);
        let (tpage, clut) = gpu_dma.load_tim(&tim);
        self.tpage = Some(tpage);
        self.clut = clut;
    }

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
                    .color(Color::WHITE)
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
