use crate::gpu::DrawEnv;

impl DrawEnv {
    fn serialize((x, y): (u16, u16)) -> u32 {
        (x as u32) | (y as u32) << 16
    }

    pub fn copy_rect(&mut self, src: (u16, u16), dst: (u16, u16), size: (u16, u16)) {
        let cmd = 0xC0 << 24;
        let cmd_params = [
            cmd,
            DrawEnv::serialize(src),
            DrawEnv::serialize(dst),
            DrawEnv::serialize(size),
        ];
        self.write_slice(&cmd_params);
    }

    pub fn rect_to_vram(&mut self, dest: (u16, u16), size: (u16, u16), data: &[u32]) {
        let cmd = 0xA0 << 24;
        let cmd_params = [cmd, DrawEnv::serialize(dest), DrawEnv::serialize(size)];
        self.write_slice(&cmd_params);
        self.write_slice(data);
    }
}
