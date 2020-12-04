use crate::framebuffer::UnsafeFramebuffer;
use crate::mmio::gpu;
use crate::printer::UnsafePrinter;
use core::panic::PanicInfo;

#[panic_handler]
//#[cfg(feature = "pretty_panic")]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut gp0 = unsafe { gpu::GP0::new() };

    let mut printer = UnsafePrinter::<1024>::default();
    let mut fb = UnsafeFramebuffer::default();

    // TODO: I should probably uncomment this for ePSXe, or better yet put it in
    // Printer somehow dma_control.gpu(true).otc(true);
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
    printer.print(b"hello".iter().map(|c| *c as u8));
    printer.print(x);
    fb.swap();
    loop {}
}

//#[panic_handler]
//#[cfg(not(feature = "pretty_panic"))]
//fn panic(panic_info: &PanicInfo) -> ! {
//    loop {}
//}
