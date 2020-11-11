use crate::gpu::DrawEnv;

impl DrawEnv {
    pub fn copy_rect(&mut self, src: (u16, u16), dst: (u16, u16), size: (u16, u16)) {
        fn serialize((x, y): (u16, u16)) -> u32 {
            (x as u32) | (y as u32) << 16
        }
        let cmd = [0xCC00_0000, serialize(src), serialize(dst), serialize(size)];
        self.write_slice(&cmd);
    }
}
