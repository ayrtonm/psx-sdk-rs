#![no_std]
#![no_main]
// This makes sure that const test failures prevent compilation
#![deny(const_err)]
#![feature(const_panic)]
#![feature(const_fn_floating_point_arithmetic)]

use core::mem::size_of;

use psx::gte::f16::f16;
use psx::gte::trig::{cos, sin, fsin, fcos};
use psx::bios;
use psx::compatibility::*;
use psx::dma;
use psx::gpu;
use psx::graphics::buffer::Buffer;
use psx::graphics::packet::Packet;
use psx::graphics::InitPrimitive;
use psx::irq;
use psx::mmio::Address;
use psx::printer::format_u32;
use psx::timer;
use psx::value::Load;
use psx::{include_u32, unzip};
use psx::println;

use const_fn::{cmp_u32, cmp_u8};
use const_fn::{pi, e};

// This provides a `print` macro for debugging
#[macro_use]
mod framework;

// Provides workarounds for const testing.
mod const_fn;

// Tests simply return true is they pass or false if they fail.
type TestResult = bool;
type Test = fn() -> TestResult;

// These are runtime tests to be evaluated by the emulator.
const TESTS: [Test; 4] = [
    test_1,
    test_2,
    buffer,
    trig_tests,
];
// These are compile-time tests to be evaluated by the compiler.
const CONST_TESTS: [TestResult; 4] = [
    mmio_addresses(),
    format_u32_tests(),
    unzip_test(),
    f16_tests(),
];

fn test_1() -> bool {
    bios::gpu_get_status() == gpu::GPUSTAT.load().bits
}

// Make sure that tests undo any changes to global state (e.g. exit a critical
// section before returning)
fn test_2() -> bool {
    CriticalSection(|| bios::enter_critical_section() == 0)
}

fn trig_tests() -> bool {
    assert!(sin(0.0) == 0.0);
    assert!(sin(90.0) == 1.0);
    assert!(fsin(0.0) == f16(0.0));
    println!(b"sin(0) = {} and expected {}\n\
             sin(pi/6) = {} and expected {}\n\
             sin(pi/4)    = {} and expected {}\n\
             sin(pi/3) = {} and expected {}\n\
             sin(pi/2)   = {} and expected {}",
             fsin(0.00).to_bits() as u32,
             f16(0.0).to_bits() as u32,
             fsin(0.33333333333).to_bits() as u32,
             f16(0.5).to_bits() as u32,
             fsin(0.50).to_bits() as u32,
             f16(0.70710678118654752440).to_bits() as u32,
             fsin(0.66666666666).to_bits() as u32,
             f16(0.86602540378443864676).to_bits() as u32,
             fsin(1.00).to_bits() as u32,
             f16(1.0).to_bits() as u32);
    fsin(0.0) == f16(0.0) &&
    fsin(0.3333333333) == f16(0.5) &&
    //fsin(0.5) == f16(0.0) && // 1/sqrt(2) as f16
    //fsin(0.6666666666) == f16(0.0) && // sqrt(3)/2 as f16
    fsin(1.0) == f16(1.0) &&
    true
}

