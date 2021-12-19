use core::mem::MaybeUninit;

/// This trait converts byte slices to null-terminated C-style strings without
/// heap allocations. It's essentially a workaround for not being able to use
/// Rust's `CStr` with no_std and intended only for internal use. The trait is
/// public, but undocumented to allow using it from other crates in `printf!`.
pub trait AsCStr: AsRef<[u8]> {
    fn as_cstr<F: FnOnce(&[u8]) -> R, R>(&self, f: F) -> R;
}

impl<T: AsRef<[u8]>> AsCStr for T {
    /// Runs a function `f` with `Self` as a null-terminated C-style string.
    /// This panics if the string is not null-terminated and requires more than
    /// 256 bytes of stack space.
    fn as_cstr<F: FnOnce(&[u8]) -> R, R>(&self, f: F) -> R {
        let slice = self.as_ref();
        // If the string is empty call `f` with just a null-terminator.
        if slice.is_empty() {
            return f(&[0])
        };
        // If the string is already null-terminated just call `f` on the original
        // string.
        let last = slice[slice.len() - 1];
        if last == 0 {
            f(slice)
        } else {
            // Panic if we need more than 256 bytes for the CStr. This is an arbitrary and
            // reasonably large limit. Since the executables are statically linked, the
            // compiler should avoid allocating the maximum stack space most of the time.
            // When printing strings with unknown length at compile-time such as
            // strings from the BIOS or user input, the compiler will allocate
            // the maximum space on the stack which is why this limit is not as high as it
            // could be.
            const MAX_STACK_ARRAY: usize = 256;
            if slice.len() >= MAX_STACK_ARRAY {
                panic!("Attempted to allocate more than 256 bytes on the stack for a `CStr`\n");
            }
            // Create an uninitialized array of up to 256 bytes on the stack
            let mut uninitialized = MaybeUninit::uninit_array::<MAX_STACK_ARRAY>();
            // Initialize the CStr with the input string
            let initialized_portion = &mut uninitialized[0..slice.len() + 1];
            MaybeUninit::write_slice(&mut initialized_portion[0..slice.len()], slice);
            // Add a null-terminator to the CStr
            initialized_portion[slice.len()].write(0);
            // SAFETY: The initialized portion of the CStr on the stack was explicitly
            // initialized and null-terminated
            let cstr = unsafe { MaybeUninit::slice_assume_init_ref(initialized_portion) };
            f(cstr)
        }
    }
}
