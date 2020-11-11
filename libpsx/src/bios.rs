extern "C" {
    fn asm_malloc(size: usize) -> *mut u8;
    fn asm_free(buf: *mut u8);
    fn asm_calloc(sizex: usize, sizey: usize) -> *const u8;
    fn asm_realloc(old_buf: *const u8, new_size: usize);
    fn asm_init_heap(addr: usize, size: usize);

    fn asm_printf(s: *const u8, v: u32);

    fn asm_gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32);

    fn asm_gpu_display_env_command_word(cmd: u32);
    fn asm_gpu_command_word(cmd: u32);
    fn asm_gpu_command_word_params(src: *const u32, num: usize);
    fn asm_gpu_get_status() -> u32;
}

pub fn malloc(size: usize) -> *mut u8 {
    unsafe { asm_malloc(size) }
}

pub fn free(buf: *mut u8) {
    unsafe { asm_free(buf) }
}

pub fn calloc(sizex: usize, sizey: usize) -> *const u8 {
    unsafe { asm_calloc(sizex, sizey) }
}

pub fn realloc(old_buf: *const u8, new_size: usize) {
    unsafe { asm_realloc(old_buf, new_size) }
}

pub fn init_heap(addr: usize, size: usize) {
    unsafe { asm_init_heap(addr, size) }
}

pub fn printf(c: *const u8, v: u32) {
    unsafe {
        asm_printf(c, v);
    }
}

pub fn gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32) {
    unsafe { asm_gpu_send_dma(xdst, ydst, xsiz, ysize, src) }
}

pub fn gpu_display_env_command_word(cmd: u32) {
    unsafe {
        asm_gpu_display_env_command_word(cmd);
    }
}

pub fn gpu_command_word(cmd: u32) {
    unsafe {
        asm_gpu_command_word(cmd);
    }
}

pub fn gpu_command_word_params(src: &[u32]) {
    unsafe {
        asm_gpu_command_word_params(src.as_ptr(), src.len());
    }
}

pub fn gpu_get_status() -> u32 {
    unsafe { asm_gpu_get_status() }
}
