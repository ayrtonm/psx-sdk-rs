use core::hint::unreachable_unchecked;

use crate::mmio::Address;
use crate::value;
use crate::value::{Load, LoadMut};

/// The clock source for timer 1.
pub enum Source {
    /// The system clock reference.
    System = 0,
    /// The horizontal blanking period.
    Hblank,
}

/// The synchronization mode for timer 1.
pub enum SyncMode {
    /// Pause counter during Vblank.
    Pause = 0,
    /// Reset counter at Vblank.
    Reset,
    /// Reset counter at Vblank and pause outside Vblank.
    Count,
    /// Pause until Vblank occurs once then switch to free-running
    FreeRun,
}

/// [Timer 1 mode](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1114`.
/// Used to configure timer 1.
pub struct MODE;

/// [Timer 1 counter](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1110`.
/// Contains the timer's current value.
pub struct CNT;

/// [Timer 1 target](http://problemkaputt.de/psx-spx.htm#timers) register at `0x1F80_1118`.
/// Contains the timer's target value.
pub struct TGT;

impl Address<u32> for MODE {
    const ADDRESS: u32 = 0x1F80_1114;
}

impl LoadMut<u32> for MODE {}

impl Address<u16> for CNT {
    const ADDRESS: u32 = 0x1F80_1110;
}

impl LoadMut<u16> for CNT {}

impl Address<u16> for TGT {
    const ADDRESS: u32 = 0x1F80_1118;
}

impl LoadMut<u16> for TGT {}

impl MODE {
    const SYNC_MODE: u32 = 1;
    const SOURCE: u32 = 8;
}

impl CNT {
    /// Waits until the counter stops.
    #[inline(always)]
    pub fn wait(&mut self) -> u16 {
        let mut bits = self.load().bits;
        while self.load().bits != bits {
            bits = self.load().bits;
        }
        bits
    }
}

/// A [`value::Value`] alias for the timer 1 mode register.
pub type Value<'r> = value::Value<'r, u32, MODE>;
/// A [`value::MutValue`] alias for the timer 1 mode register.
pub type MutValue<'r> = value::MutValue<'r, u32, MODE>;

impl Value<'_> {
    /// Gets the synchronization mode.
    #[inline(always)]
    pub fn sync_mode(&self) -> SyncMode {
        match (self.bits >> MODE::SYNC_MODE) & 0b11 {
            0 => SyncMode::Pause,
            1 => SyncMode::Reset,
            2 => SyncMode::Count,
            3 => SyncMode::FreeRun,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    /// Gets the clock source.
    #[inline(always)]
    pub fn source(&self) -> Source {
        if self.contains(1 << MODE::SOURCE) {
            Source::Hblank
        } else {
            Source::System
        }
    }
}

impl MutValue<'_> {
    /// Sets the synchronization mode.
    #[inline(always)]
    pub fn sync_mode(self, mode: SyncMode) -> Self {
        self.set((mode as u32) << MODE::SYNC_MODE)
    }

    /// Sets the clock source.
    #[inline(always)]
    pub fn source(self, src: Source) -> Self {
        self.set((src as u32) << MODE::SOURCE)
    }
}
