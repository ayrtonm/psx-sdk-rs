use core::panic::PanicInfo;

#[cfg(feature = "pretty_panic")]
use crate::{init_graphics, load_font, print};

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(panic_info: &PanicInfo) -> ! {
    init_graphics!();
    load_font!();
    if let Some(msg) = panic_info.message() {
        let msg = if let Some(msg) = msg.as_str() {
            msg.as_bytes()
        } else {
            b"Panic message contained formatted arguments"
        };
        print!(msg);
    };
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
