#![cfg(test)]

pub fn runner(tests: &[&dyn Fn()]) {
    printf!(b"Running all test...\n\0");
    for (n, test) in tests.iter().enumerate() {
        test();
        printf!(b"Passed test %d\n\0", n as u32);
    }
    printf!(b"Passed all tests!\n\0");
    loop {}
}
