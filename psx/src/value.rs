use core::default::Default;
use core::marker::PhantomData;

/// An interface for reading a generic register from memory or a coprocessor.
pub trait Read<T: Copy>: Sized {
    /// Reads the register.
    unsafe fn read(&self) -> T;
}

/// An interface for writing a generic register to memory or a coprocessor.
pub trait Write<T: Copy>: Sized {
    /// Writes a value to the register.
    unsafe fn write(&mut self, value: T);

    /// Writes a slice of values to the register.
    unsafe fn write_slice(&mut self, values: &[T]) {
        for &value in values {
            self.write(value)
        }
    }
}

/// An interface for getting a [`Value`] from a register implementing [`Read`].
pub trait Load<T: Copy>
where Self: Read<T> {
    /// Calls [`Read::read`] once to get a [`Value`], borrowing the register
    /// to ensure no mutable references to it exist.
    #[inline(always)]
    fn load(&self) -> Value<T, Self> {
        Value {
            bits: unsafe { self.read() },
            reg: PhantomData::<&Self>,
        }
    }
}

/// An interface for getting a [`MutValue`] from a register implementing
/// [`Read`] and [`Write`].
pub trait LoadMut<T: Copy + Default>
where Self: Read<T> + Write<T> {
    /// Calls [`Read::read`] once to get a [`MutValue`], mutably borrowing the
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

    // TODO: Mark this unsafe
    /// Creates an uninitialized [`MutValue`]. Although no load occurs, the
    /// register is still mutably borrowed to ensure exclusive access for any
    /// future stores.
    #[inline(always)]
    fn skip_load(&mut self) -> MutValue<T, Self> {
        MutValue {
            value: Value {
                bits: Default::default(),
                reg: PhantomData::<&Self>,
            },
            reg: self,
        }
    }
}

/// A read-only copy of a value previously read from a generic register.
pub struct Value<'r, T: Copy, R: Load<T>> {
    /// The value's raw bits.
    pub bits: T,
    reg: PhantomData<&'r R>,
}

/// A mutable copy of a value previously read from a generic register. The value
/// must be written back to the register.
#[must_use = "`MutValue` must be written back to its register"]
pub struct MutValue<'r, T: Copy + Default, R: LoadMut<T>> {
    /// The current value.
    pub value: Value<'r, T, R>,
    reg: &'r mut R,
}

impl<'r, T: Copy + Default, R: LoadMut<T>> MutValue<'r, T, R> {
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

    /// Calls [`Write::write`] to write the current [`Self::value`] to the
    /// register. Returns a mutable reference to the register.
    #[inline(always)]
    pub fn take(self) -> &'r mut R {
        unsafe { self.reg.write(self.value.bits) }
        self.reg
    }
}

// Types that can produce a `MutValue` should also be able to produce a `Value`.
impl<T: Copy + Default, R: LoadMut<T>> Load<T> for R {}

impl<R: Load<u32>> Value<'_, u32, R> {
    #[inline(always)]
    /// Checks if any of the given `flags` are set.
    pub fn any(&self, flags: u32) -> bool {
        self.bits & flags != 0
    }

    /// Checks if the given `flags` are all set.
    #[inline(always)]
    pub fn contains(&self, flags: u32) -> bool {
        self.bits & flags == flags
    }

    /// Checks if the given `flags` are all cleared.
    #[inline(always)]
    pub fn cleared(&self, flags: u32) -> bool {
        self.bits & flags == 0
    }
}

impl<R: LoadMut<u32>> MutValue<'_, u32, R> {
    /// Sets all bits.
    #[inline(always)]
    pub fn set_all(mut self) -> Self {
        self.value.bits |= !0;
        self
    }

    /// Clears all bits.
    #[inline(always)]
    pub fn clear_all(mut self) -> Self {
        self.value.bits &= 0;
        self
    }
    /// Sets the given `flags`.
    #[inline(always)]
    pub fn set(mut self, flags: u32) -> Self {
        self.value.bits |= flags;
        self
    }

    /// Clears the given `flags`.
    #[inline(always)]
    pub fn clear(mut self, flags: u32) -> Self {
        self.value.bits &= !flags;
        self
    }
}
