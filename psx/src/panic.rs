use core::panic::PanicInfo;

fn message<'a>(info: &'a PanicInfo) -> &'a [u8] {
    let default_msg = b"Panic message contained formatted arguments\0";
    // TODO: Use core::fmt::Write
    //println!("{}", info.message().unwrap());
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

    let mut fb = Framebuffer::new(buf0, buf1, res, None, NTSC, Depth::High, false);
    let mut pr = Printer::new(zero, zero, res, None);
    pr.load_font();
    match info.location() {
        Some(location) => {
            pr.print("Panicked at ", []);
            pr.print(location.file().as_bytes(), []);
            pr.println(":{}:{}", [location.line(), location.column()]);
        },
        None => pr.println("Panicked at unknown location", []),
    }
    pr.println(message(info), []);
    fb.swap(None);
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(info: &PanicInfo) -> ! {
    use crate::std::AsCStr;

    match info.location() {
        Some(location) => {
            printf!("Panicked at \0");
            location.file().as_cstr(|s| printf!(s));
            printf!(":%d:%d\n\0", location.line(), location.column());
        },
        None => printf!("Panicked at unknown location\n\0"),
    }
    message(info).as_cstr(|s| printf!(s));
    printf!("\n\0");
    loop {}
}
