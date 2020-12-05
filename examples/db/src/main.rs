#![no_std]
#![no_main]

use psx::framebuffer::UnsafeFramebuffer;
use psx::printer::{Printer, UnsafePrinter};
use psx::gpu::prim::DoubleBuffer;
use psx::gpu::Color;

psx::exe!();

fn main(mut mmio: MMIO) {
    let mut fb = UnsafeFramebuffer::default();
    let mut buffer = DoubleBuffer::<100>::new();
    let mut sprt = buffer.Sprt().unwrap();
    let a = sprt.packet().color(Color::GREEN);
    let c = a.packet().color.green;
    buffer.swap();
    let b = sprt.packet();

    let mut printer = UnsafePrinter::<256>::default();
    printer.load_font();
    if c == b.packet().color.green {
        printer.print(b"colors are the same", []);
    } else {
        printer.print(b"colors differ", []);
    };
    printer.print(b"\nhello {} {{ } {0}", [0x00ad_beef, 0x00cd_1234]);
    fb.swap();
    loop {}
}
