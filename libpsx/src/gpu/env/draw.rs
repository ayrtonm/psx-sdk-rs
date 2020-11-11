use crate::gpu::DrawEnv;

impl DrawEnv {
    fn generic_cmd<const CMD: u32, const XMASK: u32, const YMASK: u32, const SHIFT: u32>(&mut self, mut x: u32, mut y: u32) {
        //TODO: make this a compile-time config
        x &= (1 << XMASK) - 1;
        y &= (1 << YMASK) - 1;
        self.write((CMD << 24) | x | (y << SHIFT));
    }
    // Calls DrawEnv(E3h)
    pub fn start(&mut self, x: u32, y: u32) {
        self.generic_cmd::<0xE3, 10, 9, 10>(x, y)
    }
    // Calls DrawEnv(E4h)
    pub fn end(&mut self, x: u32, y: u32) {
        self.generic_cmd::<0xE4, 10, 9, 10>(x, y)
    }
    // Calls DrawEnv(E5h)
    pub fn offset(&mut self, x: u32, y: u32) {
        self.generic_cmd::<0xE5, 11, 11, 11>(x, y)
    }
}
