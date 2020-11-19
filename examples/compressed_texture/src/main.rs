#![no_std]
#![no_main]
#![feature(once_cell)]

use psx::{unzip, unzipped_size};

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    disp_port.on();
    // Size of the data in the unzipped .tim
    const N: usize = unzipped_size!("../ferris.tim.zip") - 5;
    let ferris = unzip!("../ferris.tim.zip");

    draw_port.rect_to_vram((0, 0), (256, 256), &ferris[5..]);
    loop {}
}