const fn f16_tests() -> bool {
    let pos_values = [pi, e, 0.0, 1.0, 7.0];
    let trunc_pos_values = [3, 2, 0, 1, 7];
    let neg_values = [-pi, -1.0, -7.0, -8.0];
    let trunc_neg_values = [-4, -1, -7, -8];
    const fn within_epsilon(x: f32) -> bool {
        -f16::EPSILON < x && x < f16::EPSILON
    }
    let mut i = 0;
    while i < pos_values.len() {
        let fp = f16(pos_values[i]);
        assert!(within_epsilon(fp.as_f32() - pos_values[i]));
        assert!(!fp.is_negative());
        assert!(fp.trunc() == trunc_pos_values[i]);
        assert!(fp.fract() == f16(pos_values[i] - trunc_pos_values[i] as f32).fract());
        i += 1;
    }
    let mut i = 0;
    while i < neg_values.len() {
        let fp = f16(neg_values[i]);
        assert!(within_epsilon(fp.as_f32() - neg_values[i]));
        assert!(fp.is_negative());
        assert!(fp.trunc() == trunc_neg_values[i]);
        assert!(fp.fract() == f16(neg_values[i] - (trunc_neg_values[i] as f32) - 1.0).fract());
        i += 1;
    }
    const fn trunc(x: f32) -> f32 {
        x - f16::precision_loss(x)
    }
    // Sanity checks
    assert!(f16(pi).as_f32() + f16(e).as_f32() == trunc(pi) + trunc(e));
    assert!(f16(pi).as_f32() - f16(e).as_f32() == trunc(pi) - trunc(e));
    // pi * e would overflow the 3 integer bits in f16, so let's test e^2
    assert!(f16(e).as_f32() * f16(e).as_f32() == trunc(e) * trunc(e));
    assert!(f16(pi).as_f32() / f16(e).as_f32() == trunc(pi) / trunc(e));

    // Checks addition
    let mut true_value = trunc(pi) + trunc(e);
    let mut approx_value = f16(pi).const_add(f16(e)).as_f32();
    assert!(approx_value == true_value);
    // Checks subtraction
    true_value = trunc(pi) - trunc(e);
    approx_value = f16(pi).const_sub(f16(e)).as_f32();
    assert!(approx_value == true_value);
    // Checks multiplication
    true_value = trunc(e) * trunc(e);
    approx_value = f16(e).const_mul(f16(e)).as_f32();
    assert!(within_epsilon(approx_value - true_value));
    // Checks division
    true_value = trunc(pi) / trunc(e);
    approx_value = f16(pi).const_div(f16(e)).as_f32();
    assert!(within_epsilon(approx_value - true_value));
    true
}

const fn unzip_test() -> bool {
    cmp_u32(&unzip!("../font.tim.zip"), &include_u32!("../font.tim"))
}

const fn format_u32_tests() -> bool {
    let mut hex = false;
    let mut leading = false;
    assert!(cmp_u8(&format_u32(29, leading, hex), b"29\0\0\0\0\0\0\0\0"));
    assert!(cmp_u8(&format_u32(0xFFFF_FFFF, leading, hex), b"4294967295"));
    assert!(cmp_u8(&format_u32(0, leading, hex), b"0\0\0\0\0\0\0\0\0\0"));
    hex = true;
    leading = false;
    assert!(cmp_u8(&format_u32(29, leading, hex), b"1Dh\0\0\0\0\0\0\0"));
    assert!(cmp_u8(&format_u32(0xFFFF_FFFF, leading, hex), b"FFFFFFFFh\0"));
    assert!(cmp_u8(&format_u32(0, leading, hex), b"0h\0\0\0\0\0\0\0\0"));
    hex = false;
    leading = true;
    assert!(cmp_u8(&format_u32(29, leading, hex), b"0000000029"));
    assert!(cmp_u8(&format_u32(0xFFFF_FFFF, leading, hex), b"4294967295"));
    assert!(cmp_u8(&format_u32(0, leading, hex), b"0000000000"));
    hex = true;
    leading = true;
    assert!(cmp_u8(&format_u32(29, leading, hex), b"0000001Dh\0"));
    assert!(cmp_u8(&format_u32(0xFFFF_FFFF, leading, hex), b"FFFFFFFFh\0"));
    assert!(cmp_u8(&format_u32(0, leading, hex), b"00000000h\0"));
    true
}

