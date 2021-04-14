macro_rules! read_only_mmio {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        read_only!($(#[$($meta)*])* $name<$size>);
        impl Address for $name {
            const ADDRESS: u32 = $address;
        }

        impl Read<$size> for $name {
            fn read(&self) -> $size {
                unsafe { read_volatile(Self::ADDRESS as *const $size) }
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal, $($others:tt)*) => {
        read_only_mmio!($(#[$($meta)*])* $name<$size>: $address);
        read_only_mmio!($($others)*);
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

macro_rules! read_write_mmio {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty> : $address:literal) => {
        read_write!($(#[$($meta)*])* $name<$size>);

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
        read_write_mmio!($(#[$($meta)*])* $name<$size>: $address);
        read_write_mmio!($($others)*);
    };
}
