use core::cell::UnsafeCell;
use core::lazy::Lazy;

use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::general::*;
use psx::printer::{Printer, MIN_SIZE};

struct Static<T>(Lazy<UnsafeCell<T>>);
unsafe impl<T> Sync for Static<T> {}
impl<T> Static<T> {
    pub const fn new(f: fn() -> UnsafeCell<T>) -> Self {
        Static(Lazy::new(f))
    }
    pub fn get(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
static PRINTER: Static<Printer<MIN_SIZE>> = Static::new(|| {
    let mut printer = Printer::new(0, 0, (320, 240), None);
    unsafe { printer.load_font(&mut dma::gpu::CHCR::new()) };
    UnsafeCell::new(printer)
});

macro_rules! print {
    ($msg:expr) => {
        PRINTER
            .get()
            .print($msg, [], &mut unsafe { dma::gpu::CHCR::new() });
    };
    ($msg:expr, $arg0:expr) => {
        PRINTER
            .get()
            .print($msg, [$arg0], &mut unsafe { dma::gpu::CHCR::new() });
    };
}

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) -> ! {
    reset_graphics(&mut gpu_dma);
    let _fb = Framebuffer::new((0, 0), (0, 240), (320, 240), None, &mut gpu_dma);
    enable_display();

    print!(b"\nRunning tests...\n");
    for &t in &super::TESTS {
        run_test(t);
    }
    loop {}
}

fn run_test(f: fn() -> bool) {
    let msg = if f() {
        b"Passed test {}\n"
    } else {
        b"Failed test {}\n"
    };
    static mut TEST_NUM: u32 = 0;
    unsafe {
        TEST_NUM += 1;
    }
    print!(msg, unsafe { TEST_NUM });
}
