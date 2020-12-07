#![macro_use]

macro_rules! impl_prim {
    ($name:ident, $cmd:expr) => {
        impl Init for $name {
            fn init(&mut self) {
                self.cmd();
            }
        }

        #[allow(non_snake_case)]
        impl<const N: usize> Buffer<N> {
            pub fn $name(&self) -> Option<&mut Packet<$name>> {
                self.alloc::<$name>()
            }
        }

        #[allow(non_snake_case)]
        impl<const N: usize> DoubleBuffer<N> {
            pub fn $name(&self) -> Option<DoublePacket<$name>> {
                self.alloc::<$name>()
            }
        }

        impl $name {
            pub(self) fn cmd(&mut self) -> &mut Self {
                self.cmd = $cmd;
                self
            }
        }
    };
}

macro_rules! impl_vertices {
    (1, $name:path) => {
        impl $name {
            pub fn offset<T>(&mut self, offset: T) -> &mut Self
            where Vertex: From<T> {
                self.offset = Vertex::from(offset);
                self
            }
        }
    };
    (2, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 2]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self
            }
        }
    };
    (3, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 3]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self.v2 = vertices[2];
                self
            }
        }
    };
    (4, $name:path) => {
        impl $name {
            pub fn vertices<T>(&mut self, vertices: [T; 4]) -> &mut Self
            where Vertex: From<T> {
                let vertices = vertices.map(|t| Vertex::from(t));
                self.v0 = vertices[0];
                self.v1 = vertices[1];
                self.v2 = vertices[2];
                self.v3 = vertices[3];
                self
            }
        }
    };
}

macro_rules! impl_color {
    ($name:path) => {
        impl $name {
            pub fn color(&mut self, color: Color) -> &mut Self {
                self.color = color;
                self
            }
        }
    };
}

macro_rules! impl_gouraud {
    (2, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 2]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self
            }
        }
    };
    (3, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 3]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self.color2 = palette[2];
                self
            }
        }
    };
    (4, $name:path) => {
        impl $name {
            pub fn color(&mut self, palette: [Color; 4]) -> &mut Self {
                self.color0 = palette[0];
                self.color1 = palette[1];
                self.color2 = palette[2];
                self.color3 = palette[3];
                self
            }
        }
    };
}
