//! SPU reverb module.
use crate::hw::mmio::MemRegister;
use crate::hw::Register;

pub(crate) type ReverbOutVolumeLeft = MemRegister<i16, 0x1F80_1D84>;
pub(crate) type ReverbOutVolumeRight = MemRegister<i16, 0x1F80_1D86>;
pub(crate) type ReverbBaseAddress = MemRegister<u16, 0x1F80_1DA2>;

/// The SPU's reverb structure.
pub struct SpuReverb {
    pub(crate) out_volume_left: ReverbOutVolumeLeft,
    pub(crate) out_volume_right: ReverbOutVolumeRight,
    pub(crate) base_address: ReverbBaseAddress,
}

impl SpuReverb {
    /// Set the volume for the output left channel of the reverb.
    pub fn volume_left(&mut self, volume: i16) {
        self.out_volume_left.assign(volume).store();
    }

    /// Set the volume for the output right channel of the reverb.
    pub fn volume_right(&mut self, volume: i16) {
        self.out_volume_right.assign(volume).store();
    }

    /// Set the volume for both left/right output channels of the reverb.
    pub fn volume(&mut self, volume: i16) {
        self.volume_right(volume);
        self.volume_left(volume);
    }

    /// Set the base address within the SPU's RAM for the hardware reverb to
    /// use.
    pub fn base_addr(&mut self, address: u16) {
        self.base_address.assign(address).store();
    }

    /// Set the reverb register settings, from the first APF offset register
    /// (dAPF1) to the input right volume for the reverg (vRIN), which is
    /// about 32 16-bit registers.
    pub fn config(&mut self, config: &[u16; 0x1F]) {
        const SPU_REVERB_SETTINGS: *mut u16 = 0x1F80_1DC0 as _;

        // SAFETY: The size of the configuration data is as big as the area of reverb
        // registers, therefore there isn't a risk of some buffer overflow
        // happening.
        //
        // Unfortunately, since we can't use [`volatile_copy_nonoverlapping_memory`]
        // yet, we have to write our own volatile memcpy loop
        for (i, word) in config.iter().enumerate() {
            unsafe {
                SPU_REVERB_SETTINGS.add(i).write_volatile(*word);
            }
        }
    }
}
