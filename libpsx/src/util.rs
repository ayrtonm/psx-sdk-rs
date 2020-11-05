use core::intrinsics::{log2f32, truncf32, volatile_load};
use crate::constrain;

#[macro_export]
macro_rules! define {
    ($name:ident := $num:expr) => {
        let mut $name: [u32; $num];
    };
    ($name1:ident := $num1:expr, $($name2:ident := $num2:expr),*) => {
        define!($name1 := $num1);
        define!($($name2 := $num2),*);
    };
}

#[macro_export]
macro_rules! ret {
    ($name:ident = $val:expr) => {
        {
            $name = $val;
            &mut $name[..]
        }
    };
}

pub trait Primitives {
    fn trunc(self) -> f32;
    fn fract(self) -> f32;
    fn log2(self) -> f32;
}
impl Primitives for f32 {
    fn trunc(self) -> f32 {
        unsafe { truncf32(self) }
    }
    fn fract(self) -> f32 {
        self - self.trunc()
    }
    fn log2(self) -> f32 {
        return unsafe { log2f32(self) };
    }
}

pub fn delay(n: u32) {
    for _ in 0..n {
        unsafe {
            volatile_load(0 as *mut u32);
        }
    }
}

pub fn prepend<T: Copy + Default, const N: usize, const S: usize>(a: T, b: &[T; N]) -> [T; S] {
    constrain!(N + 1 = S);
    concat(&[a], b)
}
pub fn append<T: Copy + Default, const N: usize, const S: usize>(a: T, b: &[T; N]) -> [T; S] {
    constrain!(N + 1 = S);
    concat(b, &[a])
}
pub fn concat<T: Copy + Default, const N: usize, const M: usize, const S: usize>(a: &[T; N], b: &[T; M]) -> [T; S] {
    constrain!(N + M = S);
    let mut ar: [T; S] = [Default::default(); S];
    ar[..N].copy_from_slice(a);
    ar[N..].copy_from_slice(b);
    ar
}

pub fn intercalate<T: Copy + Default, const N: usize, const M: usize>(a: &[T; N], b: &[T; N]) -> [T; M] {
    constrain!(N + N = M);
    let mut ar: [T; M] = [Default::default(); M];
    for i in 0..N {
        ar[i * 2] = a[i];
        ar[(i * 2) + 1] = b[i];
    }
    ar
}
