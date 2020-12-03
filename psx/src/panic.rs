use core::panic::PanicInfo;

use crate::mmio::gpu;
use crate::gpu::color::Color;
use crate::gpu::primitive::tile::Tile;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let (mut gp0, mut gp1) = unsafe {
        (gpu::GP0::new(), gpu::GP1::new())
    };
    gp0.start((0, 0)).end((320, 240)).offset((0, 0));
    gp1.start((0, 0));
    let clear_screen = Tile {
        color: Color::INDIGO,
        cmd: 0x60,
        offset: (0, 0).into(),
        size: (320, 240).into(),
    };
    gp0.send(&clear_screen);
    loop {}
}
