use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Converts byte slices to null-terminated C-style strings without heap
/// allocations. While the trait is public, it's undocumented since it's
/// only intended to be used in `printf!` and internally.
pub trait AsCStr: AsRef<[u8]> {
    fn as_cstr<F: FnOnce(&CStr) -> R, R>(&self, f: F) -> R;
}

impl<T: AsRef<[u8]>> AsCStr for T {
    /// Calls a function `f` with `Self` as a null-terminated C-style string.
    /// This panics if the string is not null-terminated and requires more than
    /// 128 bytes of stack space.
    fn as_cstr<F: FnOnce(&CStr) -> R, R>(&self, f: F) -> R {
        let slice = self.as_ref();
        match CStr::from_bytes_with_nul(slice) {
            Ok(cstr) => f(cstr),
            Err(_) => {
                // Panic if we need more than 128 bytes for the CStr. This is an arbitrary
                // limit. Since the executables are statically linked, the
                // compiler should avoid allocating the maximum stack space a
                // lot of the time. When printing strings with unknown length at
                // compile-time such as strings from the BIOS or user input, the
                // compiler will allocate the maximum space on the stack which is why
                // this limit is not as high as it could be.
                const MAX_LEN: usize = 128;
                if slice.len() >= MAX_LEN {
                    panic!("Attempted to allocate more than 128 bytes on the stack for a `CStr`\n");
                }
                // Create an uninitialized array of up to 128 bytes on the stack
                let mut uninitialized = MaybeUninit::uninit_array::<MAX_LEN>();
                // Initialize the CStr with the input string
                let initialized_part = &mut uninitialized[0..slice.len() + 1];
                MaybeUninit::write_slice(&mut initialized_part[0..slice.len()], slice);
                // Add a null-terminator to the CStr
                initialized_part[slice.len()].write(0);
                // SAFETY: The initialized portion of the CStr on the stack was explicitly
                // initialized and null-terminated
                let terminated_slice =
                    unsafe { MaybeUninit::slice_assume_init_ref(initialized_part) };
                // SAFETY: We null-terminated the initialized part of slice
                let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(terminated_slice) };
                f(cstr)
            },
        }
    }
}
