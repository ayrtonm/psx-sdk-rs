#![no_std]
#![no_main]
#![feature(array_map, once_cell, min_const_generics)]

use core::convert::TryFrom;
use psx::dma::{Addr, Block, BlockLen, Control};
use psx::gpu::color::*;
use psx::gpu::framebuffer::*;
use psx::gpu::primitives::*;
use psx::gpu::texture::*;
use psx::gpu::vertex::*;
use psx::gpu::Packet;
use psx::*;

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().unwrap();
    let mut gpu_stat = io.take_gpu_stat().unwrap();
    let mut disp_port = io.take_disp_port().unwrap();
    let mut gpu_dma = io.take_gpu_dma().unwrap();
    let res = (320, 240);
    let mut fb = Framebuffer::new(
        &mut draw_port,
        &mut disp_port,
        Vertex::zero(),
        (0, 240),
        res,
        None,
    );
    let font = unzip!("../font.tim.zip");
    let (page, clut) = tim!(font).load(&mut draw_port, &mut gpu_dma);
    let mut printer = Printer {
        cursor: (0, 0).into(),
        size: (8, 16).into(),
        offset: (0, 0).into(),
        limits: res.into(),
        color: Color::white(),
        page,
        clut,
    };
    let msg =
        printer.print(b"hello world! This is a very long message. It goes on and on and on... In fact, it might even start overwriting itself if I keep writing such long run-on sentences. I should find a way to make sure that words aren't split across lines. But then again, this isn't the place to write my TODOs. That's TODO.md. It really doesn't get much use though :(. Let's see what other emojis I can make :) :/ :p xD :D ^^ :] :^) 0123456789 ~!@#$%^&*_+=; Still got a ways to go before the message starts overwriting itself. Another thing I should add is msg support to my panic handler. That would make debugging so much easier!");
    for d in &msg {
        while !gpu_stat.ready() {}
        draw_port.send(d);
    }
    fb.swap(&mut draw_port, &mut disp_port);
    delay(10_000_000);
    printer.reset_cursor();
    let new_msg_1 = printer.print(b"1 + 1 = ");
    let new_msg_2 = printer.println(&[b'0' + 1 + 1]);
    let new_msg_3 = printer.print(b"That was fmt in a loose sense\n 1 + 9 = 0x");
    let expr = 1 + 9;
    let new_msg_5 =
        printer.println(&[u32::try_from(core::char::from_digit(expr, 16).unwrap()).unwrap() as u8]);
    let new_msg_6 = printer.println(
        b"That last one doesn't depend on ascii's order, but that'll only work for single digits",
    );
    for d in new_msg_1
        .iter()
        .chain(&new_msg_2)
        .chain(&new_msg_3)
        .chain(&new_msg_5)
        .chain(&new_msg_6)
    {
        while !gpu_stat.ready() {}
        draw_port.send(d);
    }
    fb.swap(&mut draw_port, &mut disp_port);
}

struct Printer {
    cursor: Vertex,
    size: Vertex,
    offset: Vertex,
    limits: Vertex,
    color: Color,
    page: Page,
    clut: Option<Clut>,
}

impl Printer {
    fn reset_cursor(&mut self) {
        self.cursor = self.offset;
    }
    fn println<const N: usize>(&mut self, msg: &[u8; N]) -> [TexturedPrimitive<4>; N] {
        let primitives = self.print(msg);
        let vshift = self.size.y();
        self.cursor.apply(|x, y| (0, y + vshift));
        primitives
    }
    fn print<const N: usize>(&mut self, msg: &[u8; N]) -> [TexturedPrimitive<4>; N] {
        let w_as_u8 = self.size.x() as u8;
        let h_as_u8 = self.size.y() as u8;
        // This assumes that only one texture page is used
        let ascii_per_row = 128 / w_as_u8;
        msg.map(|ascii| {
            if ascii == b'\n' {
                let vshift = self.size.y();
                self.cursor.apply(|x, y| (0, y + vshift));
                textured_quad(
                    Vertex::offset_rect(self.offset, (0, 0)),
                    self.color,
                    [(0, 0); 4],
                    self.page,
                    self.clut,
                )
            } else {
                let xoffset = ((ascii % ascii_per_row) * w_as_u8);
                let yoffset = ((ascii / ascii_per_row) * h_as_u8);
                let letter = textured_quad(
                    Vertex::offset_rect(self.cursor.shift(self.offset), self.size),
                    self.color,
                    [(0, 0), (0, h_as_u8), (w_as_u8, 0), (w_as_u8, h_as_u8)]
                        .map(|(x, y)| (x + xoffset, y + yoffset)),
                    self.page,
                    self.clut,
                );
                if self.cursor.x() + self.size.x() >= self.limits.x() {
                    let vshift = self.size.y();
                    self.cursor.apply(|x, y| (0, y + vshift));
                } else {
                    let hshift = self.size.x();
                    self.cursor.apply(|x, y| (x + hshift, y));
                }
                letter
            }
        })
    }
}
