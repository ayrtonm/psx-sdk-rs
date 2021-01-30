use core::panic::PanicInfo;

fn message<'a>(info: &'a PanicInfo) -> &'a [u8] {
    let default_msg = b"Panic message contained formatted arguments\0";
    info.message()
        .map(|msg| msg.as_str().map(|msg| msg.as_bytes()))
        .flatten()
        .unwrap_or(default_msg)
}

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(info: &PanicInfo) -> ! {
    use crate::framebuffer::Framebuffer;
    use crate::gpu::{Depth, NTSC};
    use crate::printer::Printer;

    let zero = (0, 0);
    let res = (320, 240);
    let buf0 = zero;
    let buf1 = (0, 240);

    let mut fb = Framebuffer::initialized(buf0, buf1, res, None, NTSC, Depth::High, false);
    let mut pr = Printer::new(zero, zero, res, None);
    pr.load_font();
    match info.location() {
        Some(location) => {
            pr.print(b"Panicked at ", []);
            pr.print(location.file().as_bytes(), []);
            pr.println(b":{}:{}", [location.line(), location.column()]);
        },
        None => pr.println(b"Panicked at unknown location", []),
    }
    pr.println(message(info), []);
    fb.swap();
    loop {}
}

// This usually prints some trash in addition to panic info
// There's no easy fix for this w/o heap allocation which may make it less
// flexible
#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            printf!(b"Panicked at \0");
            printf!(location.file().as_bytes());
            printf!(b":%d:%d\n\0", location.line(), location.column());
        },
        None => printf!(b"Panicked at unknown location\n\0"),
    }
    printf!(message(info));
    printf!(b"\n\0");
    loop {}
}
