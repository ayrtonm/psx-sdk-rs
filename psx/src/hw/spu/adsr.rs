/// Envelope mode
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EnvelopeMode {
    /// The envelope advances linearly
    Linear = 0,
    /// The envelope advances exponentially
    Exponential = 1,
}

/// Envelope direction
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EnvelopeDirection {
    /// The envelope increases up to a specified level.
    Increase = 0,
    /// The envelope decreases down to a specified level.
    Decrease = 1,
}

/// Envelope phase (never used in ADSR.)
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EnvelopePhase {
    /// The envelope advances normally (increase => +0x7FFF, decrease => 0x0000)
    Positive = 0,
    /// The envelope advances from the negative sign (increase => -0x7FFF,
    /// decrease => 0x0000)
    Negative = 1,
}

/// ADSR register value.
pub struct ADSR {
    value: u32,
}

impl ADSR {
    /// Return the ADSR register bits from this structure.
    pub fn to_bits(&self) -> u32 {
        self.value
    }
}

/// [`ADSR`] builder. This lets you automatically construct an ADSR register
/// value, best used in a const context for compile-time computation.
///
/// # Example
///
/// ```
/// # use core::assert_eq;
/// # use psx::hw::spu::adsr::ADSRBuilder;
/// # use psx::hw::spu::adsr::EnvelopeMode;
/// # use psx::hw::spu::adsr::EnvelopeDirection;
/// let env = const { ADSRBuilder::new()
///     .attack(EnvelopeMode::Exponential, 0x02, 2)
///     .decay(0x08)
///     .sustain(0x9, EnvelopeMode::Linear, EnvelopeDirection::Decrease, 0x0B, 3)
///     .release(EnvelopeMode::Exponential, 0x1A)
///     .build() };
///
/// assert_eq!(env.to_bits(), 0b0100_1011_1111_1010_1000_1010_1000_1001);
/// ```
pub struct ADSRBuilder<const A: bool, const D: bool, const S: bool, const R: bool> {
    attack_mode: EnvelopeMode,
    attack_shift: u8,
    attack_step: u8,
    decay_shift: u8,
    sustain_level: u8,
    sustain_mode: EnvelopeMode,
    sustain_direction: EnvelopeDirection,
    sustain_shift: u8,
    sustain_step: u8,
    release_mode: EnvelopeMode,
    release_shift: u8,
}

impl ADSRBuilder<false, false, false, false> {
    /// Construct a new ADSR envelope builder.
    pub const fn new() -> ADSRBuilder<false, false, false, false> {
        ADSRBuilder {
            attack_mode: EnvelopeMode::Linear,
            attack_shift: 0,
            attack_step: 0,
            decay_shift: 0,
            sustain_level: 0,
            sustain_mode: EnvelopeMode::Linear,
            sustain_direction: EnvelopeDirection::Increase,
            sustain_shift: 0,
            sustain_step: 0,
            release_mode: EnvelopeMode::Linear,
            release_shift: 0,
        }
    }
}

impl<const D: bool, const S: bool, const R: bool> ADSRBuilder<false, D, S, R> {
    /// Set the registers related to the Attack stage of the ADSR envelope. The
    /// envelope direction cannot be set (it's always set to
    /// [`EnvelopeDirection::Increase`])
    ///
    /// # Parameters
    ///
    /// * `mode`: Whether the Attack stage should increase from `0` to `0x7FFF`
    ///   exponentially or linearly.
    /// * `shift`: Coarse speed. Can be set within `0x00` to `0x1F` (faster to
    ///   slower)
    /// * `step`: Fine speed. Can be set within `0` to `3` (faster to slower)
    pub const fn attack(
        self, mode: EnvelopeMode, shift: u8, step: u8,
    ) -> ADSRBuilder<true, D, S, R> {
        if shift > 0x1F {
            panic!("ADSR attack shift is bigger than the maximum value possible (0x1F)");
        }

        if step > 3 {
            panic!("ADSR attack step is bigger than the maximum value possible (0x03)");
        }

        ADSRBuilder {
            attack_mode: mode,
            attack_shift: shift,
            attack_step: step,
            decay_shift: self.decay_shift,
            sustain_level: self.sustain_level,
            sustain_mode: self.sustain_mode,
            sustain_direction: self.sustain_direction,
            sustain_shift: self.sustain_shift,
            sustain_step: self.sustain_step,
            release_mode: self.release_mode,
            release_shift: self.release_shift,
        }
    }
}

