//! Memory-mapped I/O registers.
//!
//! The PlayStation accesses peripherals through [memory-mapped I/O
//! registers](http://problemkaputt.de/psx-spx.htm#iomap). This module defines zero-sized types for
//! read and/or write access to each register and an [`MMIO`] struct to hold
//! them.

// These struct constructors are all pub(crate) for use in Unsafe wrappers, but not all are used
#![allow(dead_code)]
mod macros;
pub mod register;

pub mod joy {
    // TODO: This is technically a 1-byte register. Would a 4-byte write be a
    // problem?
    read_write!(Data, 0x1F80_1040);
    read_only!(Stat, 0x1F80_1044);
    // TODO: These are overlapping 16-bit r/w registers. They'll require new
    // register macros. TODO: Splitting up Mode and Ctrl will prevent
    // optimizing individual 16-bit accesses to 32-bit accesses so it might
    // be best to combine them read_write!(Mode, 0x1F80_1048);
    //read_write!(Ctrl, 0x1F80_104A);
    // TODO: This doesn't overlap with anything, but is an offset 16-bit
    // register read_write!(Baud, 0x1F80_104E);
}
// TODO: These are technically 2-byte registers. But nocash is a bit vague about
// how legit it is to access them like that
/// Interrupt status and mask registers
pub mod irq {
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
                pub struct Channel {
                    pub base_address: BaseAddress,
                    pub block_control: BlockControl,
                    pub channel_control: ChannelControl,
                    // Prevents instantiation
                    _unused: (),
                }
                impl Channel {
                    pub(crate) unsafe fn new() -> Self {
                        Self {
                            base_address: BaseAddress::new(),
                            block_control: BlockControl::new(),
                            channel_control: ChannelControl::new(),
                            _unused: (),
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

macro_rules! timer_registers {
    ($offset:expr) => {
        paste::paste! {
            pub mod [<timer $offset>] {
                read_write!(Current, 0x1F80_1100 + ($offset * 0x10));
                read_write!(Mode, 0x1F80_1104 + ($offset * 0x10));
                read_write!(Target, 0x1F80_1108 + ($offset * 0x10));
                pub struct Timer {
                    pub current: Current,
                    pub mode: Mode,
                    pub target: Target,
                    // Prevents instantiation
                    _unused: (),
                }
            }
        }
    };
}
timer_registers!(0);
timer_registers!(1);
timer_registers!(2);

// TODO: MMIO must always be zero-sized. I should find a way to add static
// assertions to ensure this
/// Contains an instance of each I/O register defined in this module
pub struct MMIO {
    pub timer0: timer0::Timer,
    pub timer1: timer1::Timer,
    pub timer2: timer2::Timer,

    pub joy_data: joy::Data,
    pub joy_stat: joy::Stat,

    pub irq_stat: irq::Stat,
    pub irq_mask: irq::Mask,

    pub gpu_read: gpu::Read,
    pub gpu_stat: gpu::Stat,
    pub gp0: gpu::GP0,
    pub gp1: gpu::GP1,

    pub dma_control: dma::Control,
    pub dma_interrupt: dma::Interrupt,

    pub mdec_in_dma: dma::mdec_in::Channel,
    pub mdec_out_dma: dma::mdec_out::Channel,
    pub gpu_dma: dma::gpu::Channel,
    pub cdrom_dma: dma::cdrom::Channel,
    pub spu_dma: dma::spu::Channel,
    pub pio_dma: dma::pio::Channel,
    pub otc_dma: dma::otc::Channel,
    // Prevents instantiation
    _unused: (),
}
