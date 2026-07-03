//! SPU interface

pub mod adsr;
pub mod reverb;
pub mod volume;

use crate::dma::SPU;
use crate::hw::spu::adsr::ADSR;
use crate::hw::spu::reverb::{ReverbBaseAddress, ReverbOutVolumeLeft, ReverbOutVolumeRight};
use crate::hw::spu::volume::Volume;
use core::{hint::black_box, ops::Range};

use crate::hw::mmio::{MemRegister, SplitU32MemRegister};
use crate::hw::Register;
const SPU_CHANNELS: usize = 24;

const SPU_CHANNEL_REGS: *mut SpuChannelRegs = 0x1F80_1C00 as *mut SpuChannelRegs;

type KeyOn = SplitU32MemRegister<0x1F80_1D88>;
type KeyOff = SplitU32MemRegister<0x1F80_1D8C>;
type NoiseOn = SplitU32MemRegister<0x1F80_1D94>;
type FMOn = SplitU32MemRegister<0x1F80_1D90>;
type EchoOn = SplitU32MemRegister<0x1F80_1D98>;

type TransAddr = MemRegister<u16, 0x1F80_1DA6>;
type Control = MemRegister<u16, 0x1F80_1DAA>;
type TransControl = MemRegister<u16, 0x1F80_1DAC>;
type Status = MemRegister<u16, 0x1F80_1DAE>;

type MainVolLeft = MemRegister<u16, 0x1F80_1D80>;
type MainVolRight = MemRegister<u16, 0x1F80_1D82>;

/// The SPU structure.
pub struct Spu;

#[repr(u8)]
enum SpuRamTransfer {
    Stop = 0,
    ManualWrite = 1,
    DMAWrite = 2,
    DMARead = 3,
}

impl From<SpuRamTransfer> for u16 {
    fn from(value: SpuRamTransfer) -> Self {
        match value {
            SpuRamTransfer::Stop => 0,
            SpuRamTransfer::ManualWrite => 1,
            SpuRamTransfer::DMAWrite => 2,
            SpuRamTransfer::DMARead => 3,
        }
    }
}

enum SpuTransferMode {
    Fill,
    Normal,
    Repeat2,
    Repeat4,
    Repeat8,
}

macro_rules! define_bit {
    ($name:ident, $set_name:ident, $bit:literal) => {
        fn $name(&self) -> bool {
            self.all_set(1 << $bit)
        }

        fn $set_name(&mut self, value: bool) -> &mut Self {
            self.clear_bits((value as u16) << $bit);
            self.set_bits((value as u16) << $bit);
            self
        }
    };
    ($name:ident, $set_name:ident, $mask:literal, $shift:literal) => {
        fn $name(&self) -> u16 {
            (self.to_bits() >> $shift) & $mask
        }

        fn $set_name(&mut self, value: u16) -> &mut Self {
            self.clear_bits($mask << $shift);
            self.set_bits(value << $shift);
            self
        }
    };
}

impl Control {
    define_bit!(enable, set_enable, 15);
    define_bit!(mute, set_mute, 14);
    define_bit!(noise_freq_shift, set_noise_freq_shift, 0b1111, 10);
    define_bit!(noise_freq_step, set_noise_freq_step, 0b11, 8);
    define_bit!(reverb_master, set_reverb_master, 7);
    define_bit!(irq9, set_irq9, 6);

    define_bit!(ram_transfer, set_ram_transfer, 0b11, 4);

    define_bit!(ext_reverb, set_ext_reverb, 3);
    define_bit!(cdda_reverb, set_cdda_reverb, 2);
    define_bit!(ext_enable, set_ext_enable, 1);
    define_bit!(cdda_enable, set_cdda_enable, 0);
}

impl Status {
    define_bit!(capture_buffer, set_capture_buffer, 11);
    define_bit!(transfer_busy, set_transfer_busy, 10);
    define_bit!(dma_read_request, set_dma_read_request, 9);
    define_bit!(dma_write_request, set_dma_write_request, 8);
    define_bit!(dma_rw_request, set_dma_rw_request, 7);
    define_bit!(irq9, set_irq9, 6);
    define_bit!(ram_transfer, set_ram_transfer, 0b11, 4);
}

