//! Coprocessor register definitions
use crate::hw::private::Primitive;

/// A handle to a coprocessor register
pub struct CopRegister<T: Primitive, const COP: u32, const REG: u32> {
    pub(super) value: T,
}

impl<T: Primitive, const COP: u32, const REG: u32> AsRef<T> for CopRegister<T, COP, REG> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Primitive, const COP: u32, const REG: u32> AsMut<T> for CopRegister<T, COP, REG> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

macro_rules! define_cop {
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(,)?) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg; "m");
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr; $cop_ty:literal $(,)?) => {
        $(#[$($meta)*])*
        pub type $name = crate::hw::cop::CopRegister<$ty, $cop, $reg>;

        impl Register<$ty> for crate::hw::cop::CopRegister<$ty, $cop, $reg> {
            fn skip_load() -> Self {
                Self { value: 0 }
            }
            fn load(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        concat!($cop_ty, "fc", $cop, " $2, $", $reg), out("$2") self.value
                    }
                }
                self
            }

            fn store(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        concat!($cop_ty, "tc", $cop, " $2, $", $reg), in("$2") self.value
                    }
                }
                self
            }
        }
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(;$cop_ty:literal)?, $($others:tt)*) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg $(;$cop_ty)*);
        define_cop!($($others)*);
    };
}
