use crate::hal::GPUSTAT;

pub mod kernel;

/// [BIOS Function A(00h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn file_open(filename: *const u8, accessmode: u32) -> u8 {
    unsafe { kernel::file_open(filename, accessmode) }
}

/// [BIOS Function A(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn exit(exitcode: i32) {
    unsafe { kernel::exit(exitcode) }
}

/// [BIOS Function A(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn save_state(buf: *mut u8) {
    unsafe { kernel::save_state(buf) }
}

/// [BIOS Function A(2Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn rand() -> i16 {
    unsafe { kernel::rand() }
}

/// [BIOS Function A(30h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn srand(seed: u32) {
    unsafe { kernel::srand(seed) }
}

/// [BIOS Function A(33h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn malloc(size: usize) -> *mut u8 {
    unsafe { kernel::malloc(size) }
}

/// [BIOS Function A(34h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn free(buf: *mut u8) {
    unsafe { kernel::free(buf) }
}

/// [BIOS Function A(37h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn calloc(sizex: usize, sizey: usize) -> *const u8 {
    unsafe { kernel::calloc(sizex, sizey) }
}

/// [BIOS Function A(38h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn realloc(old_buf: *const u8, new_size: usize) {
    unsafe { kernel::realloc(old_buf, new_size) }
}

/// [BIOS Function A(39h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn init_heap(addr: usize, size: usize) {
    unsafe { kernel::init_heap(addr, size) }
}

/// [BIOS Function A(3Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn system_error_exit(exitcode: i32) {
    unsafe { kernel::system_error_exit(exitcode) }
}

/// [BIOS Function A(3Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
#[macro_export]
macro_rules! printf {
    ($msg:expr $(,$args:expr)*) => {
        unsafe {
            $crate::bios::kernel::printf($msg.as_ptr() $(,$args)*)
        }
    };
}

/// [BIOS Function A(41h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn load_exe_header(filename: *const u8, headerbuf: *mut u8) {
    unsafe { kernel::load_exe_header(filename, headerbuf) }
}

/// [BIOS Function A(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn load_exe_file(filename: *const u8, headerbuf: *mut u8) {
    unsafe { kernel::load_exe_file(filename, headerbuf) }
}

/// [BIOS Function A(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn do_execute(headerbuf: *mut u8, param1: u32, param2: u32) {
    unsafe { kernel::do_execute(headerbuf, param1, param2) }
}

/// [BIOS Function A(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn flush_cache() {
    unsafe { kernel::flush_cache() }
}

/// [BIOS Function A(47h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32) {
    unsafe { kernel::gpu_send_dma(xdst, ydst, xsiz, ysize, src) }
}

/// [BIOS Function A(48h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn gpu_gp1_command_word(cmd: u32) {
    unsafe { kernel::gpu_gp1_command_word(cmd) }
}

/// [BIOS Function A(49h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn gpu_command_word(cmd: u32) {
    unsafe { kernel::gpu_command_word(cmd) }
}

/// [BIOS Function A(4Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn gpu_command_word_params(src: *const u32, num: usize) {
    unsafe { kernel::gpu_command_word_params(src, num) }
}

/// [BIOS Function A(4Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn gpu_get_status() -> GPUSTAT {
    unsafe { GPUSTAT::from_bits(kernel::gpu_get_status()) }
}

/// [BIOS Function A(51h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn load_and_execute(filename: *const u8, stackbase: u32, stackoffset: u32) {
    unsafe { kernel::load_and_execute(filename, stackbase, stackoffset) }
}

/// [BIOS Function A(72h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn cd_remove() {
    unsafe { kernel::cd_remove() }
}

/// [BIOS Function A(A0h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn warm_boot() {
    unsafe { kernel::warm_boot() }
}

/// [BIOS Function B(12h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn init_pad(buf1: &mut [u8], buf2: &mut [u8]) {
    unsafe { kernel::init_pad(buf1.as_mut_ptr(), buf1.len(), buf2.as_mut_ptr(), buf2.len()) }
}

/// [BIOS Function B(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn start_pad() {
    unsafe { kernel::start_pad() }
}

/// [BIOS Function B(14h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn stop_pad() {
    unsafe { kernel::stop_pad() }
}

/// [BIOS Function B(5Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn change_clear_pad(int: u32) {
    unsafe { kernel::change_clear_pad(int) }
}

/// [BIOS Function C(0Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn change_clear_rcnt(t: u32, flag: u32) {
    unsafe { kernel::change_clear_rcnt(t, flag) }
}

/// [BIOS Function C(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn flush_std_in_out_put() {
    unsafe { kernel::flush_std_in_out_put() }
}

/// [BIOS Function SYS(01h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn enter_critical_section() -> u8 {
    unsafe { kernel::enter_critical_section() }
}

/// [BIOS Function SYS(02h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub fn exit_critical_section() {
    unsafe { kernel::exit_critical_section() }
}
