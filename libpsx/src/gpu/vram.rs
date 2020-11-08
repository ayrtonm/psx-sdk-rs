use crate::bios;

pub fn copy_rect(src: (u16, u16), dest: (u16, u16), size: (u16, u16)) {
    fn zero_extend(xy: (u16, u16)) -> (u32, u32) {
        (xy.0 as u32, xy.1 as u32)
    }
    let src = zero_extend(src);
    let dest = zero_extend(dest);
    let size = zero_extend(size);
    let ar = [0xcc00_0000,
              src.0 | src.1 << 16,
              dest.0 | dest.1 << 16,
              size.0 | size.1 << 16];
    bios::gpu_command_word_params(&ar);
}
