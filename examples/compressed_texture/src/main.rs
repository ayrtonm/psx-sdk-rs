#![no_std]
#![no_main]
#![feature(once_cell)]

use psx::gpu::vertex::Vertex;
use psx::{unzip, unzipped_size};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    disp_port.on();
    // Size of the data in the unzipped .tim
    const _N: usize = unzipped_size!("../ferris.tim.zip") - 5;
    let ferris = unzip!("../ferris.tim.zip");

    let zero = Vertex::zero();
    draw_port.rect_to_vram(zero, &(256, 256), &ferris[5..]);
    // The following should error
    //draw_port.rect_to_vram(zero, &(256, 256), &ferris[5..]);
    loop {}
}
