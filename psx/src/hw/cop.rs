macro_rules! define_cop {
    ($cop:expr, $reg:expr) => {
        impl Register<u32> for CopRegister<$cop, $reg> {
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
}
