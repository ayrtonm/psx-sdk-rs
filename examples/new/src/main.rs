#![no_std]
#![no_main]
#![feature(min_const_generics)]

use core::cell::RefCell;
use psx::dma::{BaseAddress, BlockControl, BlockSize, ChannelControl, Direction, SyncMode};
use psx::framebuffer::UncheckedFramebuffer;
use psx::gpu::color::Color;
use psx::gpu::primitives;
use psx::gpu::primitives::polyf::{PolyF3, PolyF4};

psx::exe!();

fn main(mut mmio: MMIO) {
    let gp0 = RefCell::new(mmio.gp0);
    let gp1 = RefCell::new(mmio.gp1);
    mmio.dma_control.gpu(true);
    mmio.dma_control.otc(true);
    let mut fb = UncheckedFramebuffer::new((0, 0), (0, 240), (320, 240), &gp0, &gp1);

    let mut buffer = primitives::Buffer::<11>::new();

    let mut prim0 = PolyF3::new(&mut buffer, [(0, 0), (100, 0), (0, 100)], Color::BLUE);
    let mut prim1 = PolyF4::from(&mut buffer);
    prim1
        .vertices([(100, 100), (50, 100), (100, 50), (50, 50)])
        .color(Color::YELLOW);

    let mut otc = OTC { entries: [0; 8] };

    mmio.otc_dma.base_address.set(&otc.entries[7]);
    mmio.otc_dma.block_control.set(8);
    let clear_otc = mmio
        .otc_dma
        .channel_control
        .set_sync_mode(SyncMode::Immediate)
        .start(());
    clear_otc.wait();

    otc.add_prim(4, &mut prim0.tag).add_prim(4, &mut prim1.tag);

    gp1.borrow_mut().dma_direction(2);
    mmio.gpu_dma.base_address.set(&otc.entries[5]);
    mmio.gpu_dma.block_control.set(BlockSize::LinkedList);
    let draw_prim = mmio
        .gpu_dma
        .channel_control
        .set_direction(Direction::FromMemory)
        .set_sync_mode(SyncMode::LinkedList)
        .start(());
    draw_prim.wait();

    fb.swap();
    loop {}
}

struct OTC {
    entries: [u32; 8],
}

impl OTC {
    fn add_prim(&mut self, z: usize, tag: &mut u32) -> &mut Self {
        *tag &= !0x00FF_FFFF;
        unsafe {
            *tag |= self.entries[z];
            self.entries[z] = core::mem::transmute::<_, u32>(tag) & 0x00FF_FFFF;
        }
        self
    }
}
