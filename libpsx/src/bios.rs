extern "C" {
    fn asm_printf(s: *const u8, v: u32);
    fn asm_gpu_get_status() -> u32;
    fn asm_gpu_gp1_command_word(cmd: u32);
    fn asm_gpu_command_word(cmd: u32);
    fn asm_gpu_command_word_params(src: *const u32, num: u32);
}

pub fn printf(c: *const u8, v: u32) {
    unsafe {
        asm_printf(c, v);
    }
}

pub fn gpu_gp1_command_word(cmd: u32) {
    unsafe {
        asm_gpu_gp1_command_word(cmd);
    }
}
pub fn gpu_command_word(cmd: u32) {
    unsafe {
        asm_gpu_command_word(cmd);
    }
}
pub fn gpu_command_word_params(src: *const u32, num: u32) {
    unsafe {
        asm_gpu_command_word_params(src, num);
    }
}
pub fn gpu_get_status() -> u32 {
    unsafe { asm_gpu_get_status() }
}
