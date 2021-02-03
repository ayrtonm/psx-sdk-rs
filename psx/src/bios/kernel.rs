global_asm!(
    r#"
.set noreorder

.globl printf
printf:
    j 0xA0
    li $9, 0x3F

.globl gpu_get_status
gpu_get_status:
    j 0xA0
    li $9, 0x4D

.globl malloc
malloc:
    j 0xA0
    li $9, 0x33

.globl free
free:
    j 0xA0
    li $9, 0x34
"#
);

extern "C" {
    pub fn printf(msg: *const u8, ...);
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(buf: *mut u8);
    pub fn gpu_get_status() -> u32;
    pub fn init_pad(buf1: *mut u8, siz1: usize, buf2: *mut u8, siz2: usize);
}
