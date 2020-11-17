use crate::dma;
use crate::gpu::{DispPort, DrawPort, GpuRead, GpuStat};
use crate::interrupt;

pub struct IO {
    //GPU ports
    draw_port: Option<DrawPort>,
    disp_port: Option<DispPort>,
    gpu_read: Option<GpuRead>,
    gpu_stat: Option<GpuStat>,

    //DMA channel registers
    gpu_dma: Option<dma::Gpu>,

    //Interrupt registers
    int_mask: Option<interrupt::Mask>,
}

impl IO {
    /// This is unsafe because IO registers inherently introduce mutable, global
    /// state. Ideally this should only be called in the single invocation of
    /// the `psx::exe` macro, but who am I to stop you from playing with
    /// fire?
    pub unsafe fn new() -> Self {
        IO {
            draw_port: Some(DrawPort::new()),
            disp_port: Some(DispPort::new()),
            gpu_read: Some(GpuRead::new()),
            gpu_stat: Some(GpuStat::new()),

            gpu_dma: Some(dma::Channel {
                addr: dma::gpu::Addr::new(),
                block: dma::gpu::Block::new(),
                control: dma::gpu::Control::new(),
            }),

            int_mask: Some(interrupt::Mask::new()),
        }
    }

    pub fn take_draw_port(&mut self) -> Option<DrawPort> {
        self.draw_port.take()
    }

    pub fn take_disp_port(&mut self) -> Option<DispPort> {
        self.disp_port.take()
    }

    pub fn take_gpu_read(&mut self) -> Option<GpuRead> {
        self.gpu_read.take()
    }

    pub fn take_gpu_stat(&mut self) -> Option<GpuStat> {
        self.gpu_stat.take()
    }

    pub fn take_gpu_dma(&mut self) -> Option<dma::Gpu> {
        self.gpu_dma.take()
    }

    pub fn take_int_mask(&mut self) -> Option<interrupt::Mask> {
        self.int_mask.take()
    }

    pub fn replace_draw_port(&mut self, draw_port: Option<DrawPort>) {
        self.draw_port = draw_port;
    }

    pub fn replace_disp_port(&mut self, disp_port: Option<DispPort>) {
        self.disp_port = disp_port;
    }

    pub fn replace_gpu_read(&mut self, gpu_read: Option<GpuRead>) {
        self.gpu_read = gpu_read;
    }

    pub fn replace_gpu_stat(&mut self, gpu_stat: Option<GpuStat>) {
        self.gpu_stat = gpu_stat;
    }

    pub fn replace_gpu_dma(&mut self, gpu_dma: Option<dma::Gpu>) {
        self.gpu_dma = gpu_dma;
    }

    pub fn replace_int_mask(&mut self, int_mask: Option<interrupt::Mask>) {
        self.int_mask = int_mask;
    }
}
