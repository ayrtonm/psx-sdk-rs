use core::panic::PanicInfo;

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(panic_info: &PanicInfo) -> ! {
    use crate::dma;
    use crate::framebuffer::Framebuffer;
    use crate::general::{enable_display, reset_graphics};
    use crate::gpu::Color;
    use crate::printer::Printer;

    let gpu_dma = &mut unsafe { dma::gpu::CHCR::new() };
    reset_graphics(gpu_dma);
    let mut printer = Printer::new(0, 0, (320, 240), None);
    Framebuffer::new(0, (0, 240), (320, 240), Some(Color::BLACK), gpu_dma);
    printer.load_font(gpu_dma);
    enable_display();
    if let Some(msg) = panic_info.message() {
        if let Some(msg) = msg.as_str() {
            printer.print(msg.as_bytes(), [], gpu_dma);
        } else {
            printer.print(b"Panic message contained formatted arguments", [], gpu_dma);
        }
    };
    // TODO: Why is the printer writing to the top buffer?
    //fb.swap(gpu_dma);
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
