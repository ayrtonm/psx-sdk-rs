macro_rules! impl_value {
    ($reg:path) => {
        pub struct Value {
            bits: u32,
        }

        impl $reg {
            #[inline(always)]
            pub fn get(&self) -> Value {
                Value {
                    bits: unsafe { self.read() },
                }
            }
        }
    };
}

macro_rules! impl_mut_value {
    ($reg:path) => {
        impl_value!($reg);
        #[must_use = "MMIO register value must be written to memory"]
        pub struct MutValue<'a> {
            value: Value,
            register: &'a mut $reg,
        }

        impl $reg {
            #[inline(always)]
            pub fn get_mut(&mut self) -> MutValue {
                MutValue {
                    value: self.get(),
                    register: self,
                }
            }
        }

        impl core::ops::Deref for MutValue<'_> {
            type Target = Value;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl<'a> MutValue<'a> {
            #[inline(always)]
            pub fn set(self) -> Value {
                unsafe { self.register.write(self.value.bits) };
                self.value
            }
        }
    };
}
