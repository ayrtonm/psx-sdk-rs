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
    (m, from, $cop:expr, $reg:expr) => {
        concat!(
            "mfc", $cop, " {}, $", $reg, "\n",
            "nop\n",
            "nop", // 2 delay slots required to ensure value is in out(reg)
        )
    };
    (m, to, $cop:expr, $reg:expr) => {
        concat!("mtc", $cop, " {}, $", $reg, "\n",
            "nop\n",
            "nop"
        )
    };
    (c, from, $cop:expr, $reg:expr) => {
        // cfc2 not currently supported by LLVM
        // MIPS CFCn (Move Control From Coprocessor) instruction layout:
        // 31    26 25   21 20   16 15   11 10           0
        // +-------+-------+-------+-------+--------------+
        // | COPn  |  CF   |  rt   |  rd   |  Padding     |
        // | 0100nn| 00010 | 00001 | reg-32| 000 0000 0000|
        // +-------+-------+-------+-------+--------------+
        //     6       5       5       5        11
        // - COPn: Opcode prefix (0x10 | $cop) -> 1 << 30 | ($cop << 26)
        // - CF:   Coprocessor sub-operation 'Move Control From' -> 2 << 21
        // - rt:   General purpose register (destination). Hardcoded to 1 ($at) -> 1 << 16
        // - rd:   Coprocessor control register (source) -> ($reg - 32) << 11
        concat!(
            ".long (1 << 30) | (", $cop, " << 26) | (2 << 21) | (1 << 16) | ((", $reg, " - 32) << 11) # cfc", $cop, "\n",
            "nop\n",
            "nop\n",
            "addiu {}, $at, 0"
        )
    };
    (c, to, $cop:expr, $reg:expr) => {
        // ctc2 not currently supported by LLVM
        // MIPS CTCn (Move Control to Coprocessor) instruction layout:
        // 31    26 25   21 20   16 15   11 10           0
        // +-------+-------+-------+-------+--------------+
        // | COPn  |  CT   |  rt   |  rd   |  Padding     |
        // | 0100nn| 00110 | 00001 | reg-32| 000 0000 0000|
        // +-------+-------+-------+-------+--------------+
        //     6       5       5       5        11
        // - COPn: Opcode prefix (0x10 | $cop) -> 1 << 30 | ($cop << 26)
        // - CT:   Coprocessor sub-operation 'Move Control To' -> 6 << 21
        // - rt:   General purpose register (source). Hardcoded to 1 ($at) -> 1 << 16
        // - rd:   Coprocessor control register (destination) -> ($reg - 32) << 11
        concat!(
            "addiu $at, {}, 0\n",
            ".long (1 << 30) | (", $cop, " << 26) | (6 << 21) | (1 << 16) | ((", $reg, " - 32) << 11) # ctc", $cop, "\n",
            "nop\n",
            "nop",
        )
    };
}

macro_rules! define_cop {
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(,)?) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg; m);
    };
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr; $cop_ty:ident $(,)?) => {
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
    ($(#[$($meta:meta)*])* $name:ident <$ty:ty>; COP: $cop:expr; R: $reg:expr $(;$cop_ty:ident)?, $($others:tt)*) => {
        define_cop!($(#[$($meta)*])* $name<$ty>; COP: $cop; R: $reg $(;$cop_ty)*);
        define_cop!($($others)*);
    };
}
