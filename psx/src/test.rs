#![cfg(test)]
use crate::bios;
use crate::hal::{MutRegister, Mutable, Register, GP1, I_MASK, I_STAT};
use core::any::type_name;

pub trait Test {
    fn run(&self);
}

impl<T: Fn()> Test for T {
    fn run(&self) {
        print!("test {} ... ", type_name::<Self>());
        self();
        println!("ok");
    }
}

pub fn runner(tests: &[&dyn Test]) {
    println!("running {} tests", tests.len());
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
    println!("\ntest result: ok. {} passed; 0 failed\n", tests.len());
}
