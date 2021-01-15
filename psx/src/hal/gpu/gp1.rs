use super::{command, Depth, HorizontalRes, PackedVertex, VerticalRes, VideoMode, DMA};
use crate::hal::{Write, GP1};

impl GP1 {
    pub fn reset_gpu(&mut self) {
        self.write(command(0x00, None));
    }

    pub fn reset_buffer(&mut self) {
        self.write(command(0x01, None));
    }

    pub fn ack_irq(&mut self) {
        self.write(command(0x02, None));
    }

    pub fn display_enable(&mut self, enabled: bool) {
        self.write(command(0x03, Some(!enabled as u32)));
    }

    pub fn dma_direction(&mut self, direction: Option<DMA>) {
        self.write(command(0x04, direction.map(|d| d as u32).or(Some(0))));
    }

    pub fn display_start(&mut self, start: PackedVertex<3, 10, 9>) {
        self.write(command(0x05, Some(start.into())));
    }

    pub fn horizontal_range(&mut self, range: PackedVertex<3, 12, 12>) {
        self.write(command(0x06, Some(range.into())));
    }

    pub fn vertical_range(&mut self, range: PackedVertex<3, 10, 10>) {
        self.write(command(0x07, Some(range.into())));
    }

    pub fn display_mode(
        &mut self, (hres, vres): (HorizontalRes, VerticalRes), mode: VideoMode, depth: Depth,
        interlace: bool,
    ) {
        let settings = hres as u32 |
            (vres as u32) << 2 |
            (mode as u32) << 3 |
            (depth as u32) << 4 |
            (interlace as u32) << 5;
        self.write(command(0x08, Some(settings)));
    }
}
