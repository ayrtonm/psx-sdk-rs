#![no_std]
#![no_main]

use core::any::Any;
use core::mem::size_of_val;

use psx::mmio::{int, MMIO};

use psx::framebuffer::UnsafeFramebuffer;
use psx::printer::UnsafePrinter;

use psx::gpu::{Color, Vertex};
use psx::interrupt::IRQ;

#[no_mangle]
fn main(mut mmio: MMIO) {
    let mut p = UnsafePrinter::<1024>::default();
    let mut f = UnsafeFramebuffer::default();
    p.load_font();
    run_tests(&mut mmio);
    p.print(b"All tests passed", []);
    f.swap();
}

fn run_tests(mmio: &mut MMIO) {
    check_sizes(mmio);
    test_int_mask(&mut mmio.int_mask);
}

fn check_sizes(mmio: &mut MMIO) {
    let value_sizes = [
        (mmio as &dyn Any, 0),
        (&Vertex::from(0) as &dyn Any, 4),
        (&Color::BLUE as &dyn Any, 3),
    ];
    for (val, size) in &value_sizes {
        assert!(size_of_val(*val) == *size);
    }
}

fn test_int_mask(int_mask: &mut int::Mask) {
    int_mask.disable_all();
    int_mask.enable(IRQ::Vblank);
    let mut enabled = int_mask.enabled();
    assert!(enabled.next() == Some(IRQ::Vblank));
    assert!(enabled.next().is_none());
}