impl<const A: bool, const S: bool, const R: bool> ADSRBuilder<A, false, S, R> {
    /// Set the registers related to the Decay stage of the ADSR envelope. Only
    /// `shift` can be set here.
    ///
    /// # Fixed parameters
    ///
    /// * `mode`: [`EnvelopeMode::Exponential`]
    /// * `direction`: [`EnvelopeDirection::Decrease`]
    /// * `step`: `0`
    ///
    /// # Parameters
    ///
    /// * `shift`: Coarse speed. Cant be set within `0x00` to `0x0F` (faster to
    ///   slower).
    pub const fn decay(self, shift: u8) -> ADSRBuilder<A, true, S, R> {
        if shift > 0x0F {
            panic!("ADSR decay shift is bigger than the maximum value possible (0x0F)");
        }

        ADSRBuilder {
            attack_mode: self.attack_mode,
            attack_shift: self.attack_shift,
            attack_step: self.attack_step,
            decay_shift: shift,
            sustain_level: self.sustain_level,
            sustain_mode: self.sustain_mode,
            sustain_direction: self.sustain_direction,
            sustain_shift: self.sustain_shift,
            sustain_step: self.sustain_step,
            release_mode: self.release_mode,
            release_shift: self.release_shift,
        }
    }
}

impl<const A: bool, const D: bool, const R: bool> ADSRBuilder<A, D, false, R> {
    /// Set the registers related to the Sustain stage of the ADSR envelope.
    ///
    /// # Parameters
    ///
    /// * `level`: Sustain level. This controls when the Decay stage will end
    ///   (when the ADSR volume reaches `(level + 1) * 0x800`). Can be set
    ///   within `0x00` to `0x0F`.
    /// * `mode`: Whether the Sustain stage should advance the volume
    ///   exponentially or linearly.
    /// * `direction`: Whether the Sustain stage should increase or decrease the
    ///   volume.
    /// * `shift`: Coarse speed. Can be set within `0x00` to `0x1F` (faster to
    ///   slower)
    /// * `step`: Fine speed. Can be set within `0` to `3` (faster to slower)
    pub const fn sustain(
        self, level: u8, mode: EnvelopeMode, direction: EnvelopeDirection, shift: u8, step: u8,
    ) -> ADSRBuilder<A, D, true, R> {
        if level > 0x0F {
            panic!("ADSR sustain level is bigger than the maximum value possible (0x0F)");
        }

        if shift > 0x1F {
            panic!("ADSR sustain shift is bigger than the maximum value possible (0x1F)");
        }

        if step > 3 {
            panic!("ADSR sustain step is bigger than the maximum value possible (0x03)");
        }

        ADSRBuilder {
            attack_mode: self.attack_mode,
            attack_shift: self.attack_shift,
            attack_step: self.attack_step,
            decay_shift: self.decay_shift,
            sustain_level: level,
            sustain_mode: mode,
            sustain_direction: direction,
            sustain_shift: shift,
            sustain_step: step,
            release_mode: self.release_mode,
            release_shift: self.release_shift,
        }
    }
}

impl<const A: bool, const D: bool, const S: bool> ADSRBuilder<A, D, S, false> {
    /// Set the registers related to the Release stage of the ADSR envelope.
    /// Only `mode` and `shift` can be set here.
    ///
    /// # Fixed parameters
    ///
    /// * `direction`: [`EnvelopeDirection::Decrease`]
    /// * `step`: `0`
    ///
    /// # Parameters
    ///
    /// * `mode`: Whether the Release stage should decrease down to `0`
    ///   exponentially or linearly.
    /// * `shift`: Coarse speed. Cant be set within `0x00` to `0x1F` (faster to
    ///   slower).
    pub const fn release(self, mode: EnvelopeMode, shift: u8) -> ADSRBuilder<A, D, S, true> {
        if shift > 0x1F {
            panic!("ADSR release shift is bigger than the maximum value possible (0x1F)");
        }

        ADSRBuilder {
            attack_mode: self.attack_mode,
            attack_shift: self.attack_shift,
            attack_step: self.attack_step,
            decay_shift: self.decay_shift,
            sustain_level: self.sustain_level,
            sustain_mode: self.sustain_mode,
            sustain_direction: self.sustain_direction,
            sustain_shift: self.sustain_shift,
            sustain_step: self.sustain_step,
            release_mode: mode,
            release_shift: shift,
        }
    }
}

impl ADSRBuilder<true, true, true, true> {
    /// Build an [`ADSR`]
    pub const fn build(self) -> ADSR {
        let env_value: u32 = ((self.sustain_mode as u32) << 31) |
            ((self.sustain_direction as u32) << 30) |
            ((self.sustain_shift as u32) << 24) |
            ((self.sustain_step as u32) << 22) |
            ((self.release_mode as u32) << 21) |
            ((self.release_shift as u32) << 16) |
            ((self.attack_mode as u32) << 15) |
            ((self.attack_shift as u32) << 10) |
            ((self.attack_step as u32) << 8) |
            ((self.decay_shift as u32) << 4) |
            (self.sustain_level as u32);

        ADSR { value: env_value }
    }
}

#[test_case]
fn adsr_builder_test() {
    let env = const {
        ADSRBuilder::new()
            .attack(EnvelopeMode::Exponential, 0x02, 2)
            .decay(0x08)
            .sustain(
                0x9,
                EnvelopeMode::Linear,
                EnvelopeDirection::Decrease,
                0x0B,
                3,
            )
            .release(EnvelopeMode::Exponential, 0x1A)
            .build()
    };

    assert_eq!(env.to_bits(), 0b0100_1011_1111_1010_1000_1010_1000_1001);
}
