#![no_std]

fn print_devices() {
    unsafe {
        bios_print_devices();
    }
}

fn putchar(c: u8) {
    unsafe {
        bios_putchar(c);
    }
}

fn printf(c: *const u8, v: u32) {
    unsafe { bios_printf(c, v) };
}

extern {
    fn bios_putchar(b: u8) -> u32;
    fn bios_puts(s: *const u8) -> u32;
    fn bios_toupper(b: u8) -> u8;
    fn bios_print_devices();
    fn bios_printf(s: *const u8, v: u32);
    fn bios_gpu_get_status() -> u32;
    fn bios_gpu_gp1_command_word(cmd: u32);
    fn bios_gpu_command_word(cmd: u32);
    fn bios_gpu_command_word_and_params(src: *const u32, num: u32);
    fn asm_load_delay_test();
}

pub fn gpu_gp1_command_word(cmd: u32) {
    unsafe {
        bios_gpu_gp1_command_word(cmd);
    }
}
pub fn gpu_command_word(cmd: u32) {
    unsafe {
        bios_gpu_command_word(cmd);
    }
}
pub fn gpu_command_word_and_params(src: *const u32, num: u32) {
    unsafe {
        bios_gpu_command_word_and_params(src, num);
    }
}
pub fn load_delay_test() {
    unsafe {
        asm_load_delay_test();
    }
}
