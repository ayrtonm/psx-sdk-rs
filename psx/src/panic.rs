use crate::hw::{cop0, Register};
use crate::{dprintln, println, Framebuffer};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cop0::Status::new().critical_section(|_| {
        // SAFETY: We're in a critical section so no other threads can get a
        // mutable reference. The previous (nested) call to panic, if any,
        // that accesses IN_PANIC won't hold onto a reference since we drop
        // them before calling display_panic.
        static mut IN_PANIC: bool = false;
        unsafe {
            if IN_PANIC {
                println!("Panicked in panic!");
            }
            IN_PANIC = true;
        }
        display_panic(info);
        loop {}
    })
}

fn display_panic(info: &core::panic::PanicInfo) {
    // Print to stdout unless no_panic is set. This includes the default case since
    // printing to the screen during a panic is not always reliable.
    #[cfg(not(feature = "no_panic"))]
    min_panic(info);
    // In the default case print the panic message to the screen
    #[cfg(not(any(feature = "min_panic", feature = "no_panic")))]
    normal_panic(info);
}

fn min_panic(info: &core::panic::PanicInfo) {
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
}

fn normal_panic(info: &core::panic::PanicInfo) {
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
        fb.draw_sync();
        fb.wait_vblank();
        fb.swap();
    }
}
