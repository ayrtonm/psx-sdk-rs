//! BIOS function wrappers
//!
//! This module provides safe wrappers for calling the built-in BIOS functions.
//! Most of these wrappers call their corresponding functions from the
//! [`kernel`] module. Exceptions are noted below.

use crate::hal::GPUSTAT;
use crate::timer;
use core::mem::transmute;
use core::slice::from_raw_parts;

pub mod kernel;
mod tests;
#[macro_use]
#[doc(hidden)]
pub mod tty;
pub mod fs;
pub mod pad;
mod thread;

pub use thread::Thread;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RootCounter {
    Timer(timer::Name),
    Vblank,
}

impl From<RootCounter> for u32 {
    fn from(rcnt: RootCounter) -> u32 {
        match rcnt {
            RootCounter::Timer(name) => name as u32,
            RootCounter::Vblank => 3,
        }
    }
}

/// Terminates the program and returns control to the BIOS.
pub fn exit(exitcode: i32) -> ! {
    unsafe { kernel::exit(exitcode) }
}

/// Stores a subset of the CPU state.
///
/// Stores 12 CPU registers in the first 48 bytes of `buffer`. This can then be
/// used with [`io_abort`], [`restore_state`] or
/// [`set_custom_exit_from_exception`].
pub fn save_state(buffer: &mut [u32]) {
    unsafe { kernel::save_state(buffer.as_mut_ptr() as *mut u8) }
}

/// Returns a random 15 bit number with the seeded generator.
///
/// Advances the generator state to `x = x * 0x41C6_4E6D + 0x3039` and returns
/// the lower 15 bits of `x / 0x10000`.
pub fn rand() -> u16 {
    unsafe { kernel::rand() }
}

/// Sets the seed of the random number generator.
pub fn srand(seed: u32) {
    unsafe { kernel::srand(seed) }
}

// TODO: Reimplement malloc since the BIOS's version is hopelessly broken
///// Allocates the requested bytes on the heap by calling [`kernel::malloc`].
/////
///// Returns a slice of the heap allocation aligned to 4-byte memory boundaries
///// and with the requested `bytes` rounded up to a multiple of 4. The address
///// may be in KUSEG, KSEG0, or KSEG1, depending on the address passed to
///// [`init_heap`].
//pub fn malloc<'a>(bytes: usize) -> &'a mut [u8] {
//    unsafe {
//        let ptr = kernel::malloc(bytes);
//        core::slice::from_raw_parts_mut(ptr, bytes)
//    }
//}
//
///// Calls [A(34h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn free(buf: &mut [u8]) {
//    unsafe { kernel::free(buf.as_mut_ptr()) }
//}
//
///// Calls [A(37h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn calloc(sizex: usize, sizey: usize) -> *const u8 {
//    unsafe { kernel::calloc(sizex, sizey) }
//}
//
///// Calls [A(38h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn realloc(old_buf: *const u8, new_size: usize) {
//    unsafe { kernel::realloc(old_buf, new_size) }
//}
//
///// Calls [A(39h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn init_heap(heap: &mut [u32]) {
//    unsafe { kernel::init_heap(heap.as_ptr() as usize, heap.len() *
// core::mem::size_of::<u32>()) }
//}

/// Terminates and locks up the BIOS.
pub fn system_error_exit(exitcode: i32) -> ! {
    unsafe { kernel::system_error_exit(exitcode) }
}

//// TODO: Test and document
///// Calls [A(41h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn load_exe_header(filename: *const u8, headerbuf: &mut [u8]) {
//    unsafe { kernel::load_exe_header(filename, headerbuf.as_mut_ptr()) }
//}
//
//// TODO: Test and document
///// Calls [A(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn load_exe_file(filename: *const u8, headerbuf: &mut [u8]) {
//    unsafe { kernel::load_exe_file(filename, headerbuf.as_mut_ptr()) }
//}
//
//// TODO: Test and document
///// Calls [A(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn do_execute(headerbuf: *mut u8, param1: u32, param2: u32) {
//    unsafe { kernel::do_execute(headerbuf, param1, param2) }
//}
//
//// TODO: Test and document
///// Calls [A(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn flush_cache() {
//    unsafe { kernel::flush_cache() }
//}
//
//// TODO: Test and document
///// Calls [A(47h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn gpu_send_dma(
//    _gpu_dma: &mut dma::GPU, xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src:
// u32,
//) {
//    unsafe { kernel::gpu_send_dma(xdst, ydst, xsiz, ysize, src) }
//}

/// Sends the given `cmd` to [`GP1`](crate::hal::GP1).
pub fn gp1_command(cmd: u32) {
    unsafe { kernel::gp1_command(cmd) }
}

/// Calls [`gpu_sync`] then sends the given `cmd` to [`GP0`](crate::hal::GP0).
pub fn gp0_command(cmd: u32) {
    unsafe { kernel::gp0_command(cmd) }
}

