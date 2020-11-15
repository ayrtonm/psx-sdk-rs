pub mod gpu {
    use crate::{ro_register, wo_register};
    ro_register!(GpuRead, 0x1F80_1810);
    ro_register!(GpuStat, 0x1F80_1814);
    wo_register!(DrawEnv, 0x1F80_1810);
    wo_register!(DisplayEnv, 0x1F80_1814);
}
