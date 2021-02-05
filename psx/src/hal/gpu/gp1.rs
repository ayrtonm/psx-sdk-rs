use super::command;
use crate::gpu::{DMAMode, Depth, PackedVertex, Vertex, VideoMode};
use crate::hal::{Write, GP1};

impl GP1 {
    pub fn reset_gpu(&mut self) -> &mut Self {
        self.write(command(0x00, None));
        self
    }

    pub fn reset_buffer(&mut self) -> &mut Self {
        self.write(command(0x01, None));
        self
    }

    pub fn ack_irq(&mut self) -> &mut Self {
        self.write(command(0x02, None));
        self
    }

    pub fn enable_display(&mut self, enabled: bool) -> &mut Self {
        self.write(command(0x03, Some(!enabled as u32)));
        self
    }

    pub fn dma_mode(&mut self, direction: Option<DMAMode>) -> &mut Self {
        self.write(command(0x04, direction.map(|d| d as u32).or(Some(0))));
        self
    }

    pub fn display_start(&mut self, start: PackedVertex<3, 10, 9>) -> &mut Self {
        self.write(command(0x05, Some(start.into())));
        self
    }

    pub fn horizontal_range(&mut self, range: PackedVertex<3, 12, 12>) -> &mut Self {
        self.write(command(0x06, Some(range.into())));
        self
    }

    pub fn vertical_range(&mut self, range: PackedVertex<3, 10, 10>) -> &mut Self {
        self.write(command(0x07, Some(range.into())));
        self
    }

    pub fn display_mode(
        &mut self, res: Vertex, mode: VideoMode, depth: Depth, interlace: bool,
    ) -> &mut Self {
        let hres = match res.x {
            256 => 0,
            320 => 1,
            512 => 2,
            640 => 3,
            368 => 1 << 6,
            _ => illegal!("Invalid horizontal resolution\0"),
        };
        let vres = match res.y {
            240 => 0,
            480 => 1,
            _ => illegal!("Invalid vertical resolution\0"),
        };
        let settings =
            hres | vres << 2 | (mode as u32) << 3 | (depth as u32) << 4 | (interlace as u32) << 5;
        self.write(command(0x08, Some(settings)));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::gpu::DMAMode;
    use crate::hal::{Register, GP0, GP1, GPUSTAT};

    #[test_case]
    fn reset_gpu() {
        GP1.reset_gpu();
        assert!(GPUSTAT::load().bits() == 0x1480_2000);
    }

    #[test_case]
    fn ack_irq() {
        GP1.reset_gpu();
        let mut stat = GPUSTAT::load();
        assert!(!stat.irq_pending());
        GP0.interrupt_request();
        stat.reload();
        assert!(stat.irq_pending());
        GP1.ack_irq();
        stat.reload();
        assert!(!stat.irq_pending());
    }

    #[test_case]
    fn display() {
        GP1.reset_gpu();
        let mut stat = GPUSTAT::load();
        assert!(!stat.display_enabled());
        GP1.enable_display(true);
        stat.reload();
        assert!(stat.display_enabled());
        GP1.enable_display(false);
        stat.reload();
        assert!(!stat.display_enabled());
    }

    #[test_case]
    fn dma_mode() {
        GP1.reset_gpu();
        GP1.dma_mode(None);
        let mut stat = GPUSTAT::load();
        assert!(!stat.dma_enabled());
        GP1.dma_mode(Some(DMAMode::GP0));
        stat.reload();
        assert!(stat.dma_enabled());
        GP1.dma_mode(Some(DMAMode::GPUREAD));
        stat.reload();
        assert!(stat.dma_enabled());
    }
}
