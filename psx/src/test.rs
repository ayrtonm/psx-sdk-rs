#![cfg(test)]
use crate::bios;
use crate::bios::fs::{File, FileTy, MemCard, SeekFrom};
use crate::bios::kernel;
use crate::hal::{MutRegister, Mutable, Register, GP1, I_MASK, I_STAT};
use core::any::type_name;
use core::convert::TryInto;
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

pub fn runner(tests: &[&dyn Test]) {
    if cfg!(not(feature = "deterministic_test_runner")) {
        // Initialize RNG seed
        unsafe {
            kernel::init_card(true);
            kernel::start_card();
        }
        let file_name = "bu00:\\SEED";
        let mut buf = [0; MemCard::SECTOR_SIZE];
        let (mut file, seed) = match File::open(file_name) {
            Ok(file) => {
                // Read previous seed
                file.read(&mut buf).ok();
                // Unwrap will not fail because subslice contains exactly 4 indices
                let seed = unsafe { u32::from_ne_bytes(buf[0..4].try_into().unwrap_unchecked()) };
                (file, seed)
            },
            Err(_) => {
                let file =
                    File::create(file_name, None).expect("Unable to create memory card file");
                let initial_seed = 0xdead_beef;
                (file, initial_seed)
            },
        };
        bios::srand(seed);
        // Compute seed for the next run
        let next_seed =
            bios::rand() as u32 | ((bios::rand() as u32) << 15) | ((bios::rand() as u32) << 30);
        // Write next seed to the start of the file
        buf[0..4].copy_from_slice(&next_seed.to_ne_bytes());
        file.seek(SeekFrom::Start(0)).ok();
        file.write(&buf).ok();
        unsafe {
            kernel::stop_card();
        }
    }

    // Compute parameters for randomly iterating through tests
    let num_tests = tests.len();
    // Expand random numbers to 32 bits to avoid overflow when calculating index
    let (a, b) = if cfg!(not(feature = "deterministic_test_runner")) {
        let b = bios::rand() as usize % num_tests;
        let mut a = bios::rand() as usize;
        // This ensures that step size `a` and the number of tests are coprime
        // TODO: Will this always terminate?
        while gcd(a, num_tests) != 1 {
            a = bios::rand() as usize;
        }
        (a, b)
    } else {
        (0, 0)
    };
    // All tests will run assuming no overflow happens when computing the random
    // indices. The sanity check stores the sum of all indices to detect most of
    // these subtle overflow bugs in the test runner
    let mut sanity_check: usize = 0;

    println!("running {} tests", tests.len());
    for n in 0..num_tests {
        bios::critical_section(|| {
            I_MASK::<Mutable>::load().disable_all().store();
            I_STAT::<Mutable>::load().ack_all().store();
            GP1.reset_gpu();
            let idx = if cfg!(not(feature = "deterministic_test_runner")) {
                let idx = (a * n + b) % num_tests;
                sanity_check += idx;
                idx
            } else {
                n
            };
            tests[idx].run();
        });
    }
    if cfg!(not(feature = "deterministic_test_runner")) {
        assert!(
            sanity_check == (num_tests * (num_tests - 1)) / 2,
            "Sum of random indices differs from expected value, re-run tests without randomization"
        );
    }
    // Failing tests panic and unwinding will add unnecessary bloat to the binary
    // so the following line is only displayed if all tests pass
    println!("\ntest result: ok. {} passed; 0 failed\n", tests.len());
}
