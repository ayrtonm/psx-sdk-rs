extern crate image;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use image::{RgbImage, Pixel};

fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 3 {
        println!("Usage: {} <vram-dump> <out-file>", argv[0]);
        return;
    }

    let mut raw_file = File::open(&argv[1]).unwrap();

    let mut raw = Vec::new();

    raw_file.read_to_end(&mut raw).unwrap();

    let mut img = RgbImage::new(XRES, YRES);

    for (x, y, p) in img.enumerate_pixels_mut() {
        let offset = (y * XRES + x) * 2;

        let offset = offset as usize;

        let b1 = raw[offset];
        let b2 = raw[offset + 1];
        
        let pixel = (b1 as u16) | ((b2 as u16) << 8);

        let r = (pixel & 0x1f) << 3;
        let g = ((pixel >> 5) & 0x1f) << 3;
        let b = ((pixel >> 10) & 0x1f) << 3;

        p.channels_mut()[0] = r as u8;
        p.channels_mut()[1] = g as u8;
        p.channels_mut()[2] = b as u8;
    }

    img.save(Path::new(&argv[2])).unwrap();
}

const XRES: u32 = 1024;
const YRES: u32 = 512;