#[repr(C)]
struct SpuChannelRegs {
    volume_left: u16,
    volume_right: u16,
    frequency: u16,
    sample_start: u16,
    adsr: u32,
    adsr_volume: u16,
    sample_repeat: u16,
}

/// Reference to a SPU channel.
pub struct SpuChannel {
    regs: *mut SpuChannelRegs,
    num: usize,
}

macro_rules! set_volatile {
    ($field:expr, $value:expr) => {
        (&raw mut $field).write_volatile($value);
    };
}

impl SpuChannel {
    /// Resets a channel
    pub fn reset(&mut self) {
        self.frequency(0);
        self.volume(&Volume::Normal(0));
        self.sample_start(0);
        self.key_off();
        self.frequency_mod(false);
        self.noise(false);
        self.reverb(false);
    }

    /// Sets the left volume of a channel.
    pub fn volume_left(&mut self, vol: &Volume) {
        let vol_bits: u16 = vol.into();

        unsafe {
            set_volatile!((*self.regs).volume_left, vol_bits);
        }
    }

    /// Sets the right volume of a channel.
    pub fn volume_right(&mut self, vol: &Volume) {
        let vol_bits: u16 = vol.into();

        unsafe {
            set_volatile!((*self.regs).volume_right, vol_bits);
        }
    }

    /// Sets the volume (both left/right) of a channel.
    pub fn volume(&mut self, vol: &Volume) {
        self.volume_left(vol);
        self.volume_right(vol);
    }

    /// Sets the address of the sample the channel should be playing off of.
    ///
    /// Note: The SPU RAM is only addressable by 8-byte chunks, so the
    /// right-most 3 bits will be ignored.
    pub fn sample_start(&mut self, sample: u32) {
        if sample > 1 << 19 {
            panic!("Sample address is bigger than the maximum addressable address in the SPU");
        }

        // In the SPU, samples are indexed by 8-byte units.
        unsafe {
            set_volatile!((*self.regs).sample_start, sample.unbounded_shr(3) as u16);
        }
    }

    /// Sets the ADPCM sample rate of the channel to the specified frequency
    /// (0x1000 == 441000Hz).
    ///
    /// Note: This does not affect the frequency of the channel if noise mode is
    /// active on it. For that, you should check out
    /// [`Self::noise_settings`]
    pub fn frequency(&mut self, frequency: u16) {
        unsafe {
            set_volatile!((*self.regs).frequency, frequency);
        }
    }

    /// Sets the ADSR envelope of the channel.
    pub fn adsr(&mut self, env: ADSR) {
        unsafe {
            set_volatile!((*self.regs).adsr, env.to_bits());
        }
    }

    /// Starts the ADSR envelope and automatically initializes the ADSR volume
    /// to zero
    pub fn key_on(&self) {
        KeyOn::new().set_bits(1 << self.num).store();
    }

    /// Releases the key in the channel, which starts the Release stage of the
    /// ADSR envelope, if set.
    pub fn key_off(&self) {
        KeyOff::new().set_bits(1 << self.num).store();
    }

    /// Enable or disable noise mode on a specific channel. If enabled, the
    /// channel will stop outputting ADPCM samples and instead output noise
    /// samples from the SPU's Noise Generator.
    ///
    /// The Noise Generator can be configured, using the
    /// [`Self::noise_settings`] function.
    pub fn noise(&self, enable: bool) {
        NoiseOn::new()
            .clear_bits((enable as u32) << self.num)
            .set_bits((enable as u32) << self.num)
            .store();
    }

    /// Enables or disables frequency modulation of the specified channel from
    /// the amplitude of the previous channel.
    ///
    /// Note: Setting frequency modulation on channel 0 will do nothing, as
    /// there is no previous channel.
    pub fn frequency_mod(&self, enable: bool) {
        FMOn::new()
            .clear_bits((enable as u32) << self.num)
            .set_bits((enable as u32) << self.num)
            .store();
    }

