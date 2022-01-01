macro_rules! define_cop {
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(,)?) => {
        $(#[$($meta)*])*
        pub type $name = CopRegister<$cop, $reg>;

        impl Register<$ty> for CopRegister<$cop, $reg> {
            fn skip_load() -> Self {
                Self { value: 0 }
            }
            fn load(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        concat!(".set noat
            mfc", $cop, " {0}, $", $reg), out(reg) self.value
                    }
                }
                self
            }

            fn store(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        concat!(".set noat
            mtc", $cop, " {0}, $", $reg), in(reg) self.value
                    }
                }
                self
            }
        }
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr, $($others:tt)*) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg);
        define_cop!($($others)*);
    };
}
