#![no_std]
#![no_main]
#![feature(once_cell)]

use psx::gpu::vertex::Vertex;
use psx::{tim, unzip, unzip_now};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    disp_port.on();
    // Size of the data in the unzipped .tim
    let ferris = unzip!("../ferris.tim.zip");
    let ferris = tim!(ferris);

    let zero = Vertex::zero();
    draw_port.rect_to_vram(zero, (256, 256), ferris.bit_map().body());
    // The following should error since zero is consumed above
    //draw_port.rect_to_vram(zero, (256, 256), &ferris[5..]);
    loop {}
}