fn buffer() -> bool {
    struct X([u32; 8]);
    struct Y([u32; 2]);
    impl InitPrimitive for X {
        fn init_primitive(&mut self) {}
    }
    impl InitPrimitive for Y {
        fn init_primitive(&mut self) {
            self.0[0] = 0xFFFF_FFFF;
        }
    }

    const BUF: usize = size_of::<Packet<X>>() / 4;
    let mut buffer = Buffer::<BUF>::new();
    let initial_size = buffer.words_remaining();
    match buffer.alloc::<X>() {
        Some(x) => {
            // Checks that `words_remaining` works
            assert!(initial_size == BUF);
            assert!(buffer.words_remaining() == 0);
            // Checks that `X` is initialized as expected
            assert!(x.0 == [0; 8]);
            // Checks that the buffer is full
            assert!(buffer.alloc::<X>().is_none());
            assert!(buffer.alloc::<Y>().is_none());
            x.0 = [0, 0xDEAD_BEEF, 0, 0, 0, 0, 0, 0];
            buffer.empty();
            match buffer.alloc::<Y>() {
                Some(y) => {
                    // Checks that Y was initialized and contains the old data written to X
                    assert!(y.0 == [0xFFFF_FFFF, 0xDEAD_BEEF]);
                    true
                },
                // Checks that we can allocate a `Y` after emptying the buffer
                None => false,
            }
        },
        // Checks that we can allocate at least one `X`
        None => false,
    }
}

const fn mmio_addresses() -> bool {
    0x1F801810 == gpu::GP0::ADDRESS &&
    0x1F801814 == gpu::GP1::ADDRESS &&
    //0x1F801810 == gpu::GPUREAD::ADDRESS &&
    0x1F801814 == gpu::GPUSTAT::ADDRESS &&
    0x1F801070 == irq::ISTAT::ADDRESS &&
    0x1F801074 == irq::IMASK::ADDRESS &&
    //0x1F801100 == timer::timer0::CNT::ADDRESS &&
    //0x1F801104 == timer::timer0::MODE::ADDRESS &&
    //0x1F801108 == timer::timer0::TGT::ADDRESS &&
    0x1F801110 == timer::timer1::CNT::ADDRESS &&
    0x1F801114 == timer::timer1::MODE::ADDRESS &&
    0x1F801118 == timer::timer1::TGT::ADDRESS &&
    //0x1F801120 == timer::timer2::CNT::ADDRESS &&
    //0x1F801124 == timer::timer2::MODE::ADDRESS &&
    //0x1F801128 == timer::timer2::TGT::ADDRESS &&
    0x1F8010F0 == dma::DPCR::ADDRESS &&
    0x1F8010F4 == dma::DICR::ADDRESS &&
    //0x1F801080 == dma::MDECin::MADR::ADDRESS &&
    //0x1F801084 == dma::MDECin::BCR::ADDRESS &&
    //0x1F801088 == dma::MDECin::CHCR::ADDRESS &&
    //0x1F801090 == dma::MDECout::MADR::ADDRESS &&
    //0x1F801094 == dma::MDECout::BCR::ADDRESS &&
    //0x1F801098 == dma::MDECout::CHCR::ADDRESS &&
    0x1F8010A0 == dma::gpu::MADR::ADDRESS &&
    0x1F8010A4 == dma::gpu::BCR::ADDRESS &&
    0x1F8010A8 == dma::gpu::CHCR::ADDRESS &&
    //0x1F8010B0 == dma::cdrom::MADR::ADDRESS &&
    //0x1F8010B4 == dma::cdrom::BCR::ADDRESS &&
    //0x1F8010B8 == dma::cdrom::CHCR::ADDRESS &&
    //0x1F8010C0 == dma::spu::MADR::ADDRESS &&
    //0x1F8010C4 == dma::spu::BCR::ADDRESS &&
    //0x1F8010C8 == dma::spu::CHCR::ADDRESS &&
    //0x1F8010D0 == dma::pio::MADR::ADDRESS &&
    //0x1F8010D4 == dma::pio::BCR::ADDRESS &&
    //0x1F8010D8 == dma::pio::CHCR::ADDRESS &&
    0x1F8010E0 == dma::otc::MADR::ADDRESS &&
    0x1F8010E4 == dma::otc::BCR::ADDRESS &&
    0x1F8010E8 == dma::otc::CHCR::ADDRESS
}
