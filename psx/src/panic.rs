use crate::println;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if cfg!(not(feature = "min_panic")) {
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
    }
    loop {}
}