/// Calls [`gpu_sync`] then sends the given `data` to [`GP0`](crate::hal::GP0).
pub fn gp0_command_params(data: &[u32]) {
    unsafe { kernel::gp0_command_params(data.as_ptr(), data.len()) }
}

/// Gets [`GPUSTAT`](crate::hal::GPUSTAT).
pub fn gpu_get_status() -> GPUSTAT {
    unsafe { GPUSTAT::from_bits(kernel::gpu_get_status()) }
}

/// Waits until the GPU is ready to receive a DMA block.
///
/// If DMA is enabled on the GPU side it first waits for any ongoing transfer
/// then disables DMA. Returns `true` in case of timeout.
pub fn gpu_sync() -> bool {
    unsafe { kernel::gpu_sync() != 0 }
}

//// TODO: Test and document
///// Calls [A(51h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn load_and_execute(filename: *const u8, stackbase: u32, stackoffset:
// u32) {    unsafe { kernel::load_and_execute(filename, stackbase, stackoffset)
// }
//}
//
//// TODO: Test and document
///// Calls [A(72h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn cd_remove() {
//    unsafe { kernel::cd_remove() }
//}

// TODO: Test and document
///// Calls [A(7Ch)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn cd_async_get_status() -> u32 {
//    let mut res = 0;
//    unsafe { kernel::cd_async_get_status(&mut res) };
//    res
//}

//// TODO: Test and document
///// Calls [A(9Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn set_memsize(megabytes: u8) {
//    unsafe { kernel::set_memsize(megabytes) }
//}

/// Resets the kernel and reboots from CDROM.
///
/// This doesn't display the intro screens, verify the PlayStation logo in the
/// ISO system area, enter the bootmenu or reload the
/// [`SYSTEM.CNF`](http://problemkaputt.de/psx-spx.htm#cdromfileformats).
pub fn warm_boot() -> ! {
    unsafe { kernel::warm_boot() }
}

//// TODO: Test and document
///// Calls [A(A4h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn cd_get_lbn(filename: *const u8) -> Option<u32> {
//    let res = unsafe { kernel::cd_get_lbn(filename) };
//    if res == -1 {
//        None
//    } else {
//        Some(res as u32)
//    }
//}

// TODO: Test and document
///// Calls [A(A6h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn cd_get_status() -> u32 {
//    let mut res = 0;
//    unsafe { kernel::cd_get_status(&mut res) };
//    res
//}

/// Gets the BIOS's date in BCD by calling [`kernel::get_system_info`] with
/// `0x00`.
pub fn system_date() -> u32 {
    unsafe { kernel::get_system_info(0) }
}

/// Gets the BIOS's version string by calling [`kernel::get_system_info`] with
/// `0x02`.
pub fn system_version() -> &'static str {
    unsafe {
        let res = kernel::get_system_info(2);
        let mut len = 0;
        let ptr = res as *const u32 as *const u8;
        // Won't hang since the BIOS str is zero-terminated
        while *ptr.add(len) != 0 {
            len += 1;
        }
        transmute(from_raw_parts(ptr, len))
    }
}

/// Gets the RAM size in kB by calling [`kernel::get_system_info`] with `0x05`.
pub fn system_ram() -> u32 {
    unsafe { kernel::get_system_info(5) }
}

// TODO: Test and document these
///// Calls [B(03h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn get_timer(rcnt: timer::Name) {
//    unsafe { kernel::get_timer(rcnt as u32) }
//}
//
///// Calls [B(04h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn enable_timer_irq(rcnt: RootCounter) {
//    unsafe { kernel::enable_timer_irq(rcnt.into()) }
//}
///// Calls [B(05h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn disable_timer_irq(rcnt: RootCounter) {
//    unsafe { kernel::disable_timer_irq(rcnt.into()) }
//}
///// Calls [B(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn restart_timer(rcnt: timer::Name) {
//    unsafe { kernel::restart_timer(rcnt as u32) }
//}

///// Calls [B(5Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn change_clear_pad(int: u32) {
//    unsafe { kernel::change_clear_pad(int) }
//}
//
///// Calls [C(0Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn change_clear_rcnt(rcnt: RootCounter, flag: bool) -> bool {
//    unsafe { kernel::change_clear_rcnt(rcnt.into(), flag) }
//}
//
///// Calls [C(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
//pub fn flush_std_in_out_put() {
//    unsafe { kernel::flush_std_in_out_put() }
//}

// TODO: Test this with cop0 registers
/// Disables interrupts in coprocessor 0. Returns `false` if called from a
/// critical section.
pub fn enter_critical_section() -> bool {
    unsafe { kernel::enter_critical_section() }
}

// TODO: Test this with cop0 registers
/// Enables interrupts in coprocessor 0.
pub fn exit_critical_section() {
    unsafe { kernel::exit_critical_section() }
}

/// Executes the given closure in a critical section.
pub fn critical_section<F: FnOnce() -> R, R>(f: F) -> R {
    enter_critical_section();
    let res = f();
    exit_critical_section();
    res
}
