//! Memory-mapped I/O registers.
//!
//! The PlayStation accesses peripherals through [memory-mapped I/O
//! registers](http://problemkaputt.de/psx-spx.htm#iomap). This module defines zero-sized types for
//! read and/or write access to each register and an [`MMIO`] struct to hold
//! them.

mod macros;
pub(crate) mod register;

pub mod interrupt {
    read_write!(Stat, 0x1F80_1070);
    read_write!(Mask, 0x1F80_1074);
}

pub mod gpu {
    read_only!(Read, 0x1F80_1810);
    write_only!(GP0, 0x1F80_1810);

    read_only!(Stat, 0x1F80_1814);
    write_only!(GP1, 0x1F80_1814);
}

pub mod dma {
    macro_rules! dma_channel {
        ($name:ident, $offset:expr) => {
            pub mod $name {
                pub struct Channel {
                    pub base_address: BaseAddress,
                    pub block_control: BlockControl,
                    pub channel_control: ChannelControl,
                }
                impl Channel {
                    pub(crate) unsafe fn new() -> Self {
                        Channel {
                            base_address: BaseAddress::new(),
                            block_control: BlockControl::new(),
                            channel_control: ChannelControl::new(),
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

pub struct MMIO {
    pub gpu_read: gpu::Read,
    pub gpu_stat: gpu::Stat,
    pub gp0: gpu::GP0,
    pub gp1: gpu::GP1,

    pub int_stat: interrupt::Stat,
    pub int_mask: interrupt::Mask,

    pub dma_control: dma::Control,
    pub dma_interrupt: dma::Interrupt,

    pub mdec_in_dma: dma::mdec_in::Channel,
    pub mdec_out_dma: dma::mdec_out::Channel,
    pub gpu_dma: dma::gpu::Channel,
    pub cdrom_dma: dma::cdrom::Channel,
    pub spu_dma: dma::spu::Channel,
    pub pio_dma: dma::pio::Channel,
    pub otc_dma: dma::otc::Channel,
}

impl MMIO {
    pub unsafe fn new() -> Self {
        MMIO {
            gpu_read: gpu::Read::new(),
            gpu_stat: gpu::Stat::new(),
            gp0: gpu::GP0::new(),
            gp1: gpu::GP1::new(),

            int_stat: interrupt::Stat::new(),
            int_mask: interrupt::Mask::new(),

            dma_control: dma::Control::new(),
            dma_interrupt: dma::Interrupt::new(),

            mdec_in_dma: dma::mdec_in::Channel::new(),
            mdec_out_dma: dma::mdec_out::Channel::new(),
            gpu_dma: dma::gpu::Channel::new(),
            cdrom_dma: dma::cdrom::Channel::new(),
            spu_dma: dma::spu::Channel::new(),
            pio_dma: dma::pio::Channel::new(),
            otc_dma: dma::otc::Channel::new(),
        }
    }
}
