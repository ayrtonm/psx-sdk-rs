use crate::mmio::Address;
use crate::value::{LoadMut, MutValue, Value};

/// [DMA Control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F0`.
/// Used to enable DMA channels and set priorities.
pub struct Control;
/// A [`Value`] for [`Control`].
pub type ControlValue<'r> = Value<'r, u32, Control>;
/// A [`MutValue`] for [`Control`].
pub type ControlMutValue<'r> = MutValue<'r, u32, Control>;

impl Address<u32> for Control {
    const ADDRESS: u32 = 0x1F80_10F0;
}
impl LoadMut<u32> for Control {}

impl ControlValue<'_> {
    /// Checks if the given DMA channel is enabled.
    #[inline(always)]
    pub fn enabled(&self, ch: Channel) -> bool {
        let bit = (ch as u32 * 4) + 3;
        self.bits & bit != 0
    }
}

impl ControlMutValue<'_> {
    /// Enables the given DMA channel.
    #[inline(always)]
    pub fn enable(mut self, ch: Channel) -> Self {
        let bit = (ch as u32 * 4) + 3;
        self.value.bits |= 1 << bit;
        self
    }

    /// Disables the given DMA channel.
    #[inline(always)]
    pub fn disable(mut self, ch: Channel) -> Self {
        let bit = (ch as u32 * 4) + 3;
        self.value.bits &= !(1 << bit);
        self
    }
}

/// A [DMA channel](http://problemkaputt.de/psx-spx.htm#dmachannels).
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// DMA channel for RAM to Macroblock Decoder transfers.
    MDECin = 0,
    /// DMA channel for Macroblock Decoder to RAM transfers.
    MDECout,
    /// DMA channel for Graphics Processing Unit linked lists and image data.
    GPU,
    /// DMA channel for CDROM to RAM transfers.
    CDROM,
    /// DMA channel for RAM to Sound Processing Unit transfers.
    SPU,
    /// DMA channel for Expansion port transfers.
    PIO,
    /// DMA channel for clearing GPU ordering tables.
    OTC,
}
/*

pub struct ControlValue(u32);
pub struct ControlMutValue<'a> {
    value: ControlValue,
    reg: &'a mut Control,
}

impl Value<u32, Control> for ControlValue {
    //fn new(r: &Control) -> Self {
    //    Self(unsafe {
    //        Load::load(r)
    //    })
    //}
    //fn new_mut(r: &mut Control) -> Self {
    //    Self(unsafe {
    //        Load::load(r)
    //    })
    //}
    //fn store(self) {
    //    unsafe {
    //        Store::store(&mut self, self.0)
    //    }
    //}
}
/// [DMA Interrupt](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F4`.
/// Used to enable and force DMA IRQs.
pub struct Interrupt;
impl Load<u32> for Interrupt {
    const ADDRESS: u32 = 0x1F80_10F4;
}

impl MutValue<'_, Control, u32> {
}

/// Marker for DMA base address registers.
pub trait BaseAddress: Load<u32> + Store<u32> {}

/// Marker for DMA block control registers.
pub trait BlockControl: Load<u32> + Store<u32> {}

/// Marker for DMA channel control registers.
pub trait ChannelControl: Load<u32> + Store<u32> {}

impl<R: BaseAddress> Value<R, u32> {
    /// Gets the DMA channel's current base address.
    #[inline(always)]
    pub fn get_address(&self) -> u32 {
        self.bits
    }
}

impl<R: BaseAddress> MutValue<'_, R, u32> {
    /// Sets the DMA channel's base address.
    #[inline(always)]
    pub fn set_address(mut self, addr: u32) -> Self {
        self.bits = addr;
        self
    }
}

/// Methods for using the Graphics Processing Unit DMA channel.
pub mod gpu {
    use super::BaseAddress as BaseAddressTrait;
    use crate::mmio::Load;

    /// [GPU DMA base address](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10A0`.
    /// Used to set the DMA channel's base address.
    pub struct BaseAddress;
    impl Load<u32> for BaseAddress {
        const ADDRESS: u32 = 0x1F80_10A0;
    }

    impl BaseAddressTrait for BaseAddress {}
}
*/
