#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;
use psx::dma;
use psx::dma::{Addr, Block, BlockLen, Control, Direction, Mode, Step};
use psx::gpu::color::Color;
use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::vertex::{Component, Vertex};
use psx::gpu::{DispPort, DmaSource, DrawPort, Hres, Vres};

psx::exe!();

fn mk_framebuffer<'a, 'b>(
    draw_port: &'a RefCell<DrawPort>, disp_port: &'b RefCell<DispPort>,
) -> Framebuffer<'a, 'b> {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    disp_port.borrow_mut().reset_gpu();
    disp_port.borrow_mut().dma(DmaSource::CPU);
    Framebuffer::new(draw_port, disp_port, buf0, buf1, res)
}

fn main(mut ctxt: Ctxt) {
    let draw_port = RefCell::new(ctxt.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(ctxt.take_disp_port().expect("DispPort has been taken"));
    let mut dma = ctxt.take_gpu_dma().expect("GPU DMA has been taken");
    let mut fb = mk_framebuffer(&draw_port, &disp_port);

    dma.control.set_direction(Direction::FromRam);
    dma.control.set_step(Step::Forward);
    dma.control.set_chopping(false);
    dma.control.set_sync_mode(Mode::Immediate);

    let mut theta = 0.0;
    let delta = 0.0625;
    loop {
        theta += delta;
        while theta > 360.0 {
            theta -= 360.0;
        }
        do_transfer(&mut dma, theta);
        fb.swap();
    }
}

fn do_transfer(dma: &mut dma::Gpu, theta: f32) {
    let dma_data = serialize(draw(theta));
    dma.addr.set(dma_data.as_ptr());
    dma.block.set(BlockLen::Words(dma_data.len()));
    let t = dma.control.start();
    t.wait();
}

fn serialize((quad, pal): ([Vertex; 4], [Color; 4])) -> [u32; 8] {
    [
        (0x3800_0000 as u32) | u32::from(&pal[0]),
        (&quad[0]).into(),
        (&pal[1]).into(),
        (&quad[1]).into(),
        (&pal[2]).into(),
        (&quad[2]).into(),
        (&pal[3]).into(),
        (&quad[3]).into(),
    ]
}

fn draw(theta: f32) -> ([Vertex; 4], [Color; 4]) {
    let center = &Vertex::new(160, 120);
    let size = 128;
    let square = Vertex::square(center, size).map(|p| rotate_point(p, theta, center));
    let palette = [
        Color::aqua(),
        Color::mint(),
        Color::indigo(),
        Color::orange(),
    ];
    (square, palette)
}

fn sin(mut x: f32) -> f32 {
    fn approx_sin(z: f32) -> f32 {
        4.0 * z * (180.0 - z) / (40500.0 - (z * (180.0 - z)))
    }
    while x < 0.0 {
        x += 360.0;
    }
    while x > 360.0 {
        x -= 360.0;
    }
    if x <= 180.0 {
        approx_sin(x)
    } else {
        -approx_sin(x - 180.0)
    }
}

fn cos(x: f32) -> f32 {
    let y = 90.0 - x;
    sin(y)
}

// Rotation is better handled by the GTE but this'll do for a demo
fn rotate_point(p: Vertex, theta: f32, c: &Vertex) -> Vertex {
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    Vertex::new(xf as Component, yf as Component)
}
