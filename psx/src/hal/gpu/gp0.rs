use super::ty::{Color, Vertex};
use super::{command, Primitive};
use crate::hal::{Write, GP0};

impl GP0 {
    pub fn clear_cache(&mut self) {
        self.write(command(0x01, None));
    }

    pub fn fill_rectangle(&mut self, color: Color, offset: Vertex, size: Vertex) {
        self.write(command(0x02, Some(color.into())));
        self.write(offset.into());
        self.write(size.into());
    }

    pub fn copy_rectangle(&mut self, src: Vertex, dest: Vertex, size: Vertex) {
        self.write(command(0x80, None));
        self.write(src.into());
        self.write(dest.into());
        self.write(size.into());
    }

    pub fn interrupt_request(&mut self) {
        self.write(command(0x1F, None));
    }

    pub fn draw<P: Primitive>(&mut self, prim: &P) -> &mut Self {
        self.write_slice(prim.primitive())
    }
}
