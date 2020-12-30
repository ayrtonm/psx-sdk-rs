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
    #[cfg_attr(feature = "inline_hints", inline(always))]
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
    const TARGET_RESET: u32 = 3;
    const TARGET_IRQ: u32 = 4;
    const OVERFLOW_IRQ: u32 = 5;
    const REPEAT_MODE: u32 = 6;
    const SOURCE: u32 = 8;
    const REACHED_TARGET: u32 = 11;
    const REACHED_OVERFLOW: u32 = 12;
}

/// A [`value::Value`] alias for the timer mode registers.
pub type Value<'r, R> = value::Value<'r, u16, R>;
/// A [`value::MutValue`] alias for the timer mode registers.
pub type MutValue<'r, R> = value::MutValue<'r, u16, R>;

impl<R: TimerMode> Value<'_, R> {
    /// Gets the synchronization mode.
    #[cfg_attr(feature = "inline_hints", inline(always))]
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
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn source(&self) -> Source {
        if self.contains(1 << R::SOURCE) {
            Source::Alternate
        } else {
            Source::System
        }
    }

    /// Checks if the counter will reset after hitting the target.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn target_reset(&self) -> bool {
        self.contains(1 << R::TARGET_RESET)
    }

    /// Checks if the counter will trigger an IRQ after hitting the target.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn target_irq(&self) -> bool {
        self.contains(1 << R::TARGET_IRQ)
    }

    /// Checks if the counter will trigger an IRQ after overflowing.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn overflow_irq(&self) -> bool {
        self.contains(1 << R::OVERFLOW_IRQ)
    }

    /// Checks if the counter reached its target.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn reached_target(&self) -> bool {
        self.contains(1 << R::REACHED_TARGET)
    }

    /// Checks if the counter overflowed.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn reached_overflow(&self) -> bool {
        self.contains(1 << R::REACHED_OVERFLOW)
    }

    /// Checks if oneshot mode is enabled.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn oneshot_mode(&self) -> bool {
        self.cleared(1 << R::REPEAT_MODE)
    }
}

impl<R: TimerMode> MutValue<'_, R> {
    /// Sets the synchronization mode.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn sync_mode(self, mode: SyncMode) -> Self {
        self.clear(0b11 << R::SYNC_MODE)
            .set((mode as u16) << R::SYNC_MODE)
    }

    /// Sets the clock source.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn source(self, src: Source) -> Self {
        self.clear(1 << R::SOURCE).set((src as u16) << R::SOURCE)
    }

    /// Sets whether the counter resets after hitting the target or not.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn target_reset(self, reset: bool) -> Self {
        self.clear(1 << R::TARGET_RESET)
            .set((reset as u16) << R::TARGET_RESET)
    }

    /// Sets if the counter triggers an IRQ after hitting the target.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn target_irq(self, enabled: bool) -> Self {
        self.clear(1 << R::TARGET_IRQ)
            .set((enabled as u16) << R::TARGET_IRQ)
    }

    /// Sets if the counter triggers an IRQ after overflowing.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn overflow_irq(self, enabled: bool) -> Self {
        self.clear(1 << R::OVERFLOW_IRQ)
            .set((enabled as u16) << R::OVERFLOW_IRQ)
    }

    /// Sets one-shot mode.
    #[cfg_attr(feature = "inline_hints", inline(always))]
    pub fn oneshot_mode(self, enabled: bool) -> Self {
        self.clear(1 << R::REPEAT_MODE)
            .set((!enabled as u16) << R::REPEAT_MODE)
    }
}
