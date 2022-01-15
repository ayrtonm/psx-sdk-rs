#![allow(unused_macros)]

macro_rules! impl_primitive {
    ($name:ident, $cmd:expr) => {
        impl $name {
            pub const fn new() -> Self {
                let buf = [0u8; size_of::<Self>()];
                let mut primitive = unsafe { transmute::<_, Self>(buf) };
                primitive.cmd = $cmd;
                primitive
            }
        }
        impl GP0Command for $name {}
    };
    ($name:ident < N > , $cmd:expr) => {
        impl<const N: usize> $name<N> {
            pub const fn new() -> Self {
                let buf = [0u8; size_of::<Self>()];
                let mut primitive = unsafe { transmute::<_, Self>(buf) };
                primitive.cmd = $cmd;
                primitive
            }
        }
        impl<const N: usize> GP0Command for $name<N> {}
    };
}

macro_rules! vertices_fn {
    (3) => {
        /// Gets the primitive's vertices.
        pub fn get_vertices(&self) -> [Vertex; 3] {
            [self.v0, self.v1, self.v2]
        }

        /// Sets the primitive's vertices.
        pub fn set_vertices(&mut self, vertices: [Vertex; 3]) -> &mut Self {
            self.v0 = vertices[0];
            self.v1 = vertices[1];
            self.v2 = vertices[2];
            self
        }
    };
    (4) => {
        /// Gets the primitive's vertices.
        pub fn get_vertices(&self) -> [Vertex; 4] {
            [self.v0, self.v1, self.v2, self.v3]
        }

        /// Sets the primitive's vertices.
        pub fn set_vertices(&mut self, vertices: [Vertex; 4]) -> &mut Self {
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
        pub fn get_color(&self) -> Color {
            self.color
        }

        /// Sets the primitive's color.
        pub fn set_color(&mut self, color: Color) -> &mut Self {
            self.color = color.into();
            self
        }
    };
}

macro_rules! gouraud_fn {
    (3) => {
        /// Gets the primitive's color.
        pub fn get_colors(&self) -> [Color; 3] {
            [self.color0, self.color1, self.color2]
        }

        /// Returns mutable references to the primitive's colors.
        pub fn get_colors_mut(&mut self) -> [&mut Color; 3] {
            [&mut self.color0, &mut self.color1, &mut self.color2]
        }

        /// Sets the primitive's color.
        pub fn set_colors(&mut self, colors: [Color; 3]) -> &mut Self {
            let colors = colors.map(|t| Color::from(t));
            self.color0 = colors[0];
            self.color1 = colors[1];
            self.color2 = colors[2];
            self
        }
    };
    (4) => {
        /// Gets the primitive's color.
        pub fn get_colors(&self) -> [Color; 4] {
            [self.color0, self.color1, self.color2, self.color3]
        }

        /// Returns mutable references to the primitive's colors.
        pub fn get_colors_mut(&mut self) -> [&mut Color; 4] {
            [
                &mut self.color0,
                &mut self.color1,
                &mut self.color2,
                &mut self.color3,
            ]
        }

        /// Sets the primitive's color.
        pub fn set_colors(&mut self, colors: [Color; 4]) -> &mut Self {
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
        pub fn get_offset(&self) -> Vertex {
            self.offset
        }

        /// Sets the primitive's offset.
        pub fn set_offset(&mut self, offset: Vertex) -> &mut Self {
            self.offset = offset;
            self
        }
    };
}

macro_rules! size_fn {
    () => {
        /// Gets the primitive's offset.
        pub fn get_size(&self) -> Vertex {
            self.size
        }

        /// Sets the primitive's offset.
        pub fn set_size(&mut self, size: Vertex) -> &mut Self {
            self.size = size;
            self
        }
    };
}

macro_rules! clut_fn {
    () => {
        /// Gets the color lookup table.
        pub fn get_clut(&self) -> Clut {
            self.clut
        }

        /// Sets the color lookup table.
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
        pub fn get_tex_coord(&self) -> TexCoord {
            self.t0
        }

        /// Sets the primitive's texcoord.
        pub fn set_tex_coord<T>(&mut self, t0: T) -> &mut Self
        where TexCoord: From<T> {
            self.t0 = t0.into();
            self
        }
    };
    (3) => {
        /// Gets the primitive's texcoords.
        pub fn get_tex_coords(&self) -> [TexCoord; 3] {
            [self.t0, self.t1, self.t2]
        }

        /// Returns mutable references to the primitive's texcoord.
        pub fn get_tex_coords_mut(&mut self) -> [&mut TexCoord; 3] {
            [&mut self.t0, &mut self.t1, &mut self.t2]
        }

        /// Sets the primitive's texcoords.
        pub fn set_tex_coords<T>(&mut self, tex_coords: [T; 3]) -> &mut Self
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
        pub fn get_tex_coord(&self) -> [TexCoord; 4] {
            [self.t0, self.t1, self.t2, self.t3]
        }

        /// Returns mutable references to the primitive's texcoord.
        pub fn get_tex_coords_mut(&mut self) -> [&mut TexCoord; 4] {
            [&mut self.t0, &mut self.t1, &mut self.t2, &mut self.t3]
        }

        /// Sets the primitive's texcoords.
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
        pub fn get_tex_page(&self) -> TexPage {
            self.tpage
        }

        /// Sets the primitive's texture page.
        pub fn set_tex_page<T>(&mut self, tpage: T) -> &mut Self
        where TexPage: From<T> {
            self.tpage = tpage.into();
            self
        }
    };
}
