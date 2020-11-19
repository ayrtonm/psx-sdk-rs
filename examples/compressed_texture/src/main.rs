#![no_std]
#![no_main]
#![feature(once_cell)]

use psx::{u32_array, unzip};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    disp_port.on();
    let ferris = unzip!("../ferris.tim.zip").0;
    const N: usize = unzip!("../ferris.tim.zip").1 - 5;

    draw_port.rect_to_vram((0, 0), (256, 256), u32_array::<N>(&ferris[5..]));
    loop {}
}
