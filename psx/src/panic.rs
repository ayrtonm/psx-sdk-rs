use core::panic::PanicInfo;

fn message<'a>(info: &'a PanicInfo) -> Option<&'a [u8]> {
    info.message().map(|msg| match msg.as_str() {
        Some(msg) => msg.as_bytes(),
        None => b"Panic message contained formatted arguments",
    })
}

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(info: &PanicInfo) -> ! {
    use crate::framebuffer::Framebuffer;
    use crate::gpu::reset_graphics;
    use crate::gpu::{Depth, VideoMode};
    use crate::printer::Printer;

    let mut fb = Framebuffer::new((0, 0), (0, 240), (320, 240), None);
    reset_graphics((320, 240), VideoMode::NTSC, Depth::High, false);
    let mut printer = Printer::new((0, 0), (0, 0), (320, 240), None);
    printer.load_font();
    message(info).map(|msg| {
        printer.reset();
        printer.print(msg, []);
        fb.swap();
    });
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(info: &PanicInfo) -> ! {
    message(info).map(|msg| crate::bios::printf(msg.as_ptr(), 0));
    loop {}
}