    /// Enables or disables reverb in the specified channel.
    pub fn reverb(&self, enable: bool) {
        EchoOn::new()
            .clear_bits((enable as u32) << self.num)
            .set_bits((enable as u32) << self.num)
            .store();
    }
}

/// SPU channel iterator
pub struct ChannelIterator<'a> {
    spu: &'a Spu,
    channels: Range<usize>,
}

impl<'a> Iterator for ChannelIterator<'a> {
    type Item = SpuChannel;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(num) = self.channels.next() {
            // SAFETY: The channel iterator is always initialized from [`Spu::channels()`],
            // with the range being set from 0 to [`SPU_CHANNELS`]
            unsafe { Some(self.spu.unchecked_channel(num)) }
        } else {
            None
        }
    }
}
impl<'a> ExactSizeIterator for ChannelIterator<'a> {
    fn len(&self) -> usize {
        self.channels.len()
    }
}

impl Spu {
    fn enable_and_unmute_spu(&mut self) {
        let mut control = Control::new();
        control.set_enable(true).set_mute(true).store();
    }

    /// Initializes the SPU to default values
    pub fn reset(&mut self) {
        // Usually the BIOS should initialize the SPU control registers for us (during
        // the intro execution), but if we use fastboot, then we have to
        // initialize the registers ourselves.
        self.enable_and_unmute_spu();
        self.noise_settings(0, 0);
        self.main_volume(&Volume::Normal(0x3FFF));

        self.channels().for_each(|mut channel| {
            channel.reset();
        });
    }

    /// Creates a new handle to the SPU structure.
    pub fn new() -> Self {
        Spu
    }

    /// Gets a specific channel from the SPu without checking bounds.
    ///
    /// # Safety
    ///
    /// Calling this method with non-existent channel number is *undefined
    /// behavior*. You must make sure that the channel is within the range
    /// of the channels that the SPU has.
    pub unsafe fn unchecked_channel(&self, channel: usize) -> SpuChannel {
        unsafe {
            SpuChannel {
                regs: &mut (*SPU_CHANNEL_REGS.wrapping_add(channel)),
                num: channel,
            }
        }
    }

    /// Gets a specific channel from the SPU. If the channel number is not in
    /// range of the amount of channels that the SPU has, it'll return
    /// [`None`].
    pub fn channel(&self, channel: usize) -> Option<SpuChannel> {
        if channel < SPU_CHANNELS {
            Some(unsafe { self.unchecked_channel(channel) })
        } else {
            None
        }
    }

