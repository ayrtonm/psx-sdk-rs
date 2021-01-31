#![no_std]
// The psx crate defines a `_start` function which serves as an entry point and calls our `main`
// Adding the `no_main` attribute tells the rust compiler to not use the normal entry point
#![no_main]

use psx::printf;

// Unmangling the `main` function is required to call it from `_start`
// Luckily forgetting this attribute will cause the linker throw an error instead of failing silently
#[no_mangle]
fn main() {
    // This macro calls the BIOS printf which requires null-terminated strings.
    // Non-null-terminated strings might be displayed along with some trash.
    // Mednafen and duckstation both show printf messages in stdout with the correct settings.
    // Mednafen needs `psx.dbg_level` set to 2 or greater.
    // Duckstation needs TTY output enabled and the log level set to `Information` or greater.
    printf!("Hello, world!\n\0");
    // Panic use `printf` by default so it requires null-termination.
    // With the psx/pretty_panic feature, panic messages are shown on screen
    // which supports, but does not require null-termination.
    panic!("Ran into some error\0");
}
