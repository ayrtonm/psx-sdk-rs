use crate::framebuffer::{Framebuffer, UnsafeFramebuffer};
use crate::gpu::color::Color;
use crate::mmio::{dma, gpu};
use crate::printer::{Printer, UnsafePrinter};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let (mut gp0, mut gp1, mut otc_dma, mut gpu_dma, mut dma_control) = unsafe {
        (
            gpu::GP0::new(),
            gpu::GP1::new(),
            dma::otc::Channel::new(),
            dma::gpu::Channel::new(),
            dma::Control::new(),
        )
    };

    let mut printer = UnsafePrinter::<1024>::new((0, 0), (8, 16), (0, 0), (320, 240), Color::WHITE);
    let mut fb = UnsafeFramebuffer::new((0, 0), (0, 240), (320, 240));

    dma_control.gpu(true).otc(true);
    // TODO: Why is the Framebuffer constructor not taking care of this?
    gp0.start((0, 0)).end((320, 240)).offset((0, 0));
    printer.load_font();

    // TODO: fix and test alloc to get better messages
    let s = &panic_info
        .message()
        .unwrap()
        .as_str()
        .unwrap_or("panic msg contained formatted arguments");
    let x = s.chars().map(|c| c as u32 as u8);
    //printer.print(b"hello".iter());
    printer.print(x);
    fb.swap();
    loop {}
}
