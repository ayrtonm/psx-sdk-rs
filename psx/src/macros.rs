#![macro_use]

macro_rules! define_register {
    ($(#[$meta:meta])* $name:ident, $address:expr, $size:ty) => {
        $(#[$meta])*
        pub struct $name;

        impl $crate::mmio::Address for $name {
            const ADDRESS: $size = $address;
        }
    };
}

macro_rules! declare_rw {
    ($(#[$meta:meta])* $name:ident, $address:expr, $size:ty) => {
        define_register!($(#[$meta])* $name, $address, $size);
        impl $crate::mmio::Load<$size> for $name {}
        impl $crate::value::Load<$size> for $name {
            #[inline(always)]
            unsafe fn load(&self) -> $size {
                <Self as $crate::mmio::Load<$size>>::load(self)
            }
        }
        impl $crate::mmio::Store<$size> for $name {}
        impl $crate::value::Store<$size> for $name {
            #[inline(always)]
            unsafe fn store(&mut self, value: $size) {
                <Self as $crate::mmio::Store<$size>>::store(self, value)
            }
        }

        impl $name {
            /// Loads the register from memory, immutably borrowing the register.
            #[inline(always)]
            pub fn load(&self) -> $crate::value::Value<Self, $size> {
                $crate::value::Value::load(self)
            }

            /// Loads the register from memory, exclusively borrowing the register.
            #[inline(always)]
            pub fn load_mut(&mut self) -> $crate::value::MutValue<Self, $size> {
                $crate::value::MutValue::load_mut(self)
            }
        }
    };
}
