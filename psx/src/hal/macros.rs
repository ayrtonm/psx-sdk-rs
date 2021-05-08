macro_rules! read_only {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty>) => {
        $(#[$($meta)*])*
        #[derive(PartialEq, Eq)]
        pub struct $name($size);

        impl $name {
            /// Creates a read-only handle with its stored value initialized to `bits`.
            pub unsafe fn from_bits(bits: $size) -> Self {
                $name(bits)
            }
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
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty>, $($others:tt)*) => {
        read_only!($(#[$($meta)*])* $name<$size>);
        read_only!($($others)*);
    };
}

macro_rules! read_write {
    ($(#[$($meta:meta)*])* $name:ident <$size:ty>) => {
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
            unsafe fn skip_load() -> Self {
                $name(0, PhantomData)
            }
        }
    };

    ($(#[$($meta:meta)*])* $name:ident <$size:ty>, $($others:tt)*) => {
        read_write!($(#[$($meta)*])* $name<$size>);
        read_write!($($others)*);
    };
}
