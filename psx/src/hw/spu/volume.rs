//! SPU volume module

use crate::hw::spu::adsr::{EnvelopeDirection, EnvelopeMode, EnvelopePhase};

/// The volume sweep structure.
pub struct VolumeSweep {
    value: u16,
}

impl VolumeSweep {
    /// The volume sweep mode.
    pub fn mode(&self) -> EnvelopeMode {
        match (self.value >> 14) & 1 {
            0 => EnvelopeMode::Linear,
            1 => EnvelopeMode::Exponential,
            _ => unreachable!(),
        }
    }

    /// The volume sweep direction.
    pub fn direction(&self) -> EnvelopeDirection {
        match (self.value >> 13) & 1 {
            0 => EnvelopeDirection::Increase,
            1 => EnvelopeDirection::Decrease,
            _ => unreachable!(),
        }
    }

    /// The volume sweep phase.
    pub fn phase(&self) -> EnvelopePhase {
        match (self.value >> 12) & 1 {
            0 => EnvelopePhase::Positive,
            1 => EnvelopePhase::Negative,
            _ => unreachable!(),
        }
    }

    /// The volume sweep shift.
    pub fn shift(&self) -> u8 {
        ((self.value >> 2) & 0x1F) as u8
    }

    /// The volume sweep step.
    pub fn step(&self) -> u8 {
        (self.value & 3) as u8
    }
}

/// SPU volume setting.
pub enum Volume {
    /// Normal volume.
    Normal(i16),
    /// Sweeping volume.
    Sweep(VolumeSweep),
}

impl From<u16> for Volume {
    fn from(value: u16) -> Self {
        match value >> 15 {
            // Sign-extend the 15-bit signed volume
            0 => Volume::Normal(((value << 1) as i16) >> 1),
            1 => Volume::Sweep(VolumeSweep { value }),
            _ => unreachable!(),
        }
    }
}

impl From<&Volume> for u16 {
    fn from(val: &Volume) -> Self {
        match val {
            Volume::Normal(vol) => (*vol as u16) & 0x7FFF,
            Volume::Sweep(sweep) => sweep.value,
        }
    }
}
