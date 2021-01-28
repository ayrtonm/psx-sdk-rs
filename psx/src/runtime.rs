#[cfg(not(test))]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[cfg(test)]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    crate::test_main();
    loop {}
}
