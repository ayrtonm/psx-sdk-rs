use crate::framebuffer::UnsafeFramebuffer;
use crate::mmio::gpu;
use crate::printer::UnsafePrinter;
use core::panic::PanicInfo;

#[panic_handler]
//#[cfg(feature = "pretty_panic")]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut gp0 = unsafe { gpu::GP0::new() };

    let mut printer = UnsafePrinter::<1024>::new((0, 0), (8, 16), (0, 0), (320, 240), None);
    let mut fb = UnsafeFramebuffer::new((0, 0), (0, 240), (320, 240));

    //dma_control.gpu(true).otc(true);
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

//#[panic_handler]
//#[cfg(not(feature = "pretty_panic"))]
//fn panic(panic_info: &PanicInfo) -> ! {
//    loop {}
//}
