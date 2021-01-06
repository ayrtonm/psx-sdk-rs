use psx::dma;
use psx::framebuffer::Framebuffer;
use psx::general::*;
use psx::global;
use psx::printer::{Printer, MIN_SIZE};
use psx::{load_font, println};

#[allow(dead_code)]
const RUN_CONST_TESTS: () = {
    use super::CONST_TESTS;

    let mut i = 0;
    while i < CONST_TESTS.len() {
        if !CONST_TESTS[i] {
            panic!("Failed a const test");
        }
        i += 1;
    }
};

#[no_mangle]
fn main(mut gpu_dma: dma::gpu::CHCR) -> ! {
    reset_graphics(&mut gpu_dma);
    let _fb = Framebuffer::new((0, 0), (0, 240), (320, 240), None, &mut gpu_dma);
    enable_display();
    load_font!();

    println!(b"Running tests...");
    for &t in &super::TESTS {
        run_test(t);
    }
    loop {}
}

fn run_test(f: fn() -> bool) {
    let msg = if f() {
        b"Passed test {}"
    } else {
        b"Failed test {}"
    };
    static mut TEST_NUM: u32 = 0;
    unsafe {
        TEST_NUM += 1;
    }
    println!(msg, unsafe { TEST_NUM });
}
