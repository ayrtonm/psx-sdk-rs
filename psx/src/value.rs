use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// An interface for loading a generic register.
pub trait Load<T> {
    /// Loads a generic register.
    unsafe fn load(&self) -> T;
}

/// An interface for storing a generic register.
pub trait Store<T> {
    /// Stores a generic register.
    unsafe fn store(&mut self, value: T);
}

/// A read-ony copy of the value of an I/O register previously read from memory.
pub struct Value<R, T> {
    pub(crate) bits: T,
    _reg: PhantomData<R>,
}

/// A read-write copy of the value of an I/O register previously read from
/// memory. Mutably borrows the loaded register to ensure exclusive access.
#[must_use = "MutValue must be stored in memory"]
pub struct MutValue<'a, R, T> {
    value: Value<R, T>,
    reg: &'a mut R,
}

impl<'a, R: Load<T>, T: Copy> Value<R, T> {
    /// Calls [`Load::load`] once to load an I/O register from memory.
    #[inline(always)]
    pub fn load(r: &R) -> Self {
        Value {
            bits: unsafe { r.load() },
            _reg: PhantomData::<R>,
        }
    }
}

impl<'a, R: Load<T> + Store<T>, T: Copy> MutValue<'a, R, T> {
    /// Calls [`Load::load`] once to load an I/O register from memory.
    #[inline(always)]
    pub fn load_mut(r: &'a mut R) -> Self {
        MutValue {
            value: Value::load(&*r),
            reg: r,
        }
    }

    /// Calls [`Store::store`] to store an I/O register in memory. Returns a
    /// read-only copy of the stored [`Value`].
    #[inline(always)]
    pub fn store(self) -> Value<R, T> {
        unsafe { self.reg.store(self.value.bits) };
        self.value
    }
}

impl<R, T> Deref for MutValue<'_, R, T> {
    type Target = Value<R, T>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<R, T> DerefMut for MutValue<'_, R, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
