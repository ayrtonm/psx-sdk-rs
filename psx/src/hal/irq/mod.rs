use crate::interrupt::IRQ;

mod mask;
mod stat;

const ALL_IRQS: u16 = (1 << (1 + IRQ::ControllerMisc as u16)) - 1;
