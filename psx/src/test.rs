#![cfg(test)]
use crate::bios;
use crate::hal::{MutRegister, Mutable, Register, GP1, I_MASK, I_STAT};
use const_random::const_random;
use core::any::type_name;
use num::integer::gcd;

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

/// Returns parameters (a, b) where a is coprime with NUM_TESTS and both are
/// smaller than NUM_TESTS.
fn idx_params<const NUM_TESTS: usize>() -> (usize, usize) {
    let b = const_random!(usize) % NUM_TESTS;
    let mut a = const_random!(usize);
    while gcd(a, NUM_TESTS) != 1 {
        // Invoking const_random! in a loop would produce the same number even if
        // this were a const fn assured to be evaluated at compile-time
        a = a / gcd(a, NUM_TESTS);
    }
    (a % NUM_TESTS, b)
}

pub fn runner<const NUM_TESTS: usize>(tests: &[&dyn Test; NUM_TESTS]) {
    // Compute parameters for randomly iterating through tests
    let (a, b) = if cfg!(not(feature = "deterministic_test_runner")) {
        idx_params::<NUM_TESTS>()
    } else {
        (0, 0)
    };
    // Sanity check to ensure that randomizing tests doesn't run a given test more
    // than once
    let mut idx_count: usize = 0;

    println!("running {} tests", NUM_TESTS);
    for n in 0..NUM_TESTS {
        bios::critical_section(|| {
            I_MASK::<Mutable>::load().disable_all().store();
            I_STAT::<Mutable>::load().ack_all().store();
            GP1.reset_gpu();
            let idx = if cfg!(not(feature = "deterministic_test_runner")) {
                (a * n + b) % NUM_TESTS
            } else {
                n
            };
            idx_count += idx;
            tests[idx].run();
        });
    }
    if cfg!(not(feature = "deterministic_test_runner")) {
        assert!(
            idx_count == (NUM_TESTS * (NUM_TESTS - 1)) / 2,
            "Error detected in test framework! Double check test runner or\
             re-run tests without randomization"
        );
    }
    // Failing tests panic and unwinding will add unnecessary bloat to the binary
    // so the following line is only displayed if all tests pass
    println!("\ntest result: ok. {} passed; 0 failed\n", NUM_TESTS);
}
