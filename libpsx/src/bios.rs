use crate::bios_asm::*;

pub fn malloc(size: usize) -> *mut u8 {
    asm_malloc(size)
}

pub fn free(buf: *mut u8) {
    asm_free(buf)
}

pub fn calloc(sizex: usize, sizey: usize) -> *const u8 {
    asm_calloc(sizex, sizey)
}

pub fn realloc(old_buf: *const u8, new_size: usize) {
    asm_realloc(old_buf, new_size)
}

pub fn init_heap(addr: usize, size: usize) {
    asm_init_heap(addr, size)
}

pub fn printf(c: *const u8, v: u32) {
    asm_printf(c, v);
}

pub fn gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32) {
    asm_gpu_send_dma(xdst, ydst, xsiz, ysize, src)
}

pub fn gpu_gp1_command_word(cmd: u32) {
    asm_gpu_gp1_command_word(cmd);
}

pub fn gpu_command_word(cmd: u32) {
    asm_gpu_command_word(cmd);
}

pub fn gpu_command_word_params(src: &[u32]) {
    asm_gpu_command_word_params(src.as_ptr(), src.len());
}

pub fn gpu_get_status() -> u32 {
    asm_gpu_get_status()
}
