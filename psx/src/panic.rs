use crate::framebuffer::UnsafeFramebuffer;
use crate::mmio::gpu;
use crate::printer::UnsafePrinter;
use core::panic::PanicInfo;

// TODO: Add some feature flag for cargo-psx --no-alloc
//extern crate alloc;
//use crate::panic::alloc::string::ToString;

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
    // Based on an extremely rough analysis using --release, --lto and --no-pad...
    // 6143 - 2275 = 3868B
    // 2660B of this is just font.tim.zip
    // That still leaves 1208B though
    printer.load_font();

    // TODO: fix and test alloc to get better messages
    // I cannot stress enough how rough this analysis is, but it does seem like
    // this is the heaviest part of this panic handler
    // 10354 - 6143 = 4211B
    let s = &panic_info
        .message()
        .unwrap()
        // TODO: See `extern crate alloc;` above
        //.to_string();
        .as_str()
        .unwrap_or("panic msg contained formatted arguments");
    // 11210 - 10354 = 856B
    printer.print(s.as_bytes(), []);
    fb.swap();
    loop {}
}

//#[panic_handler]
//#[cfg(not(feature = "pretty_panic"))]
//fn panic(panic_info: &PanicInfo) -> ! {
//    loop {}
//}
