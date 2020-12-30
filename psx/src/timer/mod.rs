use core::hint::unreachable_unchecked;

use crate::value;
use crate::value::{Load, LoadMut};

/// Timer 0 registers.
pub mod timer0;

/// Timer 1 registers.
pub mod timer1;

/// A timer's clock source.
pub enum Source {
    /// The system clock reference.
    System = 0,
    /// The alternate clock reference. Timer 0 uses the dotclock, timer1 uses
    /// horizontal blank and timer 2 uses the system clock reference divided by
    /// 8.
    Alternate,
}

/// A timer's synchronization mode.
// TODO: Fix this for timer 2
pub enum SyncMode {
    /// Pause counter during Hblank (timer 0)/Vblank (timer 1).
    Pause = 0,
    /// Reset counter at Hblank (timer 0)/Vblank (timer 1).
    Reset,
    /// Reset counter at Hblank (timer 0)/Vblank (timer 1) and pause outside.
    Count,
    /// Pause until Hblank (timer 0)/Vblank (timer 1) occurs once then switch to
    /// free-running
    FreeRun,
}

/// A marker to timer counter registers.
pub trait TimerCounter: Load<u16> {
    /// Waits until the counter stops.
    #[inline(always)]
    fn wait(&mut self) -> u16 {
        let mut bits = self.load().bits;
        while self.load().bits != bits {
            bits = self.load().bits;
        }
        bits
    }
}

/// A marker for timer mode registers.
#[allow(missing_docs)]
pub trait TimerMode: LoadMut<u16> {
    const SYNC_MODE: u32 = 1;
    const SOURCE: u32 = 8;
}

/// A [`value::Value`] alias for the timer mode registers.
pub type Value<'r, R> = value::Value<'r, u16, R>;
/// A [`value::MutValue`] alias for the timer mode registers.
pub type MutValue<'r, R> = value::MutValue<'r, u16, R>;

impl<R: TimerMode> Value<'_, R> {
    /// Gets the synchronization mode.
    #[inline(always)]
    pub fn sync_mode(&self) -> SyncMode {
        match (self.bits >> R::SYNC_MODE) & 0b11 {
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
        if self.contains(1 << R::SOURCE) {
            Source::Alternate
        } else {
            Source::System
        }
    }
}

impl<R: TimerMode> MutValue<'_, R> {
    /// Sets the synchronization mode.
    #[inline(always)]
    pub fn sync_mode(self, mode: SyncMode) -> Self {
        self.set((mode as u16) << R::SYNC_MODE)
    }

    /// Sets the clock source.
    #[inline(always)]
    pub fn source(self, src: Source) -> Self {
        self.set((src as u16) << R::SOURCE)
    }
}
