use crate::dma;
use crate::dma::{Addr, Block, BlockLen, Control};
use crate::gpu::texture::{Bpp, Clut, Page};
use crate::gpu::vertex::{Pixel, Vertex};
use crate::gpu::{AsU32, DrawPort};

pub struct TIM<'a> {
    bpp: Bpp,
    bitmap: Bitmap<'a>,
    clut: Option<Bitmap<'a>>,
}

type Result<'a, T> = dma::Transfer<'a, dma::gpu::Control, T>;
impl<'a> TIM<'a> {
    pub fn new(src: &'a [u32]) -> Self {
        let clut = ((src[1] & 8) != 0).then_some(Bitmap::new(&src[2..]));
        let (offset, clut) = match clut {
            Some((offset, clut)) => (offset + 2, Some(clut)),
            None => (2, None),
        };
        let (_, bitmap) = Bitmap::new(&src[offset..]);
        let bpp = match src[1] & 3 {
            0 => Bpp::B4,
            1 => Bpp::B8,
            2 => Bpp::B15,
            _ => unreachable!("TIM contains an invalid bpp"),
        };
        TIM { bpp, bitmap, clut }
    }

    // TODO: Make this return (Result<'a, Page>, Option<Result<'a, Clut>>) and be non-blocking.
    pub fn load(
        &self, draw_port: &mut DrawPort, gpu_dma: &'a mut dma::Gpu,
    ) -> (Page, Option<Result<'a, Clut>>) {
        fn send_header(bmp: &Bitmap, draw_port: &mut DrawPort) {
            draw_port.send(&[
                0xA0 << 24,
                Vertex::from(bmp.offset()).as_u32(),
                Vertex::from(bmp.size()).as_u32(),
            ]);
        }

        fn enqueue_bitmap(bmp: &Bitmap, gpu_dma: &mut dma::Gpu) {
            gpu_dma.addr.set(bmp.body().as_ptr());
            gpu_dma.block.set(BlockLen::Words(bmp.body().len()));
        }

        gpu_dma.control.set_direction(dma::Direction::FromRam);
        gpu_dma.control.set_step(dma::Step::Forward);
        gpu_dma.control.set_chopping(false);
        gpu_dma.control.set_sync_mode(dma::Mode::Immediate);

        let bmp = self.bitmap();
        let base_x = (bmp.offset().0 / 64) as u8;
        let base_y = (bmp.offset().1 / 256) as u8;
        let page = Page::new(base_x, base_y, self.bpp);
        send_header(bmp, draw_port);
        enqueue_bitmap(bmp, gpu_dma);
        // TODO: remove wait
        let page = gpu_dma.control.start(Some(page)).wait().unwrap();
        let clut = self.clut().map(move |clut| {
            send_header(clut, draw_port);
            enqueue_bitmap(clut, gpu_dma);
            let base_x = (clut.offset().0 / 16) as u8;
            let base_y = clut.offset().1;
            let clut = (base_x, base_y).into();
            gpu_dma.control.start(Some(clut))
        });
        (page, clut)
    }

    pub fn bitmap(&self) -> &Bitmap<'a> {
        &self.bitmap
    }

    pub fn clut(&self) -> Option<&Bitmap<'a>> {
        self.clut.as_ref()
    }

    pub fn bpp(&self) -> Bpp {
        self.bpp
    }
}

pub struct Bitmap<'a> {
    len: u32,
    offset: (Pixel, Pixel),
    size: (Pixel, Pixel),
    body: &'a [u32],
}

impl<'a> Bitmap<'a> {
    pub fn new(src: &'a [u32]) -> (usize, Self) {
        let len = src[0];
        let len_by_u32 = (len as usize) / 4;
        let x = src[1] as Pixel;
        let y = (src[1] >> 16) as Pixel;
        let width = src[2] as Pixel;
        let height = (src[2] >> 16) as Pixel;
        (
            len_by_u32,
            Bitmap {
                len,
                offset: (x, y),
                size: (width, height),
                body: &src[3..len_by_u32],
            },
        )
    }

    pub fn body(&self) -> &[u32] {
        self.body
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn offset(&self) -> (Pixel, Pixel) {
        self.offset
    }

    pub fn size(&self) -> (Pixel, Pixel) {
        self.size
    }
}
