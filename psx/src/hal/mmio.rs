macro_rules! read_only {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        $(#[$($meta)*])*
        #[derive(PartialEq, Eq)]
        pub struct $name($size);

        impl $name {
            pub fn load() -> Self {
                let unread = $name(0);
                $name(unread.read())
            }

            pub unsafe fn from_bits(bits: u32) -> Self {
                $name(bits)
            }
        }

        impl Address for $name {
            const ADDRESS: u32 = $address;
        }

        impl HasValue<$size> for $name {
            fn get(&self) -> $size {
                self.0
            }

            fn get_mut(&mut self) -> &mut $size {
                &mut self.0
            }
        }

        impl Register<$size> for $name {}

        impl Read<$size> for $name {
            fn read(&self) -> $size {
                unsafe { read_volatile(Self::ADDRESS as *const $size) }
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal, $($others:tt)*) => {
        read_only!($(#[$($meta)*])* $name<$size>: $address);
        read_only!($($others)*);
    };
}

macro_rules! write_only {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        $(#[$($meta)*])* pub struct $name;
        impl $name {
            pub fn write_slice(&mut self, values: &[$size]) -> &mut Self {
                for &v in values {
                    self.write(v);
                }
                self
            }
        }

        impl Address for $name {
            const ADDRESS: u32 = $address;
        }

        impl Write<$size> for $name {
            fn write(&mut self, value: $size) {
                unsafe {
                    write_volatile(Self::ADDRESS as *mut $size, value)
                }
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal, $($others:tt)*) => {
        write_only!($(#[$($meta)*])* $name<$size>: $address);
        write_only!($($others)*);
    };
}

macro_rules! read_write {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        $(#[$($meta)*])*
        #[derive(PartialEq, Eq)]
        pub struct $name<S: State>($size, PhantomData::<S>);

        impl $name<Shared> {
            /// Reads the register and creates a read-only handle with a copy of the register's
            /// current value.
            pub fn load() -> Self {
                let unread = Self(0, PhantomData);
                $name(unread.read(), PhantomData)
            }
        }

        impl<S: State> HasValue<$size> for $name<S> {
            fn get(&self) -> $size {
                self.0
            }

            fn get_mut(&mut self) -> &mut $size {
                &mut self.0
            }
        }

        impl<S: State> Register<$size> for $name<S> {}

        impl MutRegister<$size> for $name<Mutable> {
            /// Creates a read-write handle without reading the register's current value.
            fn skip_load() -> Self {
                $name(0, PhantomData)
            }

            /// Reads the register and creates a read-write handle with a copy of the register's
            /// current value.
            fn load_mut() -> Self {
                let unread = $name::<Shared>(0, PhantomData);
                $name(unread.read(), PhantomData)
            }
        }

        impl<S: State> Address for $name<S> {
            const ADDRESS: u32 = $address;
        }

        impl<S: State> Read<$size> for $name<S> {
            fn read(&self) -> $size {
                unsafe { read_volatile(Self::ADDRESS as *const $size) }
            }
        }

        impl Write<$size> for $name<Mutable> {
            /// Writes to the register.
            fn write(&mut self, value: $size) {
                unsafe {
                    write_volatile($address as *mut $size, value)
                }
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal, $($others:tt)*) => {
        read_write!($(#[$($meta)*])* $name<$size>: $address);
        read_write!($($others)*);
    };
}
