use core::marker::PhantomData;

/// An interface for reading a generic register from memory or a coprocessor.
pub trait Read<T>: Sized {
    /// Reads from the register.
    unsafe fn read(&self) -> T;
}

/// An interface for writing a generic register to memory or a coprocessor.
pub trait Write<T>: Sized {
    /// Writes to the register.
    unsafe fn write(&mut self, value: T);
}

/// A read-only copy of a value previously read from a generic register.
pub struct Value<'r, T, R: Read<T>> {
    pub(crate) bits: T,
    reg: PhantomData<&'r R>,
}

/// A mutable copy of a value previously read from a generic register. The value
/// must be written back to the register.
#[must_use]
pub struct MutValue<'r, T, R: Read<T> + Write<T>> {
    /// The current value.
    pub value: Value<'r, T, R>,
    reg: &'r mut R,
}

/// An interface for getting a [`Value`] from an implementor for [`Read`].
pub trait Load<T: Copy>
where Self: Read<T> {
    /// Calls [`Read::read`] **once** to get a [`Value`], borrowing the register
    /// to ensure no mutable references to it exist.
    #[inline(always)]
    fn load(&self) -> Value<T, Self> {
        Value {
            bits: unsafe { self.read() },
            reg: PhantomData::<&Self>,
        }
    }
}

/// An interface for getting a [`MutValue`] from an implementor for [`Read`] and
/// [`Write`].
pub trait LoadMut<T: Copy>
where Self: Read<T> + Write<T> {
    /// Calls [`Read::read`] **once** to get a [`Value`], mutably borrowing the
    /// register to ensure exclusive access to it.
    #[inline(always)]
    fn load_mut(&mut self) -> MutValue<T, Self> {
        MutValue {
            value: Value {
                bits: unsafe { self.read() },
                reg: PhantomData::<&Self>,
            },
            reg: self,
        }
    }
}

impl<T: Copy, R: LoadMut<T>> Load<T> for R {}

impl<'r, T: Copy, R: Read<T> + Write<T>> MutValue<'r, T, R> {
    /// Calls [`Write::write`] to write the current [`Self::value`] to the
    /// register. Returns a [`Value`] with a copy of the written value.
    #[inline(always)]
    pub fn store(self) -> Value<'r, T, R> {
        unsafe { self.reg.write(self.value.bits) }
        Value {
            bits: self.value.bits,
            reg: PhantomData::<&'r R>,
        }
    }
}
