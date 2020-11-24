use crate::gpu::vertex::Vertex;
use crate::gpu::{AsU32, DrawPort, Packet};
use crate::registers::Write;

impl DrawPort {
    pub fn send<T: Packet<N>, const N: usize>(&mut self, cmd: &T) -> &mut Self {
        for &data in cmd.packet().iter() {
            self.write(data);
        }
        self
    }

    pub fn to_vram<T, U>(&mut self, offset: T, size: U, data: &[u32]) -> &mut Self
    where Vertex: From<T> + From<U> {
        self.write(0xA0 << 24);
        self.write(Vertex::from(offset).as_u32());
        self.write(Vertex::from(size).as_u32());
        for &d in data {
            self.write(d);
        }
        self
    }

    // GPU Rendering Attributes
    pub fn start<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE3 << 24 | v.x() as u32 | ((v.y() as u32) << 10));
        self
    }

    pub fn end<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE4 << 24 | v.x() as u32 | ((v.y() as u32) << 10));
        self
    }

    pub fn offset<T>(&mut self, v: T) -> &mut Self
    where Vertex: From<T> {
        let v = Vertex::from(v);
        self.write(0xE5 << 24 | v.x() as u32 | ((v.y() as u32) << 11));
        self
    }
}
