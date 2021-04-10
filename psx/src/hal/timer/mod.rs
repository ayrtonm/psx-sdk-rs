use crate::hal::{MutRegister, Mutable, Register, State};
use crate::hal::{T0_CNT, T0_MODE, T0_TGT};
use crate::hal::{T1_CNT, T1_MODE, T1_TGT};
use crate::hal::{T2_CNT, T2_MODE, T2_TGT};
use core::fmt;
use core::fmt::{Debug, Formatter};
use ty::{Source, SyncMode};

#[macro_use]
mod timers;
pub(crate) mod ty;

const SYNC_MODE: u16 = 1;
const TARGET_RESET: u16 = 3;
const TARGET_IRQ: u16 = 4;
const OVERFLOW_IRQ: u16 = 5;
const PULSE_MODE: u16 = 6;
const SOURCE: u16 = 8;
const HIT_TARGET: u16 = 11;
const HIT_OVERFLOW: u16 = 12;

pub trait SharedCurrent: Register<u16> {
    /// Waits until the counter stops changing and returns the final value.
    fn wait(&mut self) -> u16 {
        while self.get() != self.reload().bits() {}
        self.get()
    }
}

pub trait Current: MutRegister<u16> {}

pub trait SharedMode: Register<u16> {
    fn sync_enabled(&self) -> bool {
        self.all_set(1)
    }

    fn get_sync_mode(&self) -> SyncMode {
        match (self.get() >> SYNC_MODE) & 0b11 {
            0 => SyncMode::Pause,
            1 => SyncMode::Reset,
            2 => SyncMode::Count,
            3 => SyncMode::FreeRun,
            _ => {
                // This is OK since `& 0b11` in the matched expr restricts its value to [0, 3]
                unsafe { core::hint::unreachable_unchecked() }
            },
        }
    }

    fn get_source(&self) -> Source {
        if self.all_set(1 << SOURCE) {
            Source::Alternate
        } else {
            Source::System
        }
    }

    /// Checks if hitting the target resets the counter.
    fn target_resets(&self) -> bool {
        self.all_set(1 << TARGET_RESET)
    }

    /// Checks if hitting the target triggers an IRQ.
    fn target_irqs(&self) -> bool {
        self.all_set(1 << TARGET_IRQ)
    }

    /// Checks if overflow triggers an IRQ.
    fn overflow_irqs(&self) -> bool {
        self.all_set(1 << OVERFLOW_IRQ)
    }

    /// Checks if the counter reached the target.
    fn reached_target(&self) -> bool {
        self.all_set(1 << HIT_TARGET)
    }

    /// Checks if the counter overflowed.
    fn overflowed(&self) -> bool {
        self.all_set(1 << HIT_OVERFLOW)
    }

    /// Checks if the IRQ is pulsed.
    fn pulsed_irq(&self) -> bool {
        self.all_set(1 << PULSE_MODE)
    }
}

pub trait Mode: MutRegister<u16> {
    fn sync_enable(&mut self, enabled: bool) -> &mut Self {
        self.clear(1).set(enabled as u16)
    }

    fn set_sync_mode(&mut self, mode: SyncMode) -> &mut Self {
        self.clear(0b11 << SYNC_MODE)
            .set((mode as u16) << SYNC_MODE)
    }

    fn set_source(&mut self, src: Source) -> &mut Self {
        self.clear(1 << SOURCE).set((src as u16) << SOURCE)
    }

    /// Sets whether the counter will reset when it hits the target.
    fn reset_target(&mut self, enable: bool) -> &mut Self {
        self.clear(1 << TARGET_RESET)
            .set((enable as u16) << TARGET_RESET)
    }

    /// Sets whether the counter will trigger an IRQ when it hits the target.
    fn irq_target(&mut self, enable: bool) -> &mut Self {
        self.clear(1 << TARGET_IRQ)
            .set((enable as u16) << TARGET_IRQ)
    }

    /// Sets whether the counter will trigger an IRQ when it overflows.
    fn irq_overflow(&mut self, enable: bool) -> &mut Self {
        self.clear(1 << OVERFLOW_IRQ)
            .set((enable as u16) << OVERFLOW_IRQ)
    }

    /// Sets whether the IRQ is pulsed.
    fn pulse_irq(&mut self, enable: bool) -> &mut Self {
        self.clear(1 << PULSE_MODE)
            .set((enable as u16) << PULSE_MODE)
    }
}

pub trait SharedTarget: Register<u16> {}
pub trait Target: MutRegister<u16> {}

timer! {
    [T0_CNT, T0_MODE, T0_TGT],
    [T1_CNT, T1_MODE, T1_TGT],
    [T2_CNT, T2_MODE, T2_TGT]
}
