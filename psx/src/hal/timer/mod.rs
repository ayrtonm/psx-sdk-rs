use crate::hal::{MutRegister, Mutable, Register, State};
use crate::hal::{T0_CNT, T0_MODE, T0_TGT};
use crate::hal::{T1_CNT, T1_MODE, T1_TGT};
use crate::hal::{T2_CNT, T2_MODE, T2_TGT};
use crate::illegal;

#[macro_use]
mod timers;

pub enum Source {
    System = 0,
    Alternate,
}

#[allow(non_upper_case_globals)]
pub const DotClock: Source = Source::Alternate;
#[allow(non_upper_case_globals)]
pub const Hblank: Source = Source::Alternate;
#[allow(non_upper_case_globals)]
pub const FracSys: Source = Source::Alternate;

pub enum SyncMode {
    Pause = 0,
    Reset,
    Count,
    FreeRun,
}

const SYNC_MODE: u16 = 1;
const TARGET_RESET: u16 = 3;
const TARGET_IRQ: u16 = 4;
const OVERFLOW_IRQ: u16 = 5;
const PULSE_MODE: u16 = 6;
const SOURCE: u16 = 8;
const HIT_TARGET: u16 = 11;
const HIT_OVERFLOW: u16 = 12;

pub trait Counter: Register<u16> {
    /// Waits until the counter stops changing and returns the final value.
    fn wait(&mut self) -> u16 {
        while self.get() != self.reload() {}
        self.get()
    }
}

pub trait MutCounter: MutRegister<u16> {}

pub trait Mode: Register<u16> {
    fn sync_enabled(&self) -> bool {
        self.contains(1)
    }

    fn get_sync_mode(&self) -> SyncMode {
        match (self.get() >> SYNC_MODE) & 0b11 {
            0 => SyncMode::Pause,
            1 => SyncMode::Reset,
            2 => SyncMode::Count,
            3 => SyncMode::FreeRun,
            _ => illegal(),
        }
    }

    fn get_source(&self) -> Source {
        if self.contains(1 << SOURCE) {
            Source::Alternate
        } else {
            Source::System
        }
    }

    /// Checks if hitting the target resets the counter.
    fn target_resets(&self) -> bool {
        self.contains(1 << TARGET_RESET)
    }

    /// Checks if hitting the target triggers an IRQ.
    fn target_irqs(&self) -> bool {
        self.contains(1 << TARGET_IRQ)
    }

    /// Checks if overflow triggers an IRQ.
    fn overflow_irqs(&self) -> bool {
        self.contains(1 << OVERFLOW_IRQ)
    }

    /// Checks if the counter reached the target.
    fn reached_target(&self) -> bool {
        self.contains(1 << HIT_TARGET)
    }

    /// Checks if the counter overflowed.
    fn overflowed(&self) -> bool {
        self.contains(1 << HIT_OVERFLOW)
    }

    /// Checks if the IRQ is pulsed.
    fn pulsed_irq(&self) -> bool {
        self.contains(1 << PULSE_MODE)
    }
}

pub trait MutMode: MutRegister<u16> {
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

pub trait Target: Register<u16> {}
pub trait MutTarget: MutRegister<u16> {}

timer! {
    [T0_CNT, T0_MODE, T0_TGT],
    [T1_CNT, T1_MODE, T1_TGT],
    [T2_CNT, T2_MODE, T2_TGT]
}
