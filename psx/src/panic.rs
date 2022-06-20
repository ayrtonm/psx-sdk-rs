use crate::{dprintln, println, Framebuffer};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if cfg!(feature = "min_panic") {
        match info.location() {
            Some(location) => {
                println!(
                    "Panicked at {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                )
            },
            None => {
                println!("Panicked at unknown location")
            },
        }
        if let Some(msg) = info.message() {
            println!("{}", msg)
        }
    } else if cfg!(not(feature = "no_panic")) {
        // We have no idea what state the GPU was in when the panic happened, so reset
        // it to a known state and reload the font into VRAM.
        let mut fb = Framebuffer::default();
        let mut txt = fb.load_default_font().new_text_box((0, 8), (320, 240));
        loop {
            txt.reset();
            match info.location() {
                Some(location) => {
                    dprintln!(
                        txt,
                        "Panicked at {}:{}:{}",
                        location.file(),
                        location.line(),
                        location.column()
                    );
                },
                None => {
                    dprintln!(txt, "Panicked at unknown location");
                },
            }
            if let Some(msg) = info.message() {
                dprintln!(txt, "{}", msg);
            }
            fb.draw_sync();
            fb.wait_vblank();
            fb.swap();
        }
    }
    // Both min_panic and no_panic end execution here
    loop {}
}
