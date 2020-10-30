use alloc::vec::Vec;
use core::intrinsics::volatile_load;

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

// Returns vec![u32::from(&a[0]), u32::from(&b[0]), u32::from(&a[1]), ...].
pub fn intercalate<'a, 'b, T, U, const M: usize, const N: usize>(a: &'a [T; N], b: &'b [U; M]) -> Vec<u32>
    where u32: From<&'a T>, u32: From<&'b U> {
    a.iter().zip(b.iter()).flat_map(|(ai, bi)| vec![u32::from(ai), u32::from(bi)]).collect()
}

// Prepends a in its u32 form to b in its u32 form.
pub fn prepend<'a, 'b, T, U, const N: usize>(a: &'a T, b: &'b [U; N]) -> Vec<u32>
    where u32: From<&'a T>, u32: From<&'b U> {
    let mut v = Vec::with_capacity(b.len() + 1);
    v.push(u32::from(a));
    for bi in b {
        v.push(u32::from(bi));
    }
    v
}
