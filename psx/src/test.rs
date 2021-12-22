#![cfg(test)]

use crate::{print, println};
use const_random::const_random;
use core::any::type_name;
use num::integer::gcd;

pub const MAX_TESTS: usize = 1_000;

#[macro_export]
macro_rules! fuzz {
    (|$($name:ident: $ty:ty),+| { $($body:tt)* }) => {
        {
            use const_random::const_random;
            use crate::sys::rng::Rng;

            let mut rng = Rng::new(const_random!(u32));
            for _ in 0..crate::test::MAX_TESTS {
                $(let $name = rng.rand::<$ty>();)*
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! fuzz_data {
    (|$name:ident: &[$ty:ty]| { $($body:tt)* }) => {
        {
            use const_random::const_random;
            use crate::sys::rng::Rng;

            const MAX_SIZE: usize = 1_000;
            const SIZE: usize = const_random!(usize) % MAX_SIZE;
            let mut rng = Rng::new(const_random!(u32));
            for _ in 0..crate::test::MAX_TESTS {
                let mut ar: [$ty; SIZE] = [0; SIZE];
                for n in 0..SIZE {
                    ar[n] = rng.rand::<$ty>();
                }
                let $name = &ar;
                $($body)*
            }
        }
    };
}

pub trait Test {
    fn run(&self);
}

fn index_params(n: usize) -> (usize, usize) {
    let b = const_random!(usize) % n;
    let mut a = const_random!(usize) % n;
    if a == 0 {
        a = 1;
    }
    while gcd(a, n) != 1 {
        a = a / gcd(a, n);
    }
    (a, b)
}

#[test_case]
fn test_params() {
    fuzz!(|num_tests: usize| {
        let (a, b) = index_params(num_tests);
        assert!(a != 0);
        assert!(a < num_tests);
        assert!(b < num_tests);
        assert!(gcd(a, num_tests) == 1);
    });
}

impl<T: Fn()> Test for T {
    fn run(&self) {
        print!("test {} ...", type_name::<Self>());
        self();
        println!("ok");
    }
}

pub fn runner<const N: usize>(tests: &[&dyn Test; N]) {
    let (a, b) = index_params(N);

    let mut executed_tests = [0; N];

    println!("running {} tests", N);
    for n in 0..N {
        let idx = (a * n + b) % N;
        tests[idx].run();
        executed_tests[n] = idx;
    }
    executed_tests.as_mut_slice().sort_unstable();
    let ran_all_tests = executed_tests.iter().cloned().eq(0..N);
    assert!(ran_all_tests, "Test framework failed to run all tests!");
    println!("\ntest result: ok. {} passed; 0 failed\n", N);
    loop {}
}

#[test_case]
fn sanity_check() {
    assert!(true);
}
