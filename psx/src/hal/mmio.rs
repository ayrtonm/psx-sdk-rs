macro_rules! read_only {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        $(#[$($meta)*])*
        #[derive(PartialEq, Eq)]
        pub struct $name($size);

        impl $name {
            /// Creates a read-only handle with its stored value initialized to `bits`.
            pub unsafe fn from_bits(bits: $size) -> Self {
                $name(bits)
            }
        }

        impl Address for $name {
            const ADDRESS: u32 = $address;
        }

        impl private::HasValue<$size> for $name {
            fn get(&self) -> $size {
                self.0
            }

            fn get_mut(&mut self) -> &mut $size {
                &mut self.0
            }
        }

        impl Register<$size> for $name {
            fn load() -> Self {
                let unread = $name(0);
                $name(unread.read())
            }
        }

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

        impl<S: State> private::HasValue<$size> for $name<S> {
            fn get(&self) -> $size {
                self.0
            }

            fn get_mut(&mut self) -> &mut $size {
                &mut self.0
            }
        }

        impl<S: State> Register<$size> for $name<S> {
            fn load() -> Self {
                let unread = Self(0, PhantomData);
                $name(unread.read(), PhantomData)
            }
        }

        impl MutRegister<$size> for $name<Mutable> {
            fn skip_load() -> Self {
                $name(0, PhantomData)
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
