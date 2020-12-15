//! Memory-mapped I/O registers.
//!
//! The PlayStation accesses peripherals through [memory-mapped I/O
//! registers](http://problemkaputt.de/psx-spx.htm#iomap). This module defines zero-sized types for
//! read and/or write access to each register and an [`MMIO`] struct to hold
//! them.

// These struct constructors are all pub(crate) for use in Unsafe wrappers, but not all are used
#![allow(dead_code)]
mod macros;
pub(crate) mod register;

pub trait MMIOState {}
pub struct Enabled {}
pub struct Disabled {}
impl MMIOState for Enabled {}
impl MMIOState for Disabled {}

/// Interrupt status and mask registers
pub mod int {
    read_write!(
        /// Read and acknowledge IRQs.
        Stat,
        0x1F80_1070
    );
    read_write!(
        /// Enable and disable IRQs.
        Mask,
        0x1F80_1074
    );
}

/// GPU I/O registers
pub mod gpu {
    read_only!(
        /// Receives responses to GP0(C0h) and GP1(10h) commands.
        Read,
        0x1F80_1810
    );
    write_only!(
        /// Sends GP0 commands and packets for rendering and VRAM access.
        GP0,
        0x1F80_1810
    );

    read_only!(
        /// GPU status register
        Stat,
        0x1F80_1814
    );
    write_only!(
        /// Sends GP1 commands for display and DMA control.
        GP1,
        0x1F80_1814
    );
}

/// DMA channel, control and interrupt registers
pub mod dma {
    macro_rules! dma_channel {
        ($name:ident, $offset:expr) => {
            pub mod $name {
                use crate::mmio::{Disabled, Enabled, MMIOState};
                use core::marker::PhantomData;
                pub struct Channel<T: MMIOState> {
                    pub base_address: BaseAddress,
                    pub block_control: BlockControl,
                    pub channel_control: ChannelControl,
                    _state: PhantomData<T>,
                }
                impl Channel<Disabled> {
                    pub(crate) unsafe fn new() -> Self {
                        Channel {
                            base_address: BaseAddress::new(),
                            block_control: BlockControl::new(),
                            channel_control: ChannelControl::new(),
                            _state: PhantomData::<Disabled>,
                        }
                    }

                    pub fn enable(self, dma_control: &mut super::Control) -> Channel<Enabled> {
                        unsafe {
                            dma_control.$name(true);
                            Channel {
                                base_address: BaseAddress::new(),
                                block_control: BlockControl::new(),
                                channel_control: ChannelControl::new(),
                                _state: PhantomData::<Enabled>,
                            }
                        }
                    }
                }
                read_write!(BaseAddress, 0x1F80_1080 + ($offset * 0x10));
                read_write!(BlockControl, 0x1F80_1084 + ($offset * 0x10));
                read_write!(ChannelControl, 0x1F80_1088 + ($offset * 0x10));

                impl crate::dma::BaseAddress for BaseAddress {}
                impl crate::dma::BlockControl for BlockControl {}
                impl crate::dma::ChannelControl for ChannelControl {}
            }
        };
    }

    read_write!(Control, 0x1F80_10F0);
    read_write!(Interrupt, 0x1F80_10F4);
    dma_channel!(mdec_in, 0);
    dma_channel!(mdec_out, 1);
    dma_channel!(gpu, 2);
    dma_channel!(cdrom, 3);
    dma_channel!(spu, 4);
    dma_channel!(pio, 5);
    dma_channel!(otc, 6);
}

// TODO: MMIO must always be zero-sized. I should find a way to add static
// assertions to ensure this
/// Contains an instance of each I/O register defined in this module
pub struct MMIO {
    pub gpu_read: gpu::Read,
    pub gpu_stat: gpu::Stat,
    pub gp0: gpu::GP0,
    pub gp1: gpu::GP1,

    pub int_stat: int::Stat,
    pub int_mask: int::Mask,

    pub dma_control: dma::Control,
    pub dma_interrupt: dma::Interrupt,

    pub mdec_in_dma: dma::mdec_in::Channel<Disabled>,
    pub mdec_out_dma: dma::mdec_out::Channel<Disabled>,
    pub gpu_dma: dma::gpu::Channel<Disabled>,
    pub cdrom_dma: dma::cdrom::Channel<Disabled>,
    pub spu_dma: dma::spu::Channel<Disabled>,
    pub pio_dma: dma::pio::Channel<Disabled>,
    pub otc_dma: dma::otc::Channel<Disabled>,
    // Prevents instantiation
    _unused: (),
}
