#![no_std]
#![no_main]
#![feature(array_map)]
#![feature(core_intrinsics)]
#![feature(min_const_generics)]
#![feature(once_cell)]

use psx::unzip;

psx::exe!();

fn main(mut io: IO) {
    let mut draw_port = io.take_draw_port().expect("DrawPort has been taken");
    let mut disp_port = io.take_disp_port().expect("DispPort has been taken");
    disp_port.on();
    let ferris = unzip!("../ferris.tim.zip");
    let mut ferris = ferris[5..].into_iter();

    draw_port.rect_to_vram((0, 0), (256, 256), &mut ferris);
    loop {}
}
