#![cfg(test)]

use crate::{print, println};
use const_random::const_random;
use core::any::type_name;
use num::integer::gcd;

#[macro_export]
macro_rules! fuzz {
    (|$($name:ident: $ty:ty),+| { $($body:tt)* }) => {
        use const_random::const_random;
        const MAX_TESTS: usize = 1_000;
        let mut rng = crate::sys::Rng::new(const_random!(u32));
        for _ in 0..const_random!(usize) % MAX_TESTS {
            $(let $name = rng.rand() as $ty;)*
            $($body)*
        }
    };
}

pub trait Test {
    fn run(&self);
}

fn index_params<const N: usize>() -> (usize, usize) {
    let b = const_random!(usize) % N;
    let mut a = const_random!(usize);
    while gcd(a, N) != 1 {
        a = a / gcd(a, N);
    }
    println!("Generated test parameters {} and {}", a % N, b);
    (a % N, b)
}

impl<T: Fn()> Test for T {
    fn run(&self) {
        print!("test {} ...", type_name::<Self>());
        self();
        println!("ok");
    }
}

pub fn runner<const N: usize>(tests: &[&dyn Test; N]) {
    let (a, b) = if cfg!(not(feature = "deterministic_test_runner")) {
        index_params::<N>()
    } else {
        (0, 0)
    };

    let mut idx_count = 0;

    println!("running {} tests", N);
    for n in 0..N {
        let idx = if cfg!(not(feature = "deterministic_test_runner")) {
            (a * n + b) % N
        } else {
            n
        };
        idx_count += idx;
        tests[idx].run();
    }
    if cfg!(not(feature = "deterministic_test_runner")) {
        assert!(
            idx_count == (N * (N - 1)) / 2,
            "Panicked due to error in test framework!"
        );
    }
    println!("\ntest result: ok. {} passed; 0 failed\n", N);
    loop {}
}

#[test_case]
fn sanity_check() {
    assert!(true);
}
