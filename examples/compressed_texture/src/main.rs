#![no_std]
#![no_main]
#![feature(array_map)]
#![feature(core_intrinsics)]
#![feature(min_const_generics)]

use core::cell::RefCell;

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
    let ferris = decompress::<32773>();
    let mut ferris = ferris[5..].into_iter().cloned();

    draw_port
        .borrow_mut()
        .rect_to_vram((0, 0), (256, 256), &mut ferris);
    loop {}
}

fn decompress<const N: usize>() -> [u32; N] {
    // TODO: handle possible misalignment
    let compressed_exe = unsafe { include_bytes!("../ferris.tim.zip").align_to::<u32>().1 };
    let decompressed_len = compressed_exe[0] as usize;
    let mut ret = [0; N];
    // TODO: handle possible misalignment
    let exe = unsafe { ret.align_to_mut::<u8>().1 };
    let mut possible_code = 0;
    let mut possible_code_len = 0;
    let mut i = 0;
    for &w in compressed_exe {
        let mut remaining_bits = 32;
        let mut stream = w as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code = (stream >> 32) as u32;
            possible_code_len += 1;
            (&CODES)
                .binary_search(&possible_code)
                .map(|idx| {
                    if i < decompressed_len {
                        exe[i] = SYMBOLS[idx];
                        i += 1;
                        stream &= 0x0000_0000_FFFF_FFFF;
                        possible_code_len = 0;
                    }
                })
                .ok();
        }
        possible_code = (stream >> 32) as u32;
    }
    ret
}
