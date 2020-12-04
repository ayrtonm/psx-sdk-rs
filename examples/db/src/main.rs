#![no_std]
#![no_main]

use psx::framebuffer::UnsafeFramebuffer;
use psx::printer::UnsafePrinter;
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
        printer.print(b"world".iter().map(|c| *c as u8));
    } else {
        printer.print(b"hello".iter().map(|c| *c as u8));
    };
    fb.swap();
    loop {}
}
