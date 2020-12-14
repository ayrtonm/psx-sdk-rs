use crate::framebuffer::UnsafeFramebuffer;
use crate::mmio::gpu;
use crate::printer::UnsafePrinter;
use core::panic::PanicInfo;

// TODO: Add some feature flag for cargo-psx --no-alloc
//extern crate alloc;
//use crate::panic::alloc::string::ToString;

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut gp0 = unsafe { gpu::GP0::new() };

    let mut printer = UnsafePrinter::<1024>::default();
    let mut fb = UnsafeFramebuffer::default();

    // TODO: Why is the Framebuffer constructor not taking care of this?
    gp0.start(0).end((320, 240)).offset(0);
    printer.load_font();

    let s = &panic_info
        .message()
        .unwrap()
        // TODO: See `extern crate alloc;` above
        //.to_string();
        .as_str()
        .unwrap_or("panic msg contained formatted arguments");
    printer.print(s.as_bytes(), []);
    fb.swap();
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(panic_info: &PanicInfo) -> ! {
    loop {}
}
