pub mod gpu {
    use crate::{ro_register, wo_register};
    ro_register!(GpuRead, 0x1F80_1810);
    ro_register!(GpuStat, 0x1F80_1814);
    wo_register!(DrawEnv, 0x1F80_1810);
    wo_register!(DisplayEnv, 0x1F80_1814);
}

// This `Pickup` trait was an interesting, but bad idea. I'll leave here in case
// I think of a sensible way to implement it.
//pub trait Pickup {
//    fn pickup(&mut self);
//}
//
//
//impl<T: Pickup> Drop for T {
//    fn drop(&mut self) {
//        self.pickup();
//    }
//}
//
//impl Drop for DrawEnv {
//    fn drop(&mut self) {
//        unsafe {
//            ctxt.replace_draw_env(Some(DrawEnv));
//        }
//    }
//}
//
//impl Drop for DisplayEnv {
//    fn drop(&mut self) {
//        unsafe {
//            ctxt.replace_display_env(Some(DisplayEnv));
//        }
//    }
//}
