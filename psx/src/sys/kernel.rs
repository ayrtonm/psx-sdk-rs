//! BIOS kernel functions
//!
//! These are wrappers for calling BIOS functions directly.
// This file was automatically generated by gen_bios_mod.rs

core::arch::global_asm!(include_str!("trampoline.s"));

extern "C" {
    /// Calls BIOS function [A(00h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_open(filename: *const i8, accessmode: u32) -> i8;
    /// Calls BIOS function [A(01h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_seek(fd: i8, offset: u32, seektype: u8) -> i32;
    /// Calls BIOS function [A(02h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_read(fd: i8, dst: *mut u32, length: usize) -> i32;
    /// Calls BIOS function [A(03h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_write(fd: i8, src: *const u32, length: usize) -> i32;
    /// Calls BIOS function [A(04h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_close(fd: i8) -> i8;
    /// Calls BIOS function [A(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_exit(exitcode: i32) -> !;
    /// Calls BIOS function [A(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_save_state(buf: *mut u8);
    /// Calls BIOS function [A(14h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_restore_state(buf: *const u8, ret_val: u32);
    /// Calls BIOS function [A(2Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_rand() -> u16;
    /// Calls BIOS function [A(30h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_srand(seed: u32);
    /// Calls BIOS function [A(33h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_malloc(size: usize) -> *mut u8;
    /// Calls BIOS function [A(34h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_free(buf: *mut u8);
    /// Calls BIOS function [A(37h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_calloc(sizex: usize, sizey: usize) -> *const u8;
    /// Calls BIOS function [A(38h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_realloc(old_buf: *const u8, new_size: usize);
    /// Calls BIOS function [A(39h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_init_heap(addr: usize, size: usize);
    /// Calls BIOS function [A(3Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_system_error_exit(exitcode: i32) -> !;
    /// Calls BIOS function [B(3Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_std_out_putchar(char: u8);
    /// Calls BIOS function [A(3Eh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_std_out_puts(msg: *const i8);
    /// Calls BIOS function [A(3Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_printf(msg: *const i8, ...);
    /// Calls BIOS function [A(41h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_load_exe_header(filename: *const i8, headerbuf: *mut u8);
    /// Calls BIOS function [A(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_load_exe_file(filename: *const i8, headerbuf: *mut u8);
    /// Calls BIOS function [A(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_do_execute(headerbuf: *mut u8, param1: u32, param2: u32);
    /// Calls BIOS function [A(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_flush_cache();
    /// Calls BIOS function [A(47h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32);
    /// Calls BIOS function [A(48h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gp1_command(cmd: u32);
    /// Calls BIOS function [A(49h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gp0_command(cmd: u32);
    /// Calls BIOS function [A(4Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gp0_command_params(src: *const u32, num: usize);
    /// Calls BIOS function [A(4Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gpu_get_status() -> u32;
    /// Calls BIOS function [A(4Eh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_gpu_sync() -> i32;
    /// Calls BIOS function [A(51h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_load_and_execute(filename: *const i8, stackbase: u32, stackoffset: u32);
    /// Calls BIOS function [A(54h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_cd_init();
    /// Calls BIOS function [A(56h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_cd_remove();
    /// Calls BIOS function [A(7Ch)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_cd_async_get_status(dst: *mut u32);
    /// Calls BIOS function [A(96h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_add_cdrom_device();
    /// Calls BIOS function [A(9Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_set_memsize(megabytes: u8);
    /// Calls BIOS function [A(A0h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_warm_boot() -> !;
    /// Calls BIOS function [A(A4h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_cd_get_lbn(filename: *const i8) -> i32;
    /// Calls BIOS function [A(A6h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_cd_get_status(dst: *mut u32);
    /// Calls BIOS function [A(B4h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_get_system_info(index: u8) -> u32;
    /// Calls BIOS function [B(03h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_get_timer(t: u32);
    /// Calls BIOS function [B(04h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_enable_timer_irq(t: u32);
    /// Calls BIOS function [B(05h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_disable_timer_irq(t: u32);
    /// Calls BIOS function [B(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_restart_timer(t: u32);
    /// Calls BIOS function [B(07h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_deliver_event(class: u32, spec: u16);
    /// Calls BIOS function [B(08h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_open_event(class: u32, spec: u16, mode: u16, func: *const u32) -> u32;
    /// Calls BIOS function [B(09h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_close_event(event: u32);
    /// Calls BIOS function [B(0Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_wait_event(event: u32) -> bool;
    /// Calls BIOS function [B(0Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_test_event(event: u32) -> bool;
    /// Calls BIOS function [B(0Ch)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_enable_event(event: u32);
    /// Calls BIOS function [B(0Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_disable_event(event: u32);
    /// Calls BIOS function [B(0Eh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_open_thread(pc: usize, sp: usize, gp: usize) -> u32;
    /// Calls BIOS function [B(0Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_close_thread(handle: u32);
    /// Calls BIOS function [B(10h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_change_thread(handle: u32);
    /// Calls BIOS function [B(12h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_init_pad(buf1: *mut u8, siz1: usize, buf2: *mut u8, siz2: usize);
    /// Calls BIOS function [B(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_start_pad();
    /// Calls BIOS function [B(14h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_stop_pad();
    /// Calls BIOS function [B(18h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_set_default_exit_from_exception();
    /// Calls BIOS function [B(20h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_undeliver_event(class: u32, spec: u16);
    /// Calls BIOS function [B(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_first_file(filename: *const i8) -> *const [u8; 40];
    /// Calls BIOS function [B(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_next_file() -> *const [u8; 40];
    /// Calls BIOS function [B(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_rename(old_filename: *const i8, new_filename: *const i8) -> bool;
    /// Calls BIOS function [B(45h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_delete(filename: *const i8) -> bool;
    /// Calls BIOS function [B(46h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_file_undelete(filename: *const i8) -> bool;
    /// Calls BIOS function [B(49h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_print_installed_devices();
    /// Calls BIOS function [B(4Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_init_card(pad_enable: bool);
    /// Calls BIOS function [B(4Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_start_card();
    /// Calls BIOS function [B(4Ch)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_stop_card();
    /// Calls BIOS function [B(54h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_get_last_error() -> u32;
    /// Calls BIOS function [B(55h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_get_last_file_error(fd: i8) -> u32;
    /// Calls BIOS function [B(5Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_change_clear_pad(int: u32);
    /// Calls BIOS function [C(0Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_change_clear_rcnt(t: u32, flag: bool) -> bool;
    /// Calls BIOS function [C(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_flush_std_in_out_put();
    /// Calls BIOS function [SYS(01h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_enter_critical_section() -> bool;
    /// Calls BIOS function [SYS(02h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_exit_critical_section();
    /// Calls BIOS function [SYS(03h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn psx_change_thread_sub_fn(_: usize, addr: usize);
}
/// The BIOS function number for file_open
pub const FILE_OPEN_NUM: u8 = 0x00;
/// The BIOS function type for file_open
pub const FILE_OPEN_TY: u8 = 0xA0;
/// The BIOS function number for file_seek
pub const FILE_SEEK_NUM: u8 = 0x01;
/// The BIOS function type for file_seek
pub const FILE_SEEK_TY: u8 = 0xA0;
/// The BIOS function number for file_read
pub const FILE_READ_NUM: u8 = 0x02;
/// The BIOS function type for file_read
pub const FILE_READ_TY: u8 = 0xA0;
/// The BIOS function number for file_write
pub const FILE_WRITE_NUM: u8 = 0x03;
/// The BIOS function type for file_write
pub const FILE_WRITE_TY: u8 = 0xA0;
/// The BIOS function number for file_close
pub const FILE_CLOSE_NUM: u8 = 0x04;
/// The BIOS function type for file_close
pub const FILE_CLOSE_TY: u8 = 0xA0;
/// The BIOS function number for exit
pub const EXIT_NUM: u8 = 0x06;
/// The BIOS function type for exit
pub const EXIT_TY: u8 = 0xA0;
/// The BIOS function number for save_state
pub const SAVE_STATE_NUM: u8 = 0x13;
/// The BIOS function type for save_state
pub const SAVE_STATE_TY: u8 = 0xA0;
/// The BIOS function number for restore_state
pub const RESTORE_STATE_NUM: u8 = 0x14;
/// The BIOS function type for restore_state
pub const RESTORE_STATE_TY: u8 = 0xA0;
/// The BIOS function number for rand
pub const RAND_NUM: u8 = 0x2F;
/// The BIOS function type for rand
pub const RAND_TY: u8 = 0xA0;
/// The BIOS function number for srand
pub const SRAND_NUM: u8 = 0x30;
/// The BIOS function type for srand
pub const SRAND_TY: u8 = 0xA0;
/// The BIOS function number for malloc
pub const MALLOC_NUM: u8 = 0x33;
/// The BIOS function type for malloc
pub const MALLOC_TY: u8 = 0xA0;
/// The BIOS function number for free
pub const FREE_NUM: u8 = 0x34;
/// The BIOS function type for free
pub const FREE_TY: u8 = 0xA0;
/// The BIOS function number for calloc
pub const CALLOC_NUM: u8 = 0x37;
/// The BIOS function type for calloc
pub const CALLOC_TY: u8 = 0xA0;
/// The BIOS function number for realloc
pub const REALLOC_NUM: u8 = 0x38;
/// The BIOS function type for realloc
pub const REALLOC_TY: u8 = 0xA0;
/// The BIOS function number for init_heap
pub const INIT_HEAP_NUM: u8 = 0x39;
/// The BIOS function type for init_heap
pub const INIT_HEAP_TY: u8 = 0xA0;
/// The BIOS function number for system_error_exit
pub const SYSTEM_ERROR_EXIT_NUM: u8 = 0x3A;
/// The BIOS function type for system_error_exit
pub const SYSTEM_ERROR_EXIT_TY: u8 = 0xA0;
/// The BIOS function number for std_out_putchar
pub const STD_OUT_PUTCHAR_NUM: u8 = 0x3D;
/// The BIOS function type for std_out_putchar
pub const STD_OUT_PUTCHAR_TY: u8 = 0xB0;
/// The BIOS function number for std_out_puts
pub const STD_OUT_PUTS_NUM: u8 = 0x3E;
/// The BIOS function type for std_out_puts
pub const STD_OUT_PUTS_TY: u8 = 0xA0;
/// The BIOS function number for printf
pub const PRINTF_NUM: u8 = 0x3F;
/// The BIOS function type for printf
pub const PRINTF_TY: u8 = 0xA0;
/// The BIOS function number for load_exe_header
pub const LOAD_EXE_HEADER_NUM: u8 = 0x41;
/// The BIOS function type for load_exe_header
pub const LOAD_EXE_HEADER_TY: u8 = 0xA0;
/// The BIOS function number for load_exe_file
pub const LOAD_EXE_FILE_NUM: u8 = 0x42;
/// The BIOS function type for load_exe_file
pub const LOAD_EXE_FILE_TY: u8 = 0xA0;
/// The BIOS function number for do_execute
pub const DO_EXECUTE_NUM: u8 = 0x43;
/// The BIOS function type for do_execute
pub const DO_EXECUTE_TY: u8 = 0xA0;
/// The BIOS function number for flush_cache
pub const FLUSH_CACHE_NUM: u8 = 0x44;
/// The BIOS function type for flush_cache
pub const FLUSH_CACHE_TY: u8 = 0xA0;
/// The BIOS function number for gpu_send_dma
pub const GPU_SEND_DMA_NUM: u8 = 0x47;
/// The BIOS function type for gpu_send_dma
pub const GPU_SEND_DMA_TY: u8 = 0xA0;
/// The BIOS function number for gp1_command
pub const GP1_COMMAND_NUM: u8 = 0x48;
/// The BIOS function type for gp1_command
pub const GP1_COMMAND_TY: u8 = 0xA0;
/// The BIOS function number for gp0_command
pub const GP0_COMMAND_NUM: u8 = 0x49;
/// The BIOS function type for gp0_command
pub const GP0_COMMAND_TY: u8 = 0xA0;
/// The BIOS function number for gp0_command_params
pub const GP0_COMMAND_PARAMS_NUM: u8 = 0x4A;
/// The BIOS function type for gp0_command_params
pub const GP0_COMMAND_PARAMS_TY: u8 = 0xA0;
/// The BIOS function number for gpu_get_status
pub const GPU_GET_STATUS_NUM: u8 = 0x4D;
/// The BIOS function type for gpu_get_status
pub const GPU_GET_STATUS_TY: u8 = 0xA0;
/// The BIOS function number for gpu_sync
pub const GPU_SYNC_NUM: u8 = 0x4E;
/// The BIOS function type for gpu_sync
pub const GPU_SYNC_TY: u8 = 0xA0;
/// The BIOS function number for load_and_execute
pub const LOAD_AND_EXECUTE_NUM: u8 = 0x51;
/// The BIOS function type for load_and_execute
pub const LOAD_AND_EXECUTE_TY: u8 = 0xA0;
/// The BIOS function number for cd_init
pub const CD_INIT_NUM: u8 = 0x54;
/// The BIOS function type for cd_init
pub const CD_INIT_TY: u8 = 0xA0;
/// The BIOS function number for cd_remove
pub const CD_REMOVE_NUM: u8 = 0x56;
/// The BIOS function type for cd_remove
pub const CD_REMOVE_TY: u8 = 0xA0;
/// The BIOS function number for cd_async_get_status
pub const CD_ASYNC_GET_STATUS_NUM: u8 = 0x7C;
/// The BIOS function type for cd_async_get_status
pub const CD_ASYNC_GET_STATUS_TY: u8 = 0xA0;
/// The BIOS function number for add_cdrom_device
pub const ADD_CDROM_DEVICE_NUM: u8 = 0x96;
/// The BIOS function type for add_cdrom_device
pub const ADD_CDROM_DEVICE_TY: u8 = 0xA0;
/// The BIOS function number for set_memsize
pub const SET_MEMSIZE_NUM: u8 = 0x9F;
/// The BIOS function type for set_memsize
pub const SET_MEMSIZE_TY: u8 = 0xA0;
/// The BIOS function number for warm_boot
pub const WARM_BOOT_NUM: u8 = 0xA0;
/// The BIOS function type for warm_boot
pub const WARM_BOOT_TY: u8 = 0xA0;
/// The BIOS function number for cd_get_lbn
pub const CD_GET_LBN_NUM: u8 = 0xA4;
/// The BIOS function type for cd_get_lbn
pub const CD_GET_LBN_TY: u8 = 0xA0;
/// The BIOS function number for cd_get_status
pub const CD_GET_STATUS_NUM: u8 = 0xA6;
/// The BIOS function type for cd_get_status
pub const CD_GET_STATUS_TY: u8 = 0xA0;
/// The BIOS function number for get_system_info
pub const GET_SYSTEM_INFO_NUM: u8 = 0xB4;
/// The BIOS function type for get_system_info
pub const GET_SYSTEM_INFO_TY: u8 = 0xA0;
/// The BIOS function number for get_timer
pub const GET_TIMER_NUM: u8 = 0x03;
/// The BIOS function type for get_timer
pub const GET_TIMER_TY: u8 = 0xB0;
/// The BIOS function number for enable_timer_irq
pub const ENABLE_TIMER_IRQ_NUM: u8 = 0x04;
/// The BIOS function type for enable_timer_irq
pub const ENABLE_TIMER_IRQ_TY: u8 = 0xB0;
/// The BIOS function number for disable_timer_irq
pub const DISABLE_TIMER_IRQ_NUM: u8 = 0x05;
/// The BIOS function type for disable_timer_irq
pub const DISABLE_TIMER_IRQ_TY: u8 = 0xB0;
/// The BIOS function number for restart_timer
pub const RESTART_TIMER_NUM: u8 = 0x06;
/// The BIOS function type for restart_timer
pub const RESTART_TIMER_TY: u8 = 0xB0;
/// The BIOS function number for deliver_event
pub const DELIVER_EVENT_NUM: u8 = 0x07;
/// The BIOS function type for deliver_event
pub const DELIVER_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for open_event
pub const OPEN_EVENT_NUM: u8 = 0x08;
/// The BIOS function type for open_event
pub const OPEN_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for close_event
pub const CLOSE_EVENT_NUM: u8 = 0x09;
/// The BIOS function type for close_event
pub const CLOSE_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for wait_event
pub const WAIT_EVENT_NUM: u8 = 0x0A;
/// The BIOS function type for wait_event
pub const WAIT_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for test_event
pub const TEST_EVENT_NUM: u8 = 0x0B;
/// The BIOS function type for test_event
pub const TEST_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for enable_event
pub const ENABLE_EVENT_NUM: u8 = 0x0C;
/// The BIOS function type for enable_event
pub const ENABLE_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for disable_event
pub const DISABLE_EVENT_NUM: u8 = 0x0D;
/// The BIOS function type for disable_event
pub const DISABLE_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for open_thread
pub const OPEN_THREAD_NUM: u8 = 0x0E;
/// The BIOS function type for open_thread
pub const OPEN_THREAD_TY: u8 = 0xB0;
/// The BIOS function number for close_thread
pub const CLOSE_THREAD_NUM: u8 = 0x0F;
/// The BIOS function type for close_thread
pub const CLOSE_THREAD_TY: u8 = 0xB0;
/// The BIOS function number for change_thread
pub const CHANGE_THREAD_NUM: u8 = 0x10;
/// The BIOS function type for change_thread
pub const CHANGE_THREAD_TY: u8 = 0xB0;
/// The BIOS function number for init_pad
pub const INIT_PAD_NUM: u8 = 0x12;
/// The BIOS function type for init_pad
pub const INIT_PAD_TY: u8 = 0xB0;
/// The BIOS function number for start_pad
pub const START_PAD_NUM: u8 = 0x13;
/// The BIOS function type for start_pad
pub const START_PAD_TY: u8 = 0xB0;
/// The BIOS function number for stop_pad
pub const STOP_PAD_NUM: u8 = 0x14;
/// The BIOS function type for stop_pad
pub const STOP_PAD_TY: u8 = 0xB0;
/// The BIOS function number for set_default_exit_from_exception
pub const SET_DEFAULT_EXIT_FROM_EXCEPTION_NUM: u8 = 0x18;
/// The BIOS function type for set_default_exit_from_exception
pub const SET_DEFAULT_EXIT_FROM_EXCEPTION_TY: u8 = 0xB0;
/// The BIOS function number for undeliver_event
pub const UNDELIVER_EVENT_NUM: u8 = 0x20;
/// The BIOS function type for undeliver_event
pub const UNDELIVER_EVENT_TY: u8 = 0xB0;
/// The BIOS function number for first_file
pub const FIRST_FILE_NUM: u8 = 0x42;
/// The BIOS function type for first_file
pub const FIRST_FILE_TY: u8 = 0xB0;
/// The BIOS function number for next_file
pub const NEXT_FILE_NUM: u8 = 0x43;
/// The BIOS function type for next_file
pub const NEXT_FILE_TY: u8 = 0xB0;
/// The BIOS function number for file_rename
pub const FILE_RENAME_NUM: u8 = 0x44;
/// The BIOS function type for file_rename
pub const FILE_RENAME_TY: u8 = 0xB0;
/// The BIOS function number for file_delete
pub const FILE_DELETE_NUM: u8 = 0x45;
/// The BIOS function type for file_delete
pub const FILE_DELETE_TY: u8 = 0xB0;
/// The BIOS function number for file_undelete
pub const FILE_UNDELETE_NUM: u8 = 0x46;
/// The BIOS function type for file_undelete
pub const FILE_UNDELETE_TY: u8 = 0xB0;
/// The BIOS function number for print_installed_devices
pub const PRINT_INSTALLED_DEVICES_NUM: u8 = 0x49;
/// The BIOS function type for print_installed_devices
pub const PRINT_INSTALLED_DEVICES_TY: u8 = 0xB0;
/// The BIOS function number for init_card
pub const INIT_CARD_NUM: u8 = 0x4A;
/// The BIOS function type for init_card
pub const INIT_CARD_TY: u8 = 0xB0;
/// The BIOS function number for start_card
pub const START_CARD_NUM: u8 = 0x4B;
/// The BIOS function type for start_card
pub const START_CARD_TY: u8 = 0xB0;
/// The BIOS function number for stop_card
pub const STOP_CARD_NUM: u8 = 0x4C;
/// The BIOS function type for stop_card
pub const STOP_CARD_TY: u8 = 0xB0;
/// The BIOS function number for get_last_error
pub const GET_LAST_ERROR_NUM: u8 = 0x54;
/// The BIOS function type for get_last_error
pub const GET_LAST_ERROR_TY: u8 = 0xB0;
/// The BIOS function number for get_last_file_error
pub const GET_LAST_FILE_ERROR_NUM: u8 = 0x55;
/// The BIOS function type for get_last_file_error
pub const GET_LAST_FILE_ERROR_TY: u8 = 0xB0;
/// The BIOS function number for change_clear_pad
pub const CHANGE_CLEAR_PAD_NUM: u8 = 0x5B;
/// The BIOS function type for change_clear_pad
pub const CHANGE_CLEAR_PAD_TY: u8 = 0xB0;
/// The BIOS function number for change_clear_rcnt
pub const CHANGE_CLEAR_RCNT_NUM: u8 = 0x0A;
/// The BIOS function type for change_clear_rcnt
pub const CHANGE_CLEAR_RCNT_TY: u8 = 0xC0;
/// The BIOS function number for flush_std_in_out_put
pub const FLUSH_STD_IN_OUT_PUT_NUM: u8 = 0x13;
/// The BIOS function type for flush_std_in_out_put
pub const FLUSH_STD_IN_OUT_PUT_TY: u8 = 0xC0;
/// The BIOS function number for enter_critical_section
pub const ENTER_CRITICAL_SECTION_NUM: u8 = 0x01;
/// The BIOS function number for exit_critical_section
pub const EXIT_CRITICAL_SECTION_NUM: u8 = 0x02;
/// The BIOS function number for change_thread_sub_fn
pub const CHANGE_THREAD_SUB_FN_NUM: u8 = 0x03;
