//! Coprocessor register definitions
use crate::hw::private::Primitive;

/// A coprocessor register
#[repr(C)]
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

macro_rules! cop_move {
    ("m", from, $cop:expr, $reg:expr) => {
        concat!(
            "mfc", $cop, " {}, $", $reg, "\n",
            "nop"  // required to ensure value is in out(reg)
        )
    };
    ("m", to, $cop:expr, $reg:expr) => {
        concat!("mtc", $cop, " {}, $", $reg)
    };
    ("c", from, $cop:expr, $reg:expr) => {
        //concat!("cfc", $cop, " {}, $", $reg) # Not currently supported by LLVM
        concat!(
            //       cop      n             CF      rt=$at   rd=$reg - 32
            ".long 1<<30 | ", $cop, "<<26 | 2<<21 | 1<<16 | (", $reg, "-32)<<11 # cfc", $cop, "\n",
            "nop\n",
            "addiu {}, $at, 0"
        )
    };
    ("c", to, $cop:expr, $reg:expr) => {
        //concat!("ctc", $cop, " {}, $", $reg) # Not currently supported by LLVM
        concat!(
            "addiu $at, {}, 0\n",
            //       cop      n             CT      rt=$at   rd=$reg - 32
            ".long 1<<30 | ", $cop, "<<26 | 6<<21 | 1<<16 | (", $reg, "-32)<<11 # ctc", $cop
        )
    };
}

macro_rules! define_cop {
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(,)?) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg; "m");
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr; $cop_ty:tt $(,)?) => {
        $(#[$($meta)*])*
        pub type $name = crate::hw::cop::CopRegister<$ty, $cop, $reg>;

        impl Register<$ty> for crate::hw::cop::CopRegister<$ty, $cop, $reg> {
            fn skip_load() -> Self {
                Self { value: 0 }
            }
            fn load(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        ".set noat",
                        cop_move!($cop_ty, from, $cop, $reg),
                        ".set at",
                        out(reg) self.value,
                        options(nomem, nostack)
                    }
                }
                self
            }

            fn store(&mut self) -> &mut Self {
                unsafe {
                    core::arch::asm! {
                        ".set noat",
                        cop_move!($cop_ty, to, $cop, $reg),
                        ".set at",
                        in(reg) self.value,
                        options(nomem, nostack)
                    }
                }
                self
            }
        }
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(;$cop_ty:tt)?, $($others:tt)*) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg $(;$cop_ty)*);
        define_cop!($($others)*);
    };
}
