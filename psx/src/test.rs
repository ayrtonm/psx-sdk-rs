#![cfg(test)]
use crate::bios;
use crate::std::AsCStr;
use core::any::type_name;

pub trait Test {
    fn run(&self);
}

impl<T: Fn()> Test for T {
    fn run(&self) {
        type_name::<Self>().as_cstr(|s| printf!("test %s ... \0", s));
        self();
        printf!("ok\n\0");
    }
}

pub fn runner(tests: &[&dyn Test]) {
    printf!("running %d tests\n\0", tests.len());
    bios::critical_section(|| {
        for test in tests {
            test.run();
        }
    });
    // Failing tests panic and unwinding will add unnecessary bloat to the binary
    // so the following line is only displayed if all tests pass
    printf!("\ntest result: ok. %d passed; 0 failed\n\n\0", tests.len());
    loop {}
}

#[test_case]
fn pass() {
    assert!(true);
}
