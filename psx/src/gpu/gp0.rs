use super::vertex::Vertex;
use crate::mmio::gpu;
use crate::mmio::register::Write;

impl gpu::GP0 {
    pub fn start<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(offset);
        unsafe { self.write(0xE3 << 24 | v.x() as u32 | ((v.y() as u32) << 10)) };
        self
    }

    pub fn end<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(offset);
        unsafe { self.write(0xE4 << 24 | v.x() as u32 | ((v.y() as u32) << 10)) };
        self
    }

    pub fn offset<T>(&mut self, offset: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(offset);
        unsafe { self.write(0xE5 << 24 | v.x() as u32 | ((v.y() as u32) << 11)) };
        self
    }
}
