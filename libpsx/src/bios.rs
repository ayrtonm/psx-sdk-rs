extern {
    fn asm_putchar(b: u8) -> u32;
    fn asm_puts(s: *const u8) -> u32;
    fn asm_toupper(b: u8) -> u8;
    fn asm_print_devices();
    fn asm_printf(s: *const u8, v: u32);
    fn asm_gpu_get_status() -> u32;
    fn asm_gpu_gp1_command_word(cmd: u32);
    fn asm_gpu_command_word(cmd: u32);
    fn asm_gpu_command_word_and_params(src: *const u32, num: u32);
}

fn print_devices() {
    unsafe {
        asm_print_devices();
    }
}

fn putchar(c: u8) {
    unsafe {
        asm_putchar(c);
    }
}

fn printf(c: *const u8, v: u32) {
    unsafe { asm_printf(c, v) };
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
pub fn gpu_command_word_and_params(src: *const u32, num: u32) {
    unsafe {
        asm_gpu_command_word_and_params(src, num);
    }
}
