#![no_std]
#![no_main]
#![feature(array_map)]

use core::cell::RefCell;
use core::mem::transmute;

use psx::gpu::framebuffer::Framebuffer;
use psx::gpu::{DispPort, DmaSource, DrawPort, Hres, Vres};

mod huffman_code;
use crate::huffman_code::{CODES, SYMBOLS};

psx::exe!();

fn mk_framebuffer<'a, 'b>(
    draw_port: &'a RefCell<DrawPort>,
    disp_port: &'b RefCell<DispPort>,
) -> Framebuffer<'a, 'b> {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (Hres::H320, Vres::V240);
    disp_port.borrow_mut().reset_gpu();
    disp_port.borrow_mut().dma(DmaSource::CPU);
    Framebuffer::new(draw_port, disp_port, buf0, buf1, res)
}

fn main(mut io: IO) {
    let draw_port = RefCell::new(io.take_draw_port().expect("DrawPort has been taken"));
    let disp_port = RefCell::new(io.take_disp_port().expect("DispPort has been taken"));
    mk_framebuffer(&draw_port, &disp_port);
    let ferris = decompress();
    let mut ferris = ferris[5..].into_iter().cloned();

    draw_port
        .borrow_mut()
        .rect_to_vram((0, 0), (256, 256), &mut ferris);
    loop {}
}

fn decompress() -> [u32; 32773] {
    let compressed_exe =
        unsafe { transmute::<_, [u32; 7187]>(*include_bytes!("../ferris.tim.hzip")) };
    let mut exe = [0; 131092];
    let mut possible_code_len = 0;
    let mut possible_code = 0;
    let mut i = 0;
    for &w in &compressed_exe {
        let mut remaining_bits = 32;
        let mut stream = w as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code_len += 1;
            possible_code = (stream >> 32) as u32;
            CODES
                .iter()
                .position(|&code| code == possible_code)
                .map(|idx| {
                    let symbol = SYMBOLS[idx];
                    exe[i] = symbol;
                    i += 1;
                    possible_code_len = 0;
                    stream &= 0x0000_0000_FFFF_FFFF;
                });
        }
        possible_code = (stream >> 32) as u32;
    }
    unsafe { transmute::<_, [u32; 32773]>(exe) }
}
