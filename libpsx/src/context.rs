use crate::gpu::{DisplayEnv, DrawEnv, GpuRead, GpuStat};

pub struct IOCtxt {
    draw_env: Option<DrawEnv>,
    display_env: Option<DisplayEnv>,
    gpu_read: Option<GpuRead>,
    gpu_stat: Option<GpuStat>,
}

impl IOCtxt {
    pub fn take_draw_env(&mut self) -> Option<DrawEnv> {
        self.draw_env.take()
    }

    pub fn take_display_env(&mut self) -> Option<DisplayEnv> {
        self.display_env.take()
    }

    pub fn replace_draw_env(&mut self, draw_env: Option<DrawEnv>) {
        self.draw_env = draw_env;
    }

    pub fn replace_display_env(&mut self, display_env: Option<DisplayEnv>) {
        self.display_env = display_env;
    }
}

impl Drop for DrawEnv {
    fn drop(&mut self) {
        unsafe {
            IOCX.replace_draw_env(Some(DrawEnv));
        }
    }
}

impl Drop for DisplayEnv {
    fn drop(&mut self) {
        unsafe {
            IOCX.replace_display_env(Some(DisplayEnv));
        }
    }
}

pub static mut IOCX: IOCtxt = IOCtxt {
    draw_env: Some(DrawEnv),
    display_env: Some(DisplayEnv),
    gpu_read: Some(GpuRead),
    gpu_stat: Some(GpuStat),
};

pub mod gpu {
    use crate::{ro_register, wo_register};
    ro_register!(GpuRead, 0x1F80_1810);
    ro_register!(GpuStat, 0x1F80_1814);
    wo_register!(DrawEnv, 0x1F80_1810);
    wo_register!(DisplayEnv, 0x1F80_1814);
}
