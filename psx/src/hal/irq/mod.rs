use core::mem::variant_count;

mod mask;
mod stat;
pub(crate) mod ty;

const ALL_IRQS: u16 = (1 << (variant_count::<ty::IRQ>())) - 1;
