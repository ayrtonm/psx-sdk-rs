use core::panic::PanicInfo;

#[panic_handler]
#[cfg(feature = "pretty_panic")]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[panic_handler]
#[cfg(not(feature = "pretty_panic"))]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
