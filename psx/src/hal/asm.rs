macro_rules! read_only_cop {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty>; COP: $cop:expr; R: $reg:expr) => {
        read_only!($(#[$($meta)*])* $name<$size>);
        impl Read<$size> for $name {
            fn read(&self) -> $size {
                let value;
                unsafe {
                    asm!(concat!("mfc", $cop, " $2, $", $reg), out("$2") value);
                }
                value
            }
        }
    };
    ($(#[$($meta:meta)*])* $name:ident <$size:ty>; COP: $cop:expr; R: $reg:expr, $($others:tt)*) => {
        read_only_cop!($(#[$($meta)*])* $name<$size>; COP: $cop; R: $reg);
        read_only_cop!($($others)*);
    };
}

macro_rules! read_write_cop {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty>; COP: $cop:expr; R: $reg:expr) => {
        read_write!($(#[$($meta)*])* $name<$size>);

        impl<S: State> Read<$size> for $name<S> {
            fn read(&self) -> $size {
                let value;
                unsafe {
                    asm!(concat!("mfc", $cop, " $2, $", $reg), out("$2") value);
                }
                value
            }
        }

        impl Write<$size> for $name<Mutable> {
            fn write(&mut self, value: $size) {
                unsafe {
                    asm!(concat!("mtc", $cop, " $2, $", $reg), in("$2") value);
                }
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty>; COP: $cop:expr; R: $reg:expr, $($others:tt)*) => {
        read_write_cop!($(#[$($meta)*])* $name<$size>; COP: $cop; R: $reg);
        read_write_cop!($($others)*);
    };
}
