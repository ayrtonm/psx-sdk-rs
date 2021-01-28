#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }
    main()
}
