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
    // Get an array with a .TIM for Ferris, defer decompression
    let ferris = unzip!("../ferris.tim.zip");
    // Get a TIM struct, defer parsing the TIM
    let tim = tim!(ferris);

    let zero = Vertex::zero();
    // Copy a &[u32] to VRAM
    // This triggers parsing the TIM which triggers decompressing the file
    draw_port.rect_to_vram(zero, (256, 256), tim.bitmap().body());
    // The following should error since zero is consumed above
    //draw_port.rect_to_vram(zero, (256, 256), &ferris[5..]);
    loop {}
}
