use crate::gpu::vertex::Component;
use crate::gpu::DrawEnv;
use crate::macros::RegisterWrite;

impl DrawEnv {
    // Calls DrawEnv(E3h)
    pub fn start(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE3, 10, 9, 10>(x, y)
    }

    // Calls DrawEnv(E4h)
    pub fn end(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE4, 10, 9, 10>(x, y)
    }

    // Calls DrawEnv(E5h)
    pub fn offset(&mut self, x: Component, y: Component) {
        self.generic_cmd::<0xE5, 11, 11, 11>(x, y)
    }

    fn generic_cmd<
        const CMD: u8,
        const XMASK: Component,
        const YMASK: Component,
        const SHIFT: Component,
    >(
        &mut self, mut x: Component, mut y: Component,
    ) {
        if cfg!(debug_assertions) {
            x &= (1 << XMASK) - 1;
            y &= (1 << YMASK) - 1;
        }
        let cmd = (CMD as u32) << 24;
        let x = x as u32;
        let y = (y as u32) << (SHIFT as u32);
        self.write(cmd | x | y);
    }
}
