macro_rules! impl_primitive {
    ($name:ident, $cmd:expr) => {
        impl InitPrimitive for $name {
            #[inline(always)]
            fn init_primitive(&mut self) {
                self.cmd = $cmd;
            }
        }

        impl<const N: usize> Buffer<N> {
            /// Allocates a single packet. Returns `None` if remaining buffer space is
            /// insufficient.
            pub fn $name(&self) -> Option<&mut Packet<$name>> {
                self.alloc()
            }
        }

        impl<const N: usize> DoubleBuffer<N> {
            /// Allocates a double-buffered packet. Returns `None` if remaining buffer
            /// space is insufficient.
            pub fn $name(&self) -> Option<DoublePacket<$name>> {
                self.alloc()
            }
        }
    };
}

macro_rules! vertices_fn {
    (3) => {
        /// Gets the primitive's vertices.
        #[inline(always)]
        pub fn get_vertices(&self) -> [Vertex; 3] {
            [self.v0, self.v1, self.v2]
        }

        /// Sets the primitive's vertices.
        #[inline(always)]
        pub fn set_vertices<T>(&mut self, vertices: [T; 3]) -> &mut Self
        where Vertex: From<T> {
            let vertices = vertices.map(|t| Vertex::from(t));
            self.v0 = vertices[0];
            self.v1 = vertices[1];
            self.v2 = vertices[2];
            self
        }
    };
    (4) => {
        /// Gets the primitive's vertices.
        #[inline(always)]
        pub fn get_vertices(&self) -> [Vertex; 4] {
            [self.v0, self.v1, self.v2, self.v3]
        }

        /// Sets the primitive's vertices.
        #[inline(always)]
        pub fn set_vertices<T>(&mut self, vertices: [T; 4]) -> &mut Self
        where Vertex: From<T> {
            let vertices = vertices.map(|t| Vertex::from(t));
            self.v0 = vertices[0];
            self.v1 = vertices[1];
            self.v2 = vertices[2];
            self.v3 = vertices[3];
            self
        }
    };
}

macro_rules! color_fn {
    () => {
        /// Gets the primitive's color.
        #[inline(always)]
        pub fn get_color(&self) -> Color {
            self.color
        }

        /// Sets the primitive's color.
        #[inline(always)]
        pub fn set_color<T>(&mut self, color: T) -> &mut Self
        where Color: From<T> {
            self.color = color.into();
            self
        }
    };
}

macro_rules! gouraud_fn {
    (3) => {
        /// Gets the primitive's color.
        #[inline(always)]
        pub fn get_colors(&self) -> [Color; 3] {
            [self.color0, self.color1, self.color2]
        }

        /// Sets the primitive's color.
        #[inline(always)]
        pub fn colors<T>(&mut self, colors: [T; 3]) -> &mut Self
        where Color: From<T> {
            let colors = colors.map(|t| Color::from(t));
            self.color0 = colors[0];
            self.color1 = colors[1];
            self.color2 = colors[2];
            self
        }
    };
    (4) => {
        /// Gets the primitive's color.
        #[inline(always)]
        pub fn get_colors(&self) -> [Color; 4] {
            [self.color0, self.color1, self.color2, self.color3]
        }

        /// Sets the primitive's color.
        #[inline(always)]
        pub fn set_colors<T>(&mut self, colors: [T; 4]) -> &mut Self
        where Color: From<T> {
            let colors = colors.map(|t| Color::from(t));
            self.color0 = colors[0];
            self.color1 = colors[1];
            self.color2 = colors[2];
            self.color3 = colors[3];
            self
        }
    };
}

macro_rules! offset_fn {
    () => {
        /// Gets the primitive's offset.
        #[inline(always)]
        pub fn get_offset(&self) -> Vertex {
            self.offset
        }

        /// Sets the primitive's offset.
        #[inline(always)]
        pub fn set_offset<T>(&mut self, offset: T) -> &mut Self
        where Vertex: From<T> {
            self.offset = offset.into();
            self
        }
    };
}

macro_rules! size_fn {
    () => {
        /// Gets the primitive's offset.
        #[inline(always)]
        pub fn get_size(&self) -> Vertex {
            self.size
        }

        /// Sets the primitive's offset.
        #[inline(always)]
        pub fn set_size<T>(&mut self, size: T) -> &mut Self
        where Vertex: From<T> {
            self.size = size.into();
            self
        }
    };
}

macro_rules! clut_fn {
    () => {
        /// Gets the color lookup table.
        #[inline(always)]
        pub fn get_clut(&self) -> Clut {
            self.clut
        }

        /// Sets the color lookup table.
        #[inline(always)]
        pub fn set_clut<T>(&mut self, clut: T) -> &mut Self
        where Clut: From<T> {
            self.clut = clut.into();
            self
        }
    };
}

macro_rules! tex_coord_fn {
    (1) => {
        /// Gets the primitive's texcoord.
        #[inline(always)]
        pub fn get_tex_coord(&self) -> TexCoord {
            self.t0
        }

        /// Sets the primitive's texcoord.
        #[inline(always)]
        pub fn set_tex_coord<T>(&mut self, t0: T) -> &mut Self
        where TexCoord: From<T> {
            self.t0 = t0.into();
            self
        }
    };
    (3) => {
        /// Gets the primitive's texcoords.
        #[inline(always)]
        pub fn get_tex_coord(&self) -> [TexCoord; 3] {
            [self.t0, self.t1, self.t2]
        }

        /// Sets the primitive's texcoords.
        #[inline(always)]
        pub fn set_tex_coord<T>(&mut self, tex_coords: [T; 3]) -> &mut Self
        where TexCoord: From<T> {
            let tex_coords = tex_coords.map(|t| TexCoord::from(t));
            self.t0 = tex_coords[0];
            self.t1 = tex_coords[1];
            self.t2 = tex_coords[2];
            self
        }
    };
    (4) => {
        /// Gets the primitive's texcoords.
        #[inline(always)]
        pub fn get_tex_coord(&self) -> [TexCoord; 4] {
            [self.t0, self.t1, self.t2, self.t3]
        }

        /// Sets the primitive's texcoords.
        #[inline(always)]
        pub fn set_tex_coord<T>(&mut self, tex_coords: [T; 4]) -> &mut Self
        where TexCoord: From<T> {
            let tex_coords = tex_coords.map(|t| TexCoord::from(t));
            self.t0 = tex_coords[0];
            self.t1 = tex_coords[1];
            self.t2 = tex_coords[2];
            self.t3 = tex_coords[3];
            self
        }
    };
}

macro_rules! tex_page_fn {
    () => {
        /// Gets the primitive's texture page.
        #[inline(always)]
        pub fn get_tex_page(&self) -> TexPage {
            self.tpage
        }

        /// Sets the primitive's texture page.
        #[inline(always)]
        pub fn set_tex_page<T>(&mut self, tpage: T) -> &mut Self
        where TexPage: From<T> {
            self.tpage = tpage.into();
            self
        }
    };
}