    /// Returns an iterator over the SPU's channels.
    pub fn channels(&self) -> ChannelIterator<'_> {
        ChannelIterator {
            spu: self,
            channels: (0..SPU_CHANNELS),
        }
    }

    /// Sets the SPU's main left volume.
    pub fn main_volume_left(&mut self, vol: &Volume) {
        let vol_bits: u16 = vol.into();

        MainVolLeft::skip_load().assign(vol_bits).store();
    }

    /// Sets the SPU's main right volume.
    pub fn main_volume_right(&mut self, vol: &Volume) {
        let vol_bits: u16 = vol.into();

        MainVolRight::skip_load().assign(vol_bits).store();
    }

    /// Sets the SPU's main volume.
    pub fn main_volume(&mut self, vol: &Volume) {
        self.main_volume_left(vol);
        self.main_volume_right(vol);
    }

    /// Configure the Noise Generator for all channels that have noise mode
    /// enabled.
    ///
    /// `step` finetunes the frequency of the noise output (by skipping over
    /// steps in the timer), while `shift` coarsely tunes the frequency (by
    /// the shifting the initial value of the timer)
    ///
    /// See [The PlayStation Specifications](https://psx-spx.consoledev.net/soundprocessingunitspu/#spu-noise-generator_1) for more details.
    pub fn noise_settings(&mut self, shift: usize, step: usize) {
        let mut control = Control::new();
        control
            .set_noise_freq_shift(shift as u16)
            .set_noise_freq_step(step as u16)
            .store();
    }

    fn set_ram_transfer(&mut self, mode: SpuRamTransfer) {
        let mode_val: u16 = mode.into();
        let mut control = Control::new();
        let mut status = Status::skip_load();

        // Change the RAM transfer mode.
        control.set_ram_transfer(mode_val).store();

        // Wait until the change gets applied to the SPU.
        while status.load().ram_transfer() != mode_val {}
    }

    fn set_transfer_mode(&mut self, mode: SpuTransferMode) {
        let mut trans_cnt = TransControl::skip_load();
        match mode {
            SpuTransferMode::Fill => trans_cnt.assign(0),
            SpuTransferMode::Normal => trans_cnt.assign(2 << 1),
            SpuTransferMode::Repeat2 => trans_cnt.assign(3 << 1),
            SpuTransferMode::Repeat4 => trans_cnt.assign(4 << 1),
            SpuTransferMode::Repeat8 => trans_cnt.assign(5 << 1),
        }
        .store();
    }

    fn set_transfer_address(&mut self, address: u32) {
        TransAddr::skip_load()
            .assign(address.unbounded_shr(3) as u16)
            .store();
    }

    /// Write data to an address in the SPU's RAM, without using the SPU's DMA
    /// channel.
    ///
    /// Note: The SPU RAM is only addressable by 8-byte chunks, so the
    /// right-most 3 address bits will be ignored.
    pub fn write_cpu(&mut self, mut address: u32, data: &[u16]) {
        const SPU_FIFO: *mut u16 = 0x1F80_1DA8 as _;

        // Set the SPU transfer mode to normal.
        self.set_transfer_mode(SpuTransferMode::Normal);

        // Set the RAM transfer mode to "Stop" in SPUCNT
        self.set_ram_transfer(SpuRamTransfer::Stop);

        for chunk in data.chunks(32) {
            // Set the address to transfer data to.
            self.set_transfer_address(address);

            address += chunk.len() as u32 * 2;

            // Send each half-word to the SPU's FIFO (the FIFO only has space for 32.)
            for word in chunk {
                unsafe {
                    SPU_FIFO.write_volatile(*word);
                }
            }

            // Set the RAM transfer mode to "Manual Write" now.
            self.set_ram_transfer(SpuRamTransfer::ManualWrite);

            let mut spu_status = Status::skip_load();

            // Wait for the Transfer Busy flag to go off.
            while spu_status.load().transfer_busy() {}

            // The additional delay is required for multi-block transfers according to
            // nocash's docs
            for i in 0..1000 {
                black_box(i);
            }
        }
    }

    /// Initialize a DMA write operation to an address in the SPU's RAM.
    ///
    /// Note: The SPU RAM is only addressable by 8-byte chunks, so the
    /// right-most 3 address bits will be ignored.
    pub fn write_dma(&mut self, address: u32) -> SPU {
        // Set the SPU transfer mode to normal.
        self.set_transfer_mode(SpuTransferMode::Normal);

        // Set the RAM transfer mode to "Stop" in SPUCNT
        self.set_ram_transfer(SpuRamTransfer::Stop);

        // Set the address to transfer data to.
        self.set_transfer_address(address);

        // Set the RAM transfer mode to DMA Write
        self.set_ram_transfer(SpuRamTransfer::DMAWrite);

        // Now it's up to the user to initiate the transfer over at the CPU-side with
        // the DMA channel.
        SPU::new()
    }

    /// Configure the reverb registers of the SPU.
    pub fn reverb(&self) -> reverb::SpuReverb {
        reverb::SpuReverb {
            out_volume_left: ReverbOutVolumeLeft::new(),
            out_volume_right: ReverbOutVolumeRight::new(),
            base_address: ReverbBaseAddress::new(),
        }
    }
}
