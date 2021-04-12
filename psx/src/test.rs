#![cfg(test)]
use crate::bios;
use crate::hal::{MutRegister, Mutable, Register, GP1, I_MASK, I_STAT};
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
    for test in tests {
        bios::critical_section(|| {
            I_MASK::<Mutable>::load().disable_all().store();
            I_STAT::<Mutable>::load().ack_all().store();
            GP1.reset_gpu();
            test.run();
        });
    }
    // Failing tests panic and unwinding will add unnecessary bloat to the binary
    // so the following line is only displayed if all tests pass
    printf!("\ntest result: ok. %d passed; 0 failed\n\n\0", tests.len());
}
